###Chekhov makes actor/message concurrency in Rust easy.

Chekhov is a concurrency framework for Rust focused on simplicity and ease of
use. Using Chekhov, you can briskly create __actors__ which communicate by
message-passing and require little more syntax than writing normal, sequential
functions. Chekhov uses a set of macros to create objects and handle thread
safety, so you don't have to.

I wrote Chekhov this afternoon/evening; right now it is highly unstable, not yet
feature complete, not necessarily optimized, and probably brittle in several
ways. __Pull requests welcome,__ especially those that address these things or
make Chekhov's naming scheme into more goofy acting jargon.

Documentation forthcoming soon. For now here's a simple multi-threaded echo
program written using Chekhov:
```
#![feature(core)]

#[macro_use(actor, feeder)]
extern crate chekhov;

use chekhov::*;

feeder!{ Input(next: Actor<String>) -> {
    let mut echo = String::new();
    if std::io::stdin().read_line(&mut echo).is_ok() {
        next.cue(echo);
    }
}}

actor!{ Output() -> |String: echo| {
    print!("{}", echo);
}}

fn main() {
    let output = Output::new();
    let input = Input::new(output.stage());
    from_the_top(vec![input, output]);
}
```
