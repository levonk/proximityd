pub mod completion;
pub mod install;

pub use completion::{generate_completion, CompletionArgs};
pub use install::{run_install, run_uninstall};
