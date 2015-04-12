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

use std::iter::Unfold;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};

use super::{Actor, ActorError, Cueable, Telegram, Null};

pub struct Understudy<M: Send + 'static> {
    tx: Sender<Telegram<M>>,
    rx: Receiver<Telegram<M>>
}

impl<M: Send> Understudy<M> {

    pub fn new() -> Understudy<M> {
        let (tx, rx) = channel();
        Understudy { tx: tx, rx: rx }
    }

    pub fn read(&mut self) -> Vec<M> {
        Unfold::new(&mut self.rx, |rx| rx.try_recv().map(|gram| gram.0).ok())
               .collect::<Vec<_>>()
    }
    
    pub fn read_all(self) -> Vec<M> {
        drop(self.tx);
        self.rx.iter().map(|gram| gram.0).collect::<Vec<_>>()
    }
    
    pub fn stage(&self) -> Actor<M, Null> {
        Actor {
            tx: self.tx.clone(),
            tag: 0,
            head_count: Arc::new(Mutex::new(0)),
            phantom: PhantomData,
        }
    }

}

impl<M: Send> Cueable for Understudy<M> {
    type Message = M;

    fn cue(&self, data: M) -> Result<(), ActorError> {
        self.tx.send(Telegram(data, 0)).map_err(|_| ActorError::CueError)
    }

}

#[cfg(test)]
mod tests {

    use actors::*;

    actor!{ Fount5(next: Actor<u8>) => {
        for i in 0..5 { try!(next.cue(i)) }
        break_a_leg!();
    }}

    #[test]
    fn it_collects_messages_sent_to_it() {
        let next = super::Understudy::new();
        let fount = Fount5::new(next.stage().unwrap());
        fount.action();
        assert_eq!(next.read_all(), vec![0,1,2,3,4]);
    }

    #[test]
    fn it_can_be_cued_directly() {
        let understudy: Understudy<u8> = super::Understudy::new();
        assert!(understudy.cue(0).is_ok());
        assert_eq!(understudy.read_all(), vec![0]);
    }

}
