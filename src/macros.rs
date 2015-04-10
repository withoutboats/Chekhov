// This file is a part of Chekhov, an actor/model concurrency framework for Rust.
//
// Chekhov is free software; you can redistribute it and/or modify it under the terms of the Lesser
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License or (at your option) any later version.
//
// Chekhov is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Chekhov. If
// not, see <https://www.gnu.org/licenses/>.

#![macro_use]

#[macro_export]
macro_rules! actor_mut {
    {$actor:ident ($( $arg:ident : $t:ty ),*) |> $binding:ident : $reads:ty => $script:expr} => {
        struct $actor {
            func: Box<::std::boxed::FnBox(::std::sync::mpsc::Receiver<$reads>, $( $t, )*) + Send>,
            rx: ::std::sync::mpsc::Receiver<$reads>,
            $( $arg: $t, )*
        }
        #[allow(unused_mut)]
        impl $actor {
            fn new($( $arg: $t, )*) -> Box<$crate::ProspectiveActor<$reads, $actor>> {
                let (tx, rx) = ::std::sync::mpsc::channel();
                Box::new($crate::ProspectiveActor($actor {
                    func: Box::new(|rx: ::std::sync::mpsc::Receiver<$reads>, $( mut $arg: $t, )*| 
                        while let Ok($binding) = rx.recv() { $script }
                    ),
                    rx: rx,
                    $( $arg: $arg, )*
                }, tx))
            }
        }
        impl $crate::ActorThread<$reads> for $actor {
            fn go(self) {
                ::std::thread::spawn(move || (self.func)(self.rx, $( self.$arg, )*));
            }
        }
    };
    {$actor:ident ($( $arg:ident : $t:ty ),*) => $script:expr} => {
        struct $actor {
            func: Box<::std::boxed::FnBox($( $t, )*) + Send>,
            $( $arg : $t, )*
        }
        #[allow(unused_mut)]
        impl $actor {
            fn new($( $arg: $t, )*) -> Box<$crate::IsolatedActor<$actor>> {
                Box::new($crate::IsolatedActor($actor {
                    func: Box::new(|$( mut $arg: $t, )*| loop { $script }),
                    $( $arg: $arg, )*
                }))
            }
        }
        impl $crate::ActorThread<Null> for $actor {
            fn go(self) {
                ::std::thread::spawn(move || (self.func)($( self.$arg, )*));
            }
        }
    };
}

#[macro_export]
macro_rules! actor {
    {$actor:ident ($( $arg:ident : $t:ty ),*) |> $binding:ident : $reads:ty => $script:expr} => {
        struct $actor {
            func: Box<::std::boxed::FnBox(::std::sync::mpsc::Receiver<$reads>, $( $t, )*) + Send>,
            rx: ::std::sync::mpsc::Receiver<$reads>,
            $( $arg: $t, )*
        }
        impl $actor {
            fn new($( $arg: $t, )*) -> Box<$crate::ProspectiveActor<$reads, $actor>> {
                let (tx, rx) = ::std::sync::mpsc::channel();
                Box::new($crate::ProspectiveActor($actor {
                    func: Box::new(|rx: ::std::sync::mpsc::Receiver<$reads>, $( $arg: $t, )*| 
                        while let Ok($binding) = rx.recv() { $script }
                    ),
                    rx: rx,
                    $( $arg: $arg, )*
                }, tx))
            }
        }
        impl $crate::ActorThread<$reads> for $actor {
            fn go(self) {
                ::std::thread::spawn(move || (self.func)(self.rx, $( self.$arg, )*));
            }
        }
    };
    {$actor:ident ($( $arg:ident : $t:ty ),*) => $script:expr} => {
        struct $actor {
            func: Box<::std::boxed::FnBox($( $t, )*) + Send>,
            $( $arg : $t, )*
        }
        impl $actor {
            fn new($( $arg: $t, )*) -> Box<$crate::IsolatedActor<$actor>> {
                Box::new($crate::IsolatedActor($actor {
                    func: Box::new(|$( $arg: $t, )*| loop { $script }),
                    $( $arg: $arg, )*
                }))
            }
        }
        impl $crate::ActorThread<$crate::Null> for $actor {
            fn go(self) {
                ::std::thread::spawn(move || (self.func)($( self.$arg, )*));
            }
        }
    };
}
