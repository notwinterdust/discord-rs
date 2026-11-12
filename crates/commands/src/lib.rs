//! Prefix-command registration primitives.

#![forbid(unsafe_code)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// A parsed command invocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    /// The command name.
    pub name: String,
    /// The remaining arguments.
    pub arguments: Vec<String>,
}

impl Context {
    /// Attempts to parse argument at `index` into type `T`.
    ///
    /// # Errors
    ///
    /// Returns an error message if the argument is missing or fails to parse.
    pub fn arg<T: std::str::FromStr>(&self, index: usize) -> Result<T, String>
    where
        T::Err: std::fmt::Display,
    {
        self.arguments
            .get(index)
            .ok_or_else(|| format!("missing argument at index {index}"))?
            .parse::<T>()
            .map_err(|e| format!("invalid argument at index {index}: {e}"))
    }

    /// Extracts a numeric snowflake ID from a user, channel, or role mention.
    /// Supports `<@123>`, `<@!123>`, `<#123>`, `<@&123>`.
    #[must_use]
    pub fn parse_mention(mention: &str) -> Option<u64> {
        let trimmed = mention
            .strip_prefix('<')?
            .strip_suffix('>')?
            .trim_start_matches('@')
            .trim_start_matches('!')
            .trim_start_matches('#')
            .trim_start_matches('&');
        trimmed.parse::<u64>().ok()
    }
}

/// A command group organizing subcommands.
pub struct CommandGroup {
    pub name: String,
    pub description: Option<String>,
    subcommands: Vec<CommandEntry>,
}

impl CommandGroup {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            subcommands: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn add_subcommand(
        &mut self,
        name: impl Into<String>,
        aliases: impl IntoIterator<Item = impl Into<String>>,
        handler: impl Handler + 'static,
    ) {
        self.subcommands.push(CommandEntry {
            name: name.into(),
            aliases: aliases.into_iter().map(Into::into).collect(),
            handler: Box::new(handler),
        });
    }

    pub fn dispatch(&mut self, context: &Context) -> bool {
        if context.arguments.is_empty() {
            return false;
        }
        let sub_name = &context.arguments[0];
        let sub_context = Context {
            name: format!("{} {}", context.name, sub_name),
            arguments: context.arguments[1..].to_vec(),
        };
        let Some(cmd) = self
            .subcommands
            .iter_mut()
            .find(|c| c.name == *sub_name || c.aliases.iter().any(|a| a == sub_name))
        else {
            return false;
        };
        cmd.handler.call(&sub_context);
        true
    }

    #[must_use]
    pub fn subcommand_names(&self) -> Vec<&str> {
        self.subcommands.iter().map(|c| c.name.as_str()).collect()
    }
}

/// A recurring background task loop schedule description.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TaskLoop {
    interval: Duration,
}

impl TaskLoop {
    #[must_use]
    pub const fn new(interval: Duration) -> Self {
        Self { interval }
    }

    #[must_use]
    pub const fn interval(self) -> Duration {
        self.interval
    }

    /// Spawns a background thread executing `action` every `interval` until `shutdown` is true.
    pub fn start<F>(self, shutdown: Arc<AtomicBool>, mut action: F) -> JoinHandle<()>
    where
        F: FnMut() + Send + 'static,
    {
        thread::spawn(move || {
            while !shutdown.load(Ordering::Relaxed) {
                action();
                thread::sleep(self.interval);
            }
        })
    }
}

/// A command predicate.
pub type Check = Box<dyn Fn(&Context) -> bool>;

/// A simple command cooldown.
#[derive(Clone, Debug)]
pub struct Cooldown {
    limit: usize,
    uses: usize,
}

impl Cooldown {
    /// Creates a cooldown with a maximum invocation count.
    #[must_use]
    pub const fn new(limit: usize) -> Self {
        Self { limit, uses: 0 }
    }

