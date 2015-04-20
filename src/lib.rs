// This file is a part of Chekhov, an actor/model concurrency framework for Rust.
//
// Chekhov is free software; you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License or (at your option) any later version.
//
// Chekhov is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Chekhov. If not see
// <https://www.gnu.org/licenses/>.

#![macro_use]

//! ```no_run
//! #[macro_use]
//! extern crate chekhov;
//! 
//! use std::fmt::Display;
//! use std::io;
//!
//! use chekhov::{Actor, ActorResult};
//!
//! fn read_input(next: &Actor<String>) -> ActorResult {
//!     let mut buffer = String::new();
//!     try!(io::stdin().read_line(&mut buffer));
//!     next.cue(buffer)
//! }
//! 
//! fn main() {
//!     let printer = actor!(|msg: String| -> ActorResult { println!("{}", msg); Ok(()) });
//!     let reader  = actor_loop!(read_input, printer.stage());
//!     chekhov::from_the_top(vec![&printer, &reader]).ok();
//! }
//! ```

mod macros;

pub mod actors;
pub use actors::*;

