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
#![feature(core)]

//! Chekhov is an actor/model concurrency framework for Rust. Because of its heavy reliance on
//! macros, formatting rust docs is tricky, so for now I've just dumped a bunch of markdown in the
//! main lib file. Sorry!
//!
//! Chekhov is built on top of the standard Rust concurrency primitives, but it uses them to make
//! writing concurrent code much faster and simpler.
//!
//! # Chekhov in the small (declaring actors)
//!
//! Chekhov Actors each run on their own thread and have ownership over their environment. They
//! communicate with one another by sending messages (which can be any object that implements Send,
//! of course). An Actor is declared using the actor! or actor_mut! macros. These differ only in
//! that actor_mut! Actors can mutate the persistent bindings in their environment; the values
//! these two macros return have the same type and present the same interface.
//!
//! There are two basic kinds of Actors (setting aside mutability). The first and more common kind
//! have a mailbox in which they can receive messages. Actors with a mailbox block until they
//! receive a message; when they have a message, they perform their behavior and then block again
//! until they receive another message. The second kind do not have a mailbox, and simply perform
//! their behavior in a contious loop until they stop themselves somehow.
//!
//! The actor! macros use a special syntax (which is the same in both macros). Let's look at a
//! simple Actor which prints whatever message it receives with a prefix determined when it is
//! instantiated.
//!
//! ```
//! # #![feature(core)]
//! # #[macro_use(actor)]
//! # extern crate chekhov;
//! actor!{ PrefixedPrinter(prefix: String) :: msg: String => {
//!     println!("{}", prefix.clone() + &msg);
//! }}
//! # fn main() {  }
//! ```
//!
//! Here, you can see the structure of an Actor declaration:
//!
//! 1. It is enclosed in the actor!{} or actor_mut!{} macro. Note the two closing brackets after
//!    the end of the Actor's body.
//! 2. The first part of the Actor declaration is the Actor's name (in this case
//!    `PrefixedPrinter`). 
//! 3. The Actor's name takes parens, just like a function or a tuple struct. If the Actor takes
//!    any arguments when it is constructed and initialized, these are listed here. Even if the
//!    Actor does not take arguments, the parens are required.
//! 4. If the Actor has a mail box, you have to declare it the binding and type of the mailbox,
//!    preceded by the '::' symbol. (If the actor is of the type that do not receive messages, do
//!    not include any of this).
//! 5. This is followed by the fat arrow `=>` (Not a normal function arrow!) and a block which
//!    defines the body of the actor; this block is what will be called every time the actor
//!    receives a message, or on a continuous loop if the actor does not take messages.
//!
//! This is an Actor which does not have a mailbox. It prints a line enumeration and then reads
//! from stdin, passing what it reads along to another Actor.
//!
//! ```
//! #![feature(core)]
//! # #[macro_use(actor_mut)]
//! # extern crate chekhov;
//! # use std::io;
//! # use std::io::Write;
//! # use chekhov::*;
//! actor_mut!{ EnumeratedReader(next: Actor<String>, x: u32) => {
//!     print!("{}. ", x);
//!     io::stdout().flush().ok();
//!     x += 1;
//!     let mut buffer = String::new();
//!     if io::stdin().read_line(&mut buffer).is_ok() {
//!         try!(next.cue(buffer));
//!     }
//! }}
//! # fn main() {  }
//! ```
//!
//! # Chekhov in the large (composing actors together)
//!
//! EnumeratedReader took an `Actor<String>` as an argument, and called a method `cue()`. This is
//! how actors can be composed: by communicating.
//!
//! All Actors macromagically have a new method, which takes the arguments listed in parens in the
//! header of that Actor's declaration. This returns a special object which represents an Actor
//! that has not yet begun processing messages. Note that actors do not do anything immediately
//! upon instantiation; the object returned by the constructor has a method `action()` which must
//! be called before the actors will begin performing computation. This method consumes the object
//! which it is called on.
//!
//! Actors with mailboxes have an additional method `stage()`, which creates a normal Actor object
//! which allows you to communicate with that actor. Actor objects can be cloned, moved around, and
//! sent between threads. If you want your actors to pass messages to other Actors, they should
//! take an `Actor<T>` as one of their constructor arguments.
//!
//! Messages are passed to Actors by giving their object a 'cue' so they can remember their lines.
//! This is the `cue()` method, which takes an object of whatever type that Actor receives. Passing
//! in messages with the `cue()` method is the only way to communicate with an Actor.
//!
//! To assist in setting up Actors, Chekhov comes with a function called `from_the_top()`, which
//! takes a Vec of Actor objects on which `action()` has not yet been called. This will call
//! `action()` on every Actor in the Vec and then park the thread it was called on (e.g. the main
//! thread).
//!
//! # Chekhov in the tedious details (how to import the crate)
//! 
//! Because Chekhov uses macros, when importing the crate you must tag the import with
//! `#[macro_use]`. These macros also cause import conflicts with `use chekhov;` (unfortunately).
//! If you need to use any of the types or functions in Chekhov in a module with an Actor
//! declaration, you should import with `use chekhov::*;`.
//!
//! Chekhov also relies on the unstable feature `core` (specifically FnBox). For this reason, it is
//! only usable on the nightly track, and it must be compiled with the `#![feature(core)]` flag.
//!
//! Here is a complete echo program, using the macros described earlier.
//!
//! ```no_run
//! #![feature(core)]
//!
//! #[macro_use]
//! extern crate chekhov;
//!
//! use std::io;
//! use std::io::Write;
//! use chekhov::*;
//!
//! actor!{ PrefixedPrinter(prefix: String) :: msg: String => {
//!     println!("{}", prefix.clone() + &msg);
//! }}
//! 
//! actor_mut!{ EnumeratedReader(next: Actor<String>, x: u32) => {
//!     print!("{}. ", x);
//!     io::stdout().flush().ok();
//!     x += 1;
//!     let mut buffer = String::new();
//!     if io::stdin().read_line(&mut buffer).is_ok() {
//!         try!(next.cue(buffer));
//!     }
//! }}
//!
//! fn main() {
//!     let printer = PrefixedPrinter::new(">>> ".to_string());
//!     let reader = EnumeratedReader::new(printer.stage(), 1);
//!     from_the_top(vec![printer, reader]);
//! }
//! ```

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
