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

use std::sync::mpsc;

pub struct Understudy<M: Send + 'static> {
    actor: super::Actor<M>,
    rx: mpsc::Receiver<M>,
}

impl<M: Send + 'static> Understudy<M> {

    pub fn new() -> Understudy<M> {
        let (tx, rx) = mpsc::channel();
        Understudy {
            actor: super::Actor::new_understudy(tx),
            rx: rx,
        }
    }

    pub fn read_all(self) -> Vec<M> {
        drop(self.actor);
        self.rx.iter().collect()
    }

    pub fn stage(&self) -> super::Actor<M> { self.actor.stage() }

    pub fn try_recv(&self) -> Result<M, mpsc::TryRecvError> { self.rx.try_recv() }

    pub fn recv(&self) -> Result<M, mpsc::RecvError> { self.rx.recv() }

}

impl<M: Send + 'static> Into<mpsc::Receiver<M>> for Understudy<M> {
    fn into(self) -> mpsc::Receiver<M> { self.rx }
}

#[cfg(test)]
mod tests {

    use actors::*;

    fn fountain_5(next: Actor<u8>) {
        for i in 0..5 { next.cue(i).ok(); }
    }

    #[test]
    fn collects_messages_sent_to_it() {
        let understudy = Understudy::new();
        fountain_5(understudy.stage());
        assert_eq!(understudy.read_all(), vec![0,1,2,3,4]);
    }

    #[test]
    fn can_be_cast_to_a_receiver() {
        let _: ::std::sync::mpsc::Receiver<i32> = Understudy::new().into();
    }

}
