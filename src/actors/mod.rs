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

pub mod understudy;

use std::sync::mpsc::Sender;

pub use self::understudy::Understudy;

pub struct Null;

pub type Actor<M> = Box<Cueable<Message=M>>;

pub trait Cueable: Send {
    type Message: Send + 'static;
    fn cue(&self, msg: Self::Message) -> Result<(), ActorError>;
    fn stage(&self) -> Option<Actor<Self::Message>>;
}

#[derive(Clone)]
pub struct ActorStruct<M: Send + 'static>(Sender<M>);

impl<M: Send + 'static> ActorStruct<M> {
    pub fn new(tx: Sender<M>) -> Actor<M> {
        Box::new(ActorStruct(tx))
    }
}

impl<M: Send + 'static> Cueable for ActorStruct<M> {
    type Message = M;
    fn cue(&self, msg: M) -> Result<(), ActorError> {
        self.0.send(msg).map_err(|_| ActorError::CueError)
    }
    fn stage(&self) -> Option<Actor<M>> {
        Some(Self::new(self.0.clone()))
    }
}

pub struct ActorStructMut<M: Send + 'static>(Sender<M>);

impl<M: Send + 'static> ActorStructMut<M> {
    pub fn new(tx: Sender<M>) -> Actor<M> {
        Box::new(ActorStructMut(tx))
    }
}

impl<M: Send + 'static> Cueable for ActorStructMut<M> {
    type Message = M;
    fn cue(&self, msg: M) -> Result<(), ActorError> {
        self.0.send(msg).map_err(|_| ActorError::CueError)
    }
    fn stage(&self) -> Option<Actor<M>> { None }
}

pub enum ActorError {
    CueError,
    Internal(String),
}

#[cfg(test)]
mod tests {

    use actors::*;

    actor!{ Generator(x: u8, next: Actor<u8>) => {
        try!(next.cue(x));
    }}  

    actor_mut!{ Summer(x: u8, next: Actor<u8>) :: msg: u8 => {
        x += msg;
        try!(next.cue(x));
        if x == 5 { break_a_leg!(); }
    }}
    
    actor!{ Doubler(next: Actor<u8>) :: msg: u8 => {
        try!(next.cue(msg * 2));
    }}

    #[test]
    fn actors_work() {
        let understudy = Understudy::new();
        Generator::new(1, Summer::new(0, Doubler::new(understudy.stage().unwrap())));
        assert_eq!(understudy.read_all(), vec![2,4,6,8,10]);
    }

    #[test]
    #[should_panic]
    fn actor_mut_wont_clone() {
        let actor = Summer::new(0, Understudy::new().stage().unwrap());
        actor.stage().unwrap();
    }

}
