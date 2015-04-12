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

use std::boxed::FnBox;
use std::sync::mpsc::Receiver;
use std::thread;

use actors::{Telegram, ActorError};

pub type FnActor<M, A> = Box<FnBox(A, Receiver<Telegram<M>>) -> Result<(), ActorError> + Send>;

pub struct ActorThread<M: Send + 'static, A: Send + 'static> {
    func: FnActor<M, A>,
    args: A,
    rx: Receiver<Telegram<M>>,
}

impl<M: Send, A: Send> ActorThread<M, A> {
    fn go(self) {
        thread::spawn(move || { (self.func)(self.args, self.rx).ok(); });
    }
}

pub struct Scene<M: Send + 'static, A: Send + 'static>(ActorThread<M, A>);

pub trait Scenic {
    fn action(self: Box<Self>);
}

impl<M: Send, A: Send> Scene<M, A> {

    pub fn new(func: FnActor<M, A>, args: A, rx: Receiver<Telegram<M>>) -> Scene<M, A> {
        Scene(ActorThread { func: func, args: args, rx: rx, })
    }
}

impl<M: Send, A: Send> Scenic for Scene<M, A> {
    fn action(self: Box<Self>) {
        self.0.go();
    }
}
