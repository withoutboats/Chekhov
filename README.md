###Chekhov makes actor/message concurrency in Rust easy.

Chekhov is a concurrency framework for Rust focused on simplicity and ease of
use. Using Chekhov, you can briskly create __actors__ which communicate by
__message-passing__ and require little more syntax than writing normal,
sequential functions. Chekhov uses a set of macros to create objects and handle
thread safety, so you don't have to.

I wrote Chekhov this afternoon/evening; right now it is highly unstable, not yet
feature complete, not necessarily optimized, and probably brittle in several
ways. __Pull requests welcome,__ especially those that address these things or
make Chekhov's naming scheme into more goofy acting jargon.

There's basic documentation at the top of the main lib file, which you can
format into quite attractive HTML docs with `cargo doc` (I will have it uploaded
to the web soon enough). As a sample of how Chekhov works, here is the echo
program built in those docs:

```
#![feature(core)]

#[macro_use]
extern crate chekhov;

use std::io;
use std::io::Write;
use chekhov::*;

actor!{ PrefixedPrinter(prefix: String) |> msg: String => {
    println!("{}", prefix.clone() + &msg);
}}
 
actor_mut!{ EnumeratedReader(next: Actor<String>, x: u32) => {
    print!("{}. ", x);
    io::stdout().flush().ok();
    x += 1;
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_ok() {
        next.cue(buffer);
    }
}}

fn main() {
    let printer = PrefixedPrinter::new(">>> ".to_string());
    let reader = EnumeratedReader::new(printer.stage(), 1);
    from_the_top(vec![printer, reader]);
}
```

### Rust version compatibility

Sadly, Chekhov is dependent on the unstable feature FnBox, so it is currently 
only compatibile with the nightly track of Rust. Chekhov uses no other unstable
features and should be compatible with the first stable branch that includes
FnBox.

If you know a way to perform a function application on a new thread without
using FnBox or any other unstable features, please let me know. :-)

### Licensing

Chekhov is licensed under the GNU Lesser General Public License, version 3.0 or
higher, at your choice.
