/*!
# Slash Formatter

This crate provides functions to deal with slashes and backslashes in strings.

## Examples

To see examples, check out the documentation for each function.
*/

#![no_std]

#[macro_use]
extern crate alloc;

mod backslash;
mod file_separator;
mod file_separator_build;
mod slash;

pub use backslash::*;
#[doc(hidden)]
pub use concat_with::{concat, concat_impl};
pub use file_separator::*;
pub use file_separator_build::*;
pub use slash::*;
