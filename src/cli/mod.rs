pub mod completion;
pub mod install;
pub mod tui;

pub use completion::{generate_completion, CompletionArgs};
pub use install::{run_install, run_uninstall};
pub use tui::{is_tui_supported, run_tui};
