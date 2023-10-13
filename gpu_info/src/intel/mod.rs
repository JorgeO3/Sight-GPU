#![allow(unused)]
mod bindings;

use bindings::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntelError {}