    /// Attempts to consume one invocation.
    pub fn allow(&mut self) -> bool {
        if self.uses >= self.limit {
            return false;
        }
        self.uses += 1;
        true
    }

    /// Resets the cooldown.
    pub fn reset(&mut self) {
        self.uses = 0;
    }
}

/// A command handler.
pub trait Handler {
    /// Handles an invocation.
    fn call(&mut self, context: &Context);
}

impl<F> Handler for F
where
    F: FnMut(&Context),
{
    fn call(&mut self, context: &Context) {
        self(context);
    }
}

struct CommandEntry {
    name: String,
    aliases: Vec<String>,
    handler: Box<dyn Handler>,
}

/// An extension module (Cog equivalent) for modular bot organization.
pub trait Extension {
    /// The unique name of the extension.
    fn name(&self) -> &str;
    /// Mounts commands and handlers onto the registry.
    fn load(&self, registry: &mut CommandRegistry) -> Result<(), &'static str>;
    /// Unmounts commands from the registry.
    fn unload(&self, registry: &mut CommandRegistry) {
        let _ = registry.remove(self.name());
    }
}

/// A command registry with prefix parsing and aliases.
#[derive(Default)]
pub struct CommandRegistry {
    prefix: String,
    commands: Vec<CommandEntry>,
}

