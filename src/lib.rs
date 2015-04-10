// This file is a part of Chekhov, an actor/model concurrency framework for Rust.
//
// Chekhov is free software; you can redistribute it and/or modify it under the terms of the Lesser
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License or (at your option) any later version.
//
// Chekhov is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Chekhov. If
// not, see <https://www.gnu.org/licenses/>.

#![macro_use]

use std::thread;

mod macros;
mod types;

pub use types::*;

pub fn from_the_top(actors: Vec<Box<Actionable>>) {
    for actor in actors {
        actor.action();
    }
    thread::park();
}
