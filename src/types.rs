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

use std::sync::mpsc::Sender;

pub type BackstageActor<T, A> = Box<ActorBuilder<T, A>>;

#[derive(Clone)]
pub struct Actor<T: Send>(Sender<T>);

pub struct ActorBuilder<T: Send, A: ActorThread<T>>(pub A, pub Sender<T>);

impl<T: Send, A: ActorThread<T>> ActorBuilder<T, A> {
    pub fn stage(&self)-> Actor<T> {
        Actor(self.1.clone())
    }
}

pub struct Feeder<A: ActorThread<Null>>(pub A);

pub struct Null;

pub trait Actionable {
    fn action(self: Box<Self>); 
}

impl<T: Send, A: ActorThread<T>> Actionable for ActorBuilder<T, A> {
    fn action(self: Box<Self>) {
        self.0.go();
    }
}

impl<A: ActorThread<Null>> Actionable for Feeder<A> {
    fn action(self: Box<Self>) {
        self.0.go();
    }
}

pub trait Cueable {
    type Message: Send;
    fn cue(&self, data: Self::Message);
}

impl<T: Send> Cueable for Actor<T> {
    type Message = T;
    fn cue(&self, data: T) {
       self.0.send(data).ok();
    }
}

impl<T: Send, A: ActorThread<T>> Cueable for ActorBuilder<T, A> {
    type Message = T;
    fn cue(&self, data: T) {
        self.1.send(data).ok();
    }
}

pub trait ActorThread<T: Send>: Send {
    fn go(self);
}
