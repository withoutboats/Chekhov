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

use super::{Actor, Message};

pub struct Understudy<M: Send + 'static>(Sender<Message<M>>, Receiver<Message<M>>);

impl<M: Send + 'static> Understudy<M> {

    pub fn new() -> Understudy<M> {
        let (tx, rx) = channel();
        Understudy(tx, rx)
    }

    pub fn recv(&self) -> Option<M> {
        self.1.try_recv().ok().map(Message::into)   // Option<Message<M>> -> Option<Option<M>>
                              .map(Option::unwrap)  // Option<Option<M>> -> Option<M>
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
        self.1.iter().map(Message::into)            // Message<M> -> Option<M>
                     .take_while(Option::is_some)   // Exclude None
                     .map(Option::unwrap)           // Option<M> -> M
                     .collect::<Vec<_>>()           // Collect into a Vec
    }
    
    pub fn stage(&self) -> Actor<M> {
        Actor::new(self.0.clone())
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
        fountain_5(understudy.stage());
        assert_eq!(understudy.read_all(), vec![0,1,2,3,4]);
    }

}
