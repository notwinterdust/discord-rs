# UI Components, Buttons & Pagination

Message components enable rich, interactive interfaces directly inside Discord chats using **Buttons**, **Select Menus**, **Text Inputs**, **Action Rows**, and multi-page **Paginators**.

`discord-rust` validates component constraints at compile-time and runtime.

---

## Action Rows

Every interactive component must reside inside an [`ActionRow`](#). A message can contain up to **5 Action Rows**, and each row can hold up to 5 buttons or 1 select menu.

```rust
use discord_rust::app::ui::{ActionRow, Button};

// Build a row with Success and Danger buttons
let mut row = ActionRow::new();
row.add_button(Button::success("confirm_id", "Accept")).unwrap();
row.add_button(Button::danger("cancel_id", "Cancel")).unwrap();
row.add_button(Button::link("https://github.com/notwinterdust/discord-rust", "View Source")).unwrap();
```

---

## Button Styles

| ButtonStyle | Description | Color in Discord |
|---|---|---|
| `Button::primary(id, label)` | Blurple action button | Blurple |
| `Button::secondary(id, label)` | Neutral grey button | Grey |
| `Button::success(id, label)` | Positive / Confirm action | Green |
| `Button::danger(id, label)` | Destructive / Cancel action | Red |
| `Button::link(url, label)` | External website hyperlink | Grey with arrow |

---

## Select Menus (Dropdowns)

Create searchable single- or multi-select dropdown menus:

```rust
use discord_rust::app::ui::{StringSelect, SelectOption};

let mut select = StringSelect::new("role_select_id")
    .placeholder("Choose your favorite programming language")
    .min_values(1)
    .max_values(3);

select.add_option(SelectOption::new("Rust", "lang_rust")).unwrap();
select.add_option(SelectOption::new("Python", "lang_python")).unwrap();
select.add_option(SelectOption::new("TypeScript", "lang_ts")).unwrap();
```

---

## Paginator

`Paginator<T>` manages multi-page content and generates a navigation `ActionRow` automatically. It handles ⏮ ◀ page indicator ▶ ⏭ buttons with correct disabled states.

```rust
use discord_rust::app::ui::Paginator;

let pages = vec![
    "**Page 1** — Introduction to Rust",
    "**Page 2** — Ownership & Borrowing",
    "**Page 3** — Traits & Generics",
];

let mut paginator = Paginator::new(pages)
    .prefix("help_pages")       // used as custom_id prefix
    .show_first_last(true);     // include ⏮ ⏭ buttons

// Render the navigation buttons for the current page
let nav_row = paginator.action_row();
// Send nav_row alongside your embed/text as a component

// In your interaction handler, forward the custom_id:
let changed = paginator.handle_interaction("help_pages:next");
// changed == true → page advanced, re-render

println!("Now on page {}/{}", paginator.current_index() + 1, paginator.total_pages());
println!("Content: {:?}", paginator.current_page());
```

### Navigation API

| Method | Description |
|---|---|
| `paginator.next()` | Advance one page, returns `true` if changed |
| `paginator.prev()` | Go back one page, returns `true` if changed |
| `paginator.first()` | Jump to page 0, returns `true` if changed |
| `paginator.last()` | Jump to last page, returns `true` if changed |
| `paginator.go_to(n)` | Jump to index `n`, returns `true` if changed |
| `paginator.handle_interaction(id)` | Dispatch a button `custom_id` automatically |
| `paginator.action_row()` | Build the navigation `ActionRow` for current state |

### Button custom_ids

The paginator generates `custom_id` values as `"{prefix}:first"`, `"{prefix}:prev"`, `"{prefix}:next"`, `"{prefix}:last"`, and `"{prefix}:indicator"` (the page counter, always disabled).

Pass incoming interaction `custom_id` strings directly to `handle_interaction` to advance the page and re-render.

---

## Modals & Text Inputs

Modals are pop-up dialog boxes with text input fields:

```rust
use discord_rust::app::ui::TextInput;

// Short single-line input
let title_input = TextInput::short("feedback_title", "Title")
    .min_length(5)
    .max_length(100)
    .required(true);

// Multi-line paragraph input
let body_input = TextInput::paragraph("feedback_body", "Your Feedback")
    .min_length(10)
    .max_length(1_000);
```

> [!NOTE]
> Modals are triggered as responses to incoming button clicks or slash command interactions via `InteractionResponse::modal(...)`.
