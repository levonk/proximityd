pub mod completion;
pub mod confirm;
pub mod install;
pub mod man;
pub mod pager;
pub mod tui;

pub use completion::{generate_completion, CompletionArgs};
pub use confirm::confirm;
pub use install::{run_install, run_uninstall};
pub use man::{display_man_page, generate_man_pages, install_man_pages};
pub use pager::{detect_pager, page_output, should_page_output};
pub use tui::{is_tui_supported, run_tui};
