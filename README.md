###Chekhov makes actor/message concurrency in Rust easy.

Chekhov is a concurrency framework for Rust focused on simplicity and ease of
use. With Chekhov, you can create __actors__ which communicate by __message-
passing__ using very similar syntax to writing functions. Chekhov uses a set of
macros to create actors and handle threading, so you don't have to. You just
define the behavior of actors and compose them together.

Chkehov is highly unstable and incomplete, __do not try use it yet__. It will
eat your laundry and set your house on fire. __Pull requests welcome.__ There's
no documentation yet, but here's an example:

```rust
#[macro_use]
extern crate chekhov;

use std::fmt::Display;
use std::io;

use chekhov::*;

fn print_prefixed<T: Display>(msg: T, prefix: &str) -> Result<(), ActorError> {
    println!("{}{}", prefix, msg);
    Ok(())
} 

fn read_input(next: &Actor<String>) -> Result<(), ActorError> {
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buf).is_ok() {
        next.cue(buffer)
    } else { ActorError::Internal("Could not read from stdin.".to_string()) }
}

fn main() {
    let printer = actor!(print_prefixed, prefix=">>> ");
    actor_loop!(read_input, next=printer);
}
```

### Rust version compatibility

Chekhov is compatible with Rust 1.0.0-beta and up.

### Licensing

Chekhov is licensed under the GNU General Public License, version 3.0 or
higher, at your choice, with the Classpath Library Linking Exception.
