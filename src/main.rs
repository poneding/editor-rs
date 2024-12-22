#![warn(
    clippy::all,
    clippy::padantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
use editor::Editor;

mod editor;
mod terminal;

fn main() {
    Editor::default().run();
}
