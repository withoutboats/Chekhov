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

use chekhov::{Actor, ActorResult}

fn read_input(next: &Actor<String>) -> ActorResult {
    let mut buffer = String::new();
    try!(io::stdin().read_line(&mut buffer));
    next.cue(buffer)
}

fn main() {
    let printer = actor!(|msg: String| -> ActorResult { println("{}", msg); Ok(()) });
    let reader  = actor_loop!(read_input, printer.stage());
    chekhov::from_the_top(vec![&printer, &reader]).ok();
}
```


### Rust version compatibility

Chekhov is compatible with Rust 1.0.0-beta and up.

### Licensing

Chekhov is licensed under the GNU General Public License, version 3.0 or
higher, at your choice, with the Classpath Library Linking Exception.
