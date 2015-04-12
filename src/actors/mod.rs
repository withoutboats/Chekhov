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
pub mod scene;

use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;

pub use self::understudy::Understudy;
pub use self::scene::{Scene, Scenic};

pub struct Telegram<M: Send + 'static>(pub M, pub u32);

pub struct Null;

pub enum ActorError {
    CueError,
    Internal(String),
}

pub trait Cueable {
    type Message: Send;
    fn cue(&self, msg: Self::Message) -> Result<(), ActorError>;
}

pub struct Actor<M: Send + 'static, A: Send + 'static> {
    tx: Sender<Telegram<M>>,
    tag: u32,
    head_count: Arc<Mutex<u32>>,
    phantom: PhantomData<*const A>,
}

impl<M: Send, A: Send> Actor<M, A> {

    pub fn new(tx: Sender<Telegram<M>>) -> Actor<M, A> {
        Actor {
            tx: tx,
            tag: 0,
            head_count: Arc::new(Mutex::new(0)),
            phantom: PhantomData
        }
    }
}

impl<M: Send, A: Send> Cueable for Actor<M, A> {
    type Message = M;

    fn cue(&self, data: M) -> Result<(), ActorError> {
        self.tx.send(Telegram(data, self.tag)).map_err(|_| ActorError::CueError)
    }

}

impl<M: Send, A: Send> Clone for Actor<M, A> where A: Clone {
    fn clone(&self) -> Actor<M, A> {
        let tag;
        {
            let mut n = self.head_count.lock().unwrap();
            *n += 1;
            tag = *n;
        }
        Actor {
            tx: self.tx.clone(),
            tag: tag,
            head_count: self.head_count.clone(),
            phantom: PhantomData,
        }
    }
}
