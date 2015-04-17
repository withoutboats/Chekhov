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

use std::sync::mpsc::{channel, Sender, Receiver};

use super::{Actor, ActorStruct, ActorError, Cued};

pub struct Understudy<M: Send + 'static>(Sender<M>, Receiver<M>);

impl<M: Send + 'static> Understudy<M> {

    pub fn new() -> Box<Understudy<M>> {
        let (tx, rx) = channel();
        Box::new(Understudy(tx, rx))
    }

    pub fn read(&self) -> Vec<M> {
        let mut out = Vec::new();
        while let Ok(data) = self.1.try_recv() { out.push(data); }
        out
    }
    
    pub fn read_all(self) -> Vec<M> {
        drop(self.0);
        self.1.iter().collect::<Vec<_>>()
    }
    
}

impl<M: Send + 'static> Cued for Understudy<M> {
    type Message = M;

    fn cue(&self, msg: M) -> Result<(), ActorError> {
        self.0.send(msg).map_err(|_| ActorError::CueError)
    }

    fn stage(&self) -> Option<Actor<M>> {
        Some(ActorStruct::new(self.0.clone()))
    }

}

#[cfg(test)]
mod tests {

    use actors::*;

    fn fountain_5(next: Actor<u8>) {
        for i in 0..5 { next.cue(i).ok(); }
    }

    #[test]
    fn it_collects_messages_sent_to_it() {
        let understudy = super::Understudy::new();
        fountain_5(understudy.stage().unwrap());
        assert_eq!(understudy.read_all(), vec![0,1,2,3,4]);
    }

    #[test]
    fn it_can_be_cued_directly() {
        let understudy = super::Understudy::new();
        assert!(understudy.cue(0u8).is_ok());
        assert_eq!(understudy.read_all(), vec![0]);
    }

}
