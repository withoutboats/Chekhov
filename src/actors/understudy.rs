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

use super::{Actor, ActorStruct, ActorResult, ActorError, Message};

pub struct Understudy<M: Send + 'static>(Sender<Message<M>>, Receiver<Message<M>>);

impl<M: Send + 'static> Understudy<M> {

    pub fn new() -> Understudy<M> {
        let (tx, rx) = channel();
        Understudy(tx, rx)
    }

    pub fn read(&self) -> Vec<M> {
        let mut out = Vec::new();
        while let Ok(msg) = self.1.try_recv() {
            match msg {
                Message::Cue(data) => out.push(data),
                Message::Cut => break,
            }
        }
        out
    }
    
    pub fn read_all(self) -> Vec<M> {
        drop(self.0);
        self.1.iter().map(Message::into)
                     .take_while(Option::is_some)
                     .map(Option::unwrap)
                     .collect::<Vec<_>>()
    }
    
}

impl<M: Send + 'static> Actor<M> for Understudy<M> {

    fn cue(&self, msg: M) -> ActorResult {
        self.0.send(Message::Cue(msg)).map_err(|_| ActorError::CueError)
    }

    fn cut(&self) -> ActorResult {
        self.0.send(Message::Cut).map_err(|_| ActorError::CueError)
    }

    fn stage(&self) -> Option<ActorStruct<M>> {
        Some(ActorStruct::new(self.0.clone()))
    }

}

#[cfg(test)]
mod tests {

    use actors::*;

    fn fountain_5<A: Actor<u8>>(next: A) {
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
