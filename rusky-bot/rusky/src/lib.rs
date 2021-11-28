#![feature(decl_macro)]
pub use tokio;
pub use serenity;
pub mod commands;
pub mod logging;
pub use rusky_core::{macros::*, Result};
