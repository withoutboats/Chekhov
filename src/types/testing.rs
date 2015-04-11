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
use std::sync::mpsc::{channel, Sender, Receiver};

use super::{Actor, Cueable, ActorError};

pub struct Understudy<T: Send>(Sender<T>, Receiver<T>);

impl<T: Send> Understudy<T> {

    pub fn new() -> Understudy<T> {
        let (tx, rx) = channel();
        Understudy(tx, rx)
    }

    pub fn stage(&self) -> Actor<T> {
        Actor(self.0.clone())
    }

    pub fn read(&mut self) -> Vec<T> {
        Unfold::new(&mut self.1, |tx| tx.try_recv().ok())
               .collect::<Vec<_>>()
    }
    
    pub fn read_all(self) -> Vec<T> {
        drop(self.0);
        self.1.iter().collect::<Vec<_>>()
    }

}

impl<T: Send> Cueable for Understudy<T> {
    type Message = T;

    fn cue(&self, data: T) -> Result<(), ActorError> {
        self.0.send(data).map_err(|_| ActorError::CueError)
    }

}


#[cfg(test)]
mod tests {

    use types::*;

    actor!{ Fount5(next: Actor<u8>) => {
        for i in 1..5 { try!(next.cue(i)) }
        break_a_leg!();
    }}

    #[test]
    fn it_collects_messages_sent_to_it() {
        let next = super::Understudy::new();
        let fount = Fount5::new(next.stage());
        fount.action();
        assert_eq!(next.read_all(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn it_can_be_cued_directly() {
        let understudy: Understudy<u8> = super::Understudy::new();
        assert!(understudy.cue(0).is_ok());
        assert_eq!(understudy.read_all(), vec![0]);
    }

}