impl CommandRegistry {
    /// Creates a registry with a prefix.
    #[must_use]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            commands: Vec::new(),
        }
    }

    /// Registers a command.
    pub fn add(
        &mut self,
        name: impl Into<String>,
        aliases: impl IntoIterator<Item = impl Into<String>>,
        handler: impl Handler + 'static,
    ) -> Result<(), &'static str> {
        let name = name.into();
        if self.commands.iter().any(|command| {
            command.name == name || command.aliases.iter().any(|alias| alias == &name)
        }) {
            return Err("command name already exists");
        }
        self.commands.push(CommandEntry {
            name,
            aliases: aliases.into_iter().map(Into::into).collect(),
            handler: Box::new(handler),
        });
        Ok(())
    }

    /// Mounts a subcommand group.
    pub fn add_group(&mut self, mut group: CommandGroup) -> Result<(), &'static str> {
        let name = group.name.clone();
        self.add(name, Vec::<String>::new(), move |ctx: &Context| {
            group.dispatch(ctx);
        })
    }

    /// Loads an extension module.
    pub fn load_extension(&mut self, extension: &dyn Extension) -> Result<(), &'static str> {
        extension.load(self)
    }

    /// Removes a command by name.
    pub fn remove(&mut self, name: &str) -> bool {
        let initial = self.commands.len();
        self.commands.retain(|c| c.name != name);
        self.commands.len() < initial
    }

    /// Generates a human-readable help listing.
    #[must_use]
    pub fn generate_help(&self) -> String {
        let mut help = String::from("Commands:\n");
        for cmd in &self.commands {
            help.push_str(&format!("  {}{}", self.prefix, cmd.name));
            if !cmd.aliases.is_empty() {
                help.push_str(&format!(" (aliases: {})", cmd.aliases.join(", ")));
            }
            help.push('\n');
        }
        help
    }

    /// Parses and dispatches a message.
    pub fn dispatch(&mut self, message: &str) -> bool {
        let Some(rest) = message.strip_prefix(&self.prefix) else {
            return false;
        };
        let mut words = Self::tokenize(rest);
        let Some(name) = words.next() else {
            return false;
        };
        let context = Context {
            name: name.to_owned(),
            arguments: words.collect(),
        };
        let Some(command) = self.commands.iter_mut().find(|command| {
            command.name == name || command.aliases.iter().any(|alias| alias == &name)
        }) else {
            return false;
        };
        command.handler.call(&context);
        true
    }

    /// Parses and dispatches a message only when a predicate accepts it.
    pub fn dispatch_checked(&mut self, message: &str, check: impl Fn(&Context) -> bool) -> bool {
        let Some(rest) = message.strip_prefix(&self.prefix) else {
            return false;
        };
        let mut words = Self::tokenize(rest);
        let Some(name) = words.next() else {
            return false;
        };
        let context = Context {
            name: name.clone(),
            arguments: words.collect(),
        };
        if !check(&context) {
            return false;
        }
        let Some(command) = self.commands.iter_mut().find(|command| {
            command.name == name || command.aliases.iter().any(|alias| alias == &name)
        }) else {
            return false;
        };
        command.handler.call(&context);
        true
    }

    fn tokenize(input: &str) -> impl Iterator<Item = String> + '_ {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut quoted = false;
        for character in input.chars() {
            match character {
                '"' => quoted = !quoted,
                character if character.is_whitespace() && !quoted => {
                    if !current.is_empty() {
                        values.push(std::mem::take(&mut current));
                    }
                }
                character => current.push(character),
            }
        }
        if !current.is_empty() {
            values.push(current);
        }
        values.into_iter()
    }

    /// Returns the configured prefix.
    #[must_use]
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_argument_parsing_and_mentions() {
        let ctx = Context {
            name: "test".into(),
            arguments: vec!["42".into(), "<@!123456789>".into(), "hello".into()],
        };
        assert_eq!(ctx.arg::<i32>(0), Ok(42));
        assert!(ctx.arg::<i32>(1).is_err());
        assert_eq!(ctx.arg::<String>(2), Ok("hello".into()));
        assert!(ctx.arg::<String>(3).is_err());

        assert_eq!(Context::parse_mention("<@123>"), Some(123));
        assert_eq!(Context::parse_mention("<@!456>"), Some(456));
        assert_eq!(Context::parse_mention("<#789>"), Some(789));
        assert_eq!(Context::parse_mention("<@&1011>"), Some(1011));
        assert_eq!(Context::parse_mention("not_a_mention"), None);
    }

    #[test]
    fn command_groups_dispatch_subcommands() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let counter = Arc::new(AtomicU32::new(0));
        let c = Arc::clone(&counter);

        let mut group = CommandGroup::new("git").with_description("Git operations");
        group.add_subcommand("commit", vec!["ci"], move |_: &Context| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(group.subcommand_names(), vec!["commit"]);

        let mut registry = CommandRegistry::new("!");
        assert!(registry.add_group(group).is_ok());

        assert!(registry.dispatch("!git commit"));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert!(registry.dispatch("!git ci"));
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn extension_modules_load_and_unload() {
        struct MathCog;
        impl Extension for MathCog {
            fn name(&self) -> &str {
                "math"
            }
            fn load(&self, registry: &mut CommandRegistry) -> Result<(), &'static str> {
                registry.add("add", Vec::<String>::new(), |_: &Context| {})
            }
        }

        let mut registry = CommandRegistry::new("!");
        let cog = MathCog;
        assert!(registry.load_extension(&cog).is_ok());
        assert!(registry.remove("add"));
        assert!(!registry.remove("add"));
    }

    #[test]
    fn help_generation_lists_commands() {
        let mut registry = CommandRegistry::new("!");
        registry.add("ping", vec!["p"], |_: &Context| {}).unwrap();
        let help = registry.generate_help();
        assert!(help.contains("!ping (aliases: p)"));
    }

    #[test]
    fn task_loop_executes_until_shutdown() {
        let task = TaskLoop::new(Duration::from_millis(5));
        let shutdown = Arc::new(AtomicBool::new(false));
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = Arc::clone(&counter);

        let handle = task.start(Arc::clone(&shutdown), move || {
            c.fetch_add(1, Ordering::SeqCst);
        });

        thread::sleep(Duration::from_millis(25));
        shutdown.store(true, Ordering::Relaxed);
        handle.join().unwrap();
        assert!(counter.load(Ordering::SeqCst) >= 2);
    }
}
