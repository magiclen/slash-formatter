/*!
# Slash Formatter

This crate provides functions to deal with slashes and backslashes in strings.

## Examples

To see examples, check out the documentation for each function.
*/

#![no_std]

#[macro_use]
extern crate alloc;

pub extern crate concat_with;

mod backslash;
mod file_separator;
mod slash;

pub use backslash::*;
pub use file_separator::*;
pub use slash::*;
