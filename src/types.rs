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

#[derive(Clone)]
pub struct Actor<T: Send>(Sender<T>);

pub struct ProspectiveActor<T: Send, A: ActorThread<T>>(A, Sender<T>);

impl<T: Send, A: ActorThread<T>> ProspectiveActor<T, A> {

    pub fn new(actor: A, tx: Sender<T>) -> ProspectiveActor<T, A> {
        ProspectiveActor(actor, tx)
    }

    pub fn stage(&self)-> Actor<T> {
        Actor(self.1.clone())
    }
}

pub struct IsolatedActor<A: ActorThread<Null>>(A);

impl<A: ActorThread<Null>> IsolatedActor<A> {
    pub fn new(actor: A) -> IsolatedActor<A> {
        IsolatedActor(actor)
    }
}

pub struct Null;

pub trait Actionable {
    fn action(self: Box<Self>); 
}

impl<T: Send, A: ActorThread<T>> Actionable for ProspectiveActor<T, A> {
    fn action(self: Box<Self>) {
        self.0.go();
    }
}

impl<A: ActorThread<Null>> Actionable for IsolatedActor<A> {
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

impl<T: Send, A: ActorThread<T>> Cueable for ProspectiveActor<T, A> {
    type Message = T;
    fn cue(&self, data: T) {
        self.1.send(data).ok();
    }
}

pub trait ActorThread<T: Send>: Send {
    fn go(self);
}
