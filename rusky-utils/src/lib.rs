pub mod config;
use std::{error::Error as StdError, result::Result as StdResult};
pub type Error = Box<dyn StdError + Sync + Send>;
pub type Result<T> = StdResult<T, Error>;
pub use serde;
pub use tracing;
