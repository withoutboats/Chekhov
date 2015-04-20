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

#[macro_export]
macro_rules! actor {
    ($script:expr, $($e:expr),*) => ( actor_expand!(actor $script => ($($e,)*) ()) );
}

#[macro_export]
macro_rules! actor_mut {
    ($script:expr, $($e:expr),*) => ( actor_expand!(actor_mut $script => ($($e,)*) ()) );
}

#[macro_export]
macro_rules! actor_loop {
    ($script:expr, $($e:expr),*) => ( actor_expand!(actor_loop $script => ($($e,)*) ()) );
}

#[macro_export]
macro_rules! actor_expand {
    ($kind:ident $script:expr => ($head:expr, $($rest:expr,)*) ($($bound:ident),*)) => ({
        let binding = $head;
        actor_expand!($kind $script => ($($rest,)*) ($($bound,)* binding))
    });
    (actor $script:expr => () ($($bound:ident),*)) => ({
        let (tx, rx) = ::std::sync::mpsc::channel();
        ::std::thread::spawn(move || while let Ok(Message::Cue(msg)) = rx.recv() {
            if $script(msg, $( &$bound, )*).is_err() { break; }
        });
        Actor::new(tx)
    });
    (actor_mut $script:expr => () ($($bound:ident),*)) => ({
        $( let mut $bound = $bound; )*
        let (tx, rx) = ::std::sync::mpsc::channel();
        ::std::thread::spawn(move || while let Ok(Message::Cue(msg)) = rx.recv() {
            if $script(msg, $( &mut $bound, )*).is_err() { break; }
        });
        Actor::new(tx)
    });
    (actor_loop $script:expr => () ($($bound:ident),*)) => ({
        $( let mut $bound = $bound; )*
        ::std::thread::spawn(move || loop {
            if $script($( &mut $bound, )*).is_err() { break; }
        });
    });
}
