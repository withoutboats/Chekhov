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

use std::error::Error;
use std::fmt;
use std::iter::IntoIterator;
use std::sync::mpsc::Sender;

pub use self::understudy::Understudy;

pub enum Actor<M: Send + 'static> {
    Actor(Sender<Message<M>>),
    ActorMut(Sender<Message<M>>),
    Understudy(Sender<M>),
}

impl<M: Send + 'static> Actor<M> {

    pub fn new(tx: Sender<Message<M>>) -> Actor<M> { Actor::Actor(tx) }

    pub fn new_mut(tx: Sender<Message<M>>) -> Actor<M> { Actor::ActorMut(tx) }

    pub fn new_understudy(tx: Sender<M>) -> Actor<M> { Actor::Understudy(tx) }

    pub fn cue(&self, msg: M) -> ActorResult {
        match *self {
            Actor::Actor(ref tx)
                | Actor::ActorMut(ref tx)
                => tx.send(Message::Cue(msg)).map_err(From::from),
            Actor::Understudy(ref tx) => tx.send(msg).map_err(From::from)
        }
    }

    pub fn cue_all<I: IntoIterator<Item=M>>(&self, iterable: I) -> ActorResult {
        for msg in iterable { try!(self.cue(msg)); }
        Ok(())
    }

    pub fn cut(&self) -> ActorResult {
        match *self {
            Actor::Actor(ref tx)
                | Actor::ActorMut(ref tx)
                => tx.send(Message::Cut).map_err(From::from),
            _ => Ok(())
        }
    }

    pub fn duplicate(&self) -> Option<Actor<M>> {
        match * self {
            Actor::Actor(ref tx) => Some(Actor::new(tx.clone())),
            Actor::ActorMut(_) => None,
            Actor::Understudy(ref tx) => Some(Actor::new_understudy(tx.clone())),
        }
    }

}

pub type ActorResult = Result<(), Box<Error>>;

pub enum Message<M: Send + 'static> {
    Cue(M), Cut,
}

impl<M: Send + 'static> Into<Option<M>> for Message<M> {
    fn into(self) -> Option<M> {
        match self {
            Message::Cue(data) => Some(data),
            Message::Cut       => None,
        }
    }
}

#[derive(Debug)]
pub struct ActorFinished;

impl fmt::Display for ActorFinished {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> { Ok(()) }
}

impl Error for ActorFinished {
    fn description(&self) -> &str { "actor has finished working" }
}

pub fn curtain_call() -> ActorResult {
    Err(Box::new(ActorFinished))
}

#[cfg(test)]
mod tests {

    use actors::*;

        fn sum(msg: u8, x: &mut u8, next: &Actor<u8>) -> ActorResult {
        *x += msg;
        try!(next.cue(*x));
        if *x == 5 { curtain_call() }
        else { Ok(()) }
    }
    
    fn double(msg: u8, next: &Actor<u8>) -> ActorResult {
        try!(next.cue(msg * 2));
        Ok(())
    }

    #[test]
    fn actors_work() {
        let understudy = Understudy::new();
        actor_loop!(|x: &u8, next: &Actor<u8>| next.cue(*x),
                    1, actor_mut!(sum, 0, actor!(double, understudy.stage())));
        assert_eq!(understudy.read_all(), vec![2,4,6,8,10]);
    }

    #[test]
    fn actors_can_be_cued_all() {
        let understudy = Understudy::new();
        let actor = actor!(double, understudy.stage());
        assert!(actor.cue_all(0..5).is_ok());
        assert!(actor.cut().is_ok());
        assert_eq!(understudy.read_all(), vec![0,2,4,6,8]);
    }

}
