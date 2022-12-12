/* Web IDL data structures */
mod consts;
mod definitions;
mod members;
mod types;

pub use consts::*;
pub use definitions::*;
pub use members::*;
pub use types::*;

/* Display and parser logic */
mod display;
mod parser;

/* Tests */
#[cfg(test)]
mod tests;

/* Exposed utility functions */
mod utils;

pub use utils::parse;
pub use utils::to_string;
