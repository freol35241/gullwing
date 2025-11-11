//! Runtime string formatting with format specifications.

mod engine;
mod writer;

pub use engine::Formatter;

use crate::error::{Error, Result};
use crate::spec::FormatSpec;
use crate::types::Value;
