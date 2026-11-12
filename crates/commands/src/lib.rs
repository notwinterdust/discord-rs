//! Prefix-command registration primitives.

#![forbid(unsafe_code)]

/// A parsed command invocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Context {
    /// The command name.
    pub name: String,
    /// The remaining arguments.
    pub arguments: Vec<String>,
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
    pub fn dispatch_checked(
        &mut self,
        message: &str,
        check: impl Fn(&Context) -> bool,
    ) -> bool {
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
