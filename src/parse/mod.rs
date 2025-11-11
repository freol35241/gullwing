//! Runtime string parsing using format specifications.

mod builder;
mod matcher;

pub use matcher::{ParseResult, Parser};

use crate::error::{Error, Result};
use crate::spec::FormatSpec;
use crate::types::Value;
