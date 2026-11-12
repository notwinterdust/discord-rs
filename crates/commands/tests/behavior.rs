use std::cell::Cell;
use std::rc::Rc;

use commands::{CommandRegistry, Cooldown};

#[test]
fn quoted_arguments_are_preserved() {
    let seen = Rc::new(Cell::new(false));
    let output = Rc::clone(&seen);
    let mut registry = CommandRegistry::new("!");
    registry
        .add(
            "say",
            Vec::<String>::new(),
            move |context: &commands::Context| {
                output.set(context.arguments == ["hello world"]);
            },
        )
        .unwrap();

    assert!(registry.dispatch("!say \"hello world\""));
    assert!(seen.get());
}

#[test]
fn cooldown_is_bounded_and_resettable() {
    let mut cooldown = Cooldown::new(1);
    assert!(cooldown.allow());
    assert!(!cooldown.allow());
    cooldown.reset();
    assert!(cooldown.allow());
}
