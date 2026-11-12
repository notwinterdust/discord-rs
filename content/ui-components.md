# UI Components & Buttons

Message components enable rich, interactive interfaces directly inside Discord chats using **Buttons**, **Select Menus**, **Text Inputs**, and **Action Rows**.

`discord-rust` validates component constraints at compile-time and runtime.

---

## Action Rows

Every interactive component must reside inside an [`ActionRow`](#). A message can contain up to **5 Action Rows**, and each row can hold up to 5 buttons or 1 select menu.

```rust
use discord_rust::app::ui::{ActionRow, Button};
use discord_rust::enums::ButtonStyle;

// Build a row with Primary and Danger buttons
let row = ActionRow::new()
    .add_button(
        Button::new("confirm_id", ButtonStyle::Success)
            .with_label("Accept")
    )
    .add_button(
        Button::new("cancel_id", ButtonStyle::Danger)
            .with_label("Cancel")
    )
    .add_button(
        Button::link("https://github.com/notwinterdust/discord-rust")
            .with_label("View Source")
    );
```

---

## Button Styles

| ButtonStyle | Description | Color in Discord |
|---|---|---|
| `ButtonStyle::Primary` | Blurple action button | Blurple |
| `ButtonStyle::Secondary` | Neutral grey button | Grey |
| `ButtonStyle::Success` | Positive / Confirm action | Green |
| `ButtonStyle::Danger` | Destructive / Cancel action | Red |
| `ButtonStyle::Link` | External website hyperlink | Grey with arrow |

---

## Select Menus (Dropdowns)

Create searchable single- or multi-select dropdown menus:

```rust
use discord_rust::app::ui::{SelectMenu, SelectOption};

let select = SelectMenu::text("role_select_id")
    .with_placeholder("Choose your favorite programming language")
    .with_min_values(1)
    .with_max_values(3)
    .add_option(SelectOption::new("Rust", "lang_rust").with_description("Blazingly fast & safe"))
    .add_option(SelectOption::new("Python", "lang_python").with_description("Simple & clean"))
    .add_option(SelectOption::new("TypeScript", "lang_ts").with_description("Typed JavaScript"));
```

---

## Modals & Text Inputs

Modals are pop-up dialog boxes with text input fields:

```rust
use discord_rust::app::ui::TextInput;
use discord_rust::enums::TextStyle;

// Short single-line input
let title_input = TextInput::new("feedback_title", TextStyle::Short)
    .with_label("Title")
    .with_min_length(5)
    .with_max_length(100)
    .with_required(true);

// Multi-line paragraph input
let body_input = TextInput::new("feedback_body", TextStyle::Paragraph)
    .with_label("Your Feedback")
    .with_placeholder("Tell us what you think...")
    .with_min_length(10)
    .with_max_length(1000);
```

> [!NOTE]
> Modals are triggered as responses to incoming button clicks or slash command interactions via `InteractionResponse::modal(...)`.
