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
    { $actor:ident ($( $arg:ident : $t:ty ),*) :: $msg:ident : $reads:ty => $script:block }
    => {
        pub struct $actor;
        #[allow(unused_mut, unreachable_code)]
        impl $actor {
            pub fn new($( $arg: $t, )*) -> $crate::Actor<$reads> {
                let (__chek_tx, __chek_rx) = ::std::sync::mpsc::channel();
                let __chek_script = move |$( $arg: $t, )*| -> Result<(), $crate::ActorError> {
                    while let Ok($msg) = __chek_rx.recv() { $script }
                    Ok(())
                };
                ::std::thread::spawn(move || { __chek_script($( $arg, )*).ok(); });
                $crate::ActorStruct::new(__chek_tx)
            }
        }
    };
    { $actor:ident ($( $arg:ident : $t:ty ),*) => $script:block }
    => {
        pub struct $actor;
        #[allow(unused_mut, unreachable_code)]
        impl $actor {
            pub fn new($( $arg: $t, )*) {
                let __chek_script = move |$( $arg: $t, )*| -> Result<(), $crate::ActorError> {
                    loop { $script }
                    Ok(())
                };
                ::std::thread::spawn(move || { __chek_script($( $arg, )*).ok(); });
            }
        }
    };
}

#[macro_export]
macro_rules! actor_mut {
    { $actor:ident ($( $arg:ident : $t:ty ),*) :: $msg:ident : $reads:ty => $script:block }
    => {
        pub struct $actor;
        #[allow(unused_mut, unreachable_code)]
        impl $actor {
            pub fn new($( mut $arg: $t, )*) -> $crate::Actor<$reads> {
                let (__chek_tx, __chek_rx) = ::std::sync::mpsc::channel::<$reads>();
                let __chek_script = move |$( mut $arg: $t, )*| -> Result<(), $crate::ActorError> {
                    while let Ok($msg) = __chek_rx.recv() { $script }
                    Ok(())
                };
                ::std::thread::spawn(move || { __chek_script($( $arg, )*).ok(); });
                $crate::ActorStructMut::new(__chek_tx)
            }
        }
    };
    { $actor:ident ($( $arg:ident : $t:ty ),*) => $script:block }
    => {
        pub struct $actor;
        #[allow(unused_mut, unreachable_code)]
        impl $actor {
            pub fn new($( mut $arg: $t, )*) {
                let __chek_script = move |$( mut $arg: $t, )*| -> Result<(), $crate::ActorError> {
                    loop { $script }
                    Ok(())
                };
                ::std::thread::spawn(move || { __chek_script($( $arg, )*).ok(); });
            }
        }
    };
}
