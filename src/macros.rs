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
    ($script:expr, $($bind:ident=$val:expr),*) => ({
        $( let $bind = $val; )*
        let (__chek_tx, __chek_rx) = ::std::sync::mpsc::channel();
        ::std::thread::spawn(move || {
            while let Ok(__chek_msg) = __chek_rx.recv() {
                if let Err(_) = $script(__chek_msg, $( &$bind, )*) {
                    break;
                }
            }
        });
        $crate::ActorStruct::new(__chek_tx)
    });
}

#[macro_export]
macro_rules! actor_mut {
    ($script:expr, $($bind:ident=$val:expr),*) => ({
        $( let mut $bind = $val; )*
        let (__chek_tx, __chek_rx) = ::std::sync::mpsc::channel();
        ::std::thread::spawn(move || {
            while let Ok(__chek_msg) = __chek_rx.recv() {
                if let Err(_) = $script(__chek_msg, $( &mut $bind, )*) {
                    break;
                }
            }
        });
        $crate::ActorStructMut::new(__chek_tx)
    });
}

#[macro_export]
macro_rules! actor_loop {
    ($script:expr, $($bind:ident=$val:expr),*) => ({
        $( let mut $bind = $val; )*
        ::std::thread::spawn(move || {
            loop {
                if let Err(_) = $script($( &mut $bind, )*) {
                    break;
                }
            }
        });
    });
}
