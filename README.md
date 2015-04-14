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
use std::io::Write;

actor!{ PrefixedPrinter(prefix: String) :: msg: String => {
    println!("{}", prefix.clone() + &msg);
}}

actor_mut!{ EnumeratedReader(next: ::chekhov::Actor<String>, x: u32) => {
    print!("{}. ", x);
    io::stdout().flush().ok();
    x += 1;
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_ok() {
        try!(next.cue(buffer));
    }
}}

fn main() {
    let printer = PrefixedPrinter::new(">>> ".to_string());
    let reader = EnumeratedReader::new(printer, 1);
}
```

### Rust version compatibility

Chekhov is compatible with Rust 1.0.0-beta and up.

### Licensing

Chekhov is licensed under the GNU General Public License, version 3.0 or
higher, at your choice, with the Classpath Library Linking Exception.
