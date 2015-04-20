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
    ActorMut(Sender<Message<M>>, bool),
    ActorLoop(Sender<Message<M>>),
    Understudy(Sender<M>),
}

impl<M: Send + 'static> Actor<M> {

    pub fn new(tx: Sender<Message<M>>) -> Actor<M> { Actor::Actor(tx) }

    pub fn new_mut(tx: Sender<Message<M>>) -> Actor<M> { Actor::ActorMut(tx, true) }

    pub fn new_loop(tx: Sender<Message<M>>) -> Actor<M> { Actor::ActorLoop(tx) }

    pub fn new_understudy(tx: Sender<M>) -> Actor<M> { Actor::Understudy(tx) }

    pub fn cue(&self, msg: M) -> ActorResult {
        match *self {
            Actor::Actor(ref tx)
                | Actor::ActorMut(ref tx, true)
                => tx.send(Message::Cue(msg)).map_err(From::from),
            Actor::Understudy(ref tx) => tx.send(msg).map_err(From::from),
            _ => Ok(())
        }
    }

    pub fn cue_all<I: IntoIterator<Item=M>>(&self, iterable: I) -> ActorResult {
        for msg in iterable { try!(self.cue(msg)); }
        Ok(())
    }

    pub fn cut(&self) -> ActorResult {
        self.pass_msg(Message::Cut)
    }

    pub fn stage(&self) -> Actor<M> {
        match *self {
            Actor::Actor(ref tx)       => Actor::new(tx.clone()),
            Actor::ActorMut(ref tx, _) => Actor::ActorMut(tx.clone(), false),
            Actor::ActorLoop(ref tx)   => Actor::new_loop(tx.clone()),
            Actor::Understudy(ref tx)  => Actor::new_understudy(tx.clone()),
        }
    }

    fn pass_msg(&self, msg: Message<M>) -> ActorResult {
        match * self {
            Actor::Actor(ref tx)
                | Actor::ActorMut(ref tx, _)
                | Actor::ActorLoop(ref tx)
                => tx.send(msg).map_err(From::from),
            _ => Ok(())
        }
    }

}

pub trait Direction {
    fn start(&self) -> ActorResult;
    fn pause(&self) -> ActorResult;
}

impl<M: Send + 'static> Direction for Actor<M> {
    
    fn start(&self) -> ActorResult {
        self.pass_msg(Message::Start)
    }

    fn pause(&self) -> ActorResult {
        self.pass_msg(Message::Pause)
    }

}

pub type ActorResult = Result<(), Box<Error>>;

#[derive(PartialEq)]
pub enum Message<M: Send + 'static> {
    Cue(M), Start, Pause, Cut,
}

impl<M: Send + 'static> Into<Option<M>> for Message<M> {
    fn into(self) -> Option<M> {
        match self {
            Message::Cue(data) => Some(data),
            _                  => None,
        }
    }
}

#[derive(PartialEq)]
pub struct Null;

#[derive(Debug)]
pub struct ActorFinished;

impl fmt::Display for ActorFinished {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> { Ok(()) }
}

impl Error for ActorFinished {
    fn description(&self) -> &str { "actor has finished working" }
}

pub fn from_the_top(actors: Vec<&Direction>) -> ActorResult {
    for actor in actors {
        try!(actor.start());
    }
    Ok(())
}

pub fn curtain_call() -> ActorResult {
    Err(Box::new(ActorFinished))
}

#[cfg(test)]
mod tests {

    use actors::*;

    #[test]
    fn can_cue() {
        let understudy = Understudy::new();
        let actor = actor!(|msg: u8, next: &Actor<u8>| next.cue(msg), understudy.stage());
        assert!(actor.start().is_ok());
        assert!(actor.cue(0).is_ok());
        assert!(actor.cut().is_ok());
        assert_eq!(understudy.read_all(), vec![0]);
    }

    #[test]
    fn can_be_mutable() {
        let understudy = Understudy::new();
        let actor = actor_mut!(|msg: u8, x: &mut u8, next: &Actor<u8>| { *x += msg; next.cue(*x) },
                               1, understudy.stage());
        assert!(actor.start().is_ok());
        assert!(actor.cue(1).is_ok());
        assert!(actor.cue(2).is_ok());
        assert!(actor.cut().is_ok());
        assert_eq!(understudy.read_all(), vec![2, 4]);
    }

    #[test]
    fn can_cue_all() {
        let understudy = Understudy::new();
        let actor = actor!(|msg: u8, next: &Actor<u8>| next.cue(msg * 2), understudy.stage());
        assert!(actor.cue_all(0..5).is_ok());
        assert!(actor.cut().is_ok());
        assert_eq!(understudy.read_all(), vec![0,2,4,6,8]);
    }

    #[test]
    fn can_curtain_call() {
        let actor = actor!(|_: u8| curtain_call());
        assert!(actor.start().is_ok());
        assert!(actor.cue(0).is_ok());
    }

    #[test]
    fn can_from_the_top() {
        let actor = actor!(|_: u8| -> ActorResult { Ok(()) });
        let actor_loop = actor_loop!(|next: &Actor<u8>| next.cue(0), actor.stage());
        assert!(from_the_top(vec![&actor, &actor_loop]).is_ok());
        assert!(actor_loop.cut().is_ok());
    }

    #[test]
    fn propogates_failure_from_other_actors() {
        let actor1 = actor!(|_: bool| -> ActorResult { Ok(()) });
        let actor2 = actor!(|msg: bool, next: &Actor<bool>|
                            if msg == false { curtain_call() } else { next.cue(msg) },
                            actor1.stage());
        let actor_loop = actor_loop!(|next: &Actor<bool>| next.cue(true), actor2.stage());
        assert!(from_the_top(vec![&actor1, &actor2, &actor_loop]).is_ok());
        assert!(actor1.cue(false).is_ok());
        //NOTE: Unfortunately, this test will hang if it is broken, rather than fail.
    }

}
