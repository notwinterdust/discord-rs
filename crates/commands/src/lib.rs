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
        let mut words = rest.split_whitespace();
        let Some(name) = words.next() else {
            return false;
        };
        let context = Context {
            name: name.to_owned(),
            arguments: words.map(str::to_owned).collect(),
        };
        let Some(command) = self.commands.iter_mut().find(|command| {
            command.name == name || command.aliases.iter().any(|alias| alias == name)
        }) else {
            return false;
        };
        command.handler.call(&context);
        true
    }

    /// Returns the configured prefix.
    #[must_use]
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}
