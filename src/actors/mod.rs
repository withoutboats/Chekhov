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

pub type ActorResult = Result<(), Box<Error>>;

pub trait Actor<M: Send + 'static>: Send {
    fn cue(&self, msg: M) -> ActorResult;
    fn cut(&self) -> ActorResult;
    fn stage(&self) -> Option<ActorStruct<M>>;
    
    fn cue_all<I: IntoIterator<Item=M>>(&self, iterable: I) -> ActorResult {
        for msg in iterable { try!(self.cue(msg)); }
        Ok(())
    }
}

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

#[derive(Clone)]
pub struct ActorStruct<M: Send + 'static>(Sender<Message<M>>);

impl<M: Send + 'static> ActorStruct<M> {
    pub fn new(tx: Sender<Message<M>>) -> ActorStruct<M> {
        ActorStruct(tx)
    }
}

impl<M: Send + 'static> Actor<M> for ActorStruct<M> {

    fn cue(&self, msg: M) -> ActorResult {
        self.0.send(Message::Cue(msg)).map_err(From::from)
    }

    fn cut(&self) -> ActorResult {
        self.0.send(Message::Cut).map_err(From::from)
    }

    fn stage(&self) -> Option<ActorStruct<M>> { Some(Self::new(self.0.clone())) }

}

pub struct ActorStructMut<M: Send + 'static>(Sender<Message<M>>);

impl<M: Send + 'static> ActorStructMut<M> {
    pub fn new(tx: Sender<Message<M>>) -> ActorStructMut<M> {
        ActorStructMut(tx)
    }
}

impl<M: Send + 'static> Actor<M> for ActorStructMut<M> {

    fn cue(&self, msg: M) -> ActorResult {
        self.0.send(Message::Cue(msg)).map_err(From::from)
    }

    fn cut(&self) -> ActorResult {
        self.0.send(Message::Cut).map_err(From::from)
    }

    fn stage(&self) -> Option<ActorStruct<M>> { None }

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

    fn sum<A: Actor<u8>>(msg: u8, x: &mut u8, next: &A) -> ActorResult {
        *x += msg;
        try!(next.cue(*x));
        if *x == 5 { curtain_call() }
        else { Ok(()) }
    }
    
    fn double<A: Actor<u8>>(msg: u8, next: &A) -> ActorResult {
        try!(next.cue(msg * 2));
        Ok(())
    }

    #[test]
    fn actors_work() {
        let understudy = Understudy::new();
        actor_loop!(|x: &u8, next: &ActorStructMut<u8>| next.cue(*x),
                    1, actor_mut!(sum, 0, actor!(double, understudy.stage().unwrap())));
        assert_eq!(understudy.read_all(), vec![2,4,6,8,10]);
    }

    #[test]
    fn actors_can_be_cued_all() {
        let understudy = Understudy::new();
        let actor = actor!(double, understudy.stage().unwrap());
        assert!(actor.cue_all(0..5).is_ok());
        assert!(actor.cut().is_ok());
        assert_eq!(understudy.read_all(), vec![0,2,4,6,8]);
    }

    #[test]
    #[should_panic]
    fn actor_mut_wont_clone() {
        let actor = actor_mut!(sum, 0, Understudy::new().stage().unwrap());
        actor.stage().unwrap();
    }

}
