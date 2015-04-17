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

use std::io;

fn print_prefixed<T: Display>(msg: T, prefix: &str) -> Result<(), ActorError> {
    println!("{}{}", prefix, msg);
} 

fn read_input(next: ::chekhov::Actor<String>) -> Result<(), ActorError> {
    while true {
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_ok() {
            try!(next.cue(buf));
        }
    }
}

fn main() {
    let printer = actor!(print_prefixed, ">>> ");
    read_input(printer.stage().unwrap()).ok();
}
```

### Rust version compatibility

Chekhov is compatible with Rust 1.0.0-beta and up.

### Licensing

Chekhov is licensed under the GNU General Public License, version 3.0 or
higher, at your choice, with the Classpath Library Linking Exception.
