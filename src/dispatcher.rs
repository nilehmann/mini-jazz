use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData};

use crate::{actor::PersistentActor, context::Context};

use super::{
    actor::{AnyActorId, PersistentActorId},
    handler::Handler,
    message::{AnyMessage, Message},
};

pub struct Dispatcher {
    pub(crate) messages: HashMap<AnyActorId, AnyMessage>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    pub fn send<A, M>(&mut self, actor_id: PersistentActorId<A>, message: M)
    where
        M: Message,
        A: Handler<M>,
    {
        self.messages
            .insert(AnyActorId::from(actor_id), AnyMessage::from(message));
    }

    pub fn create_callback<A, M>(&mut self) -> CallbackId<M>
    where
        A: Callback<M, Env = ()>,
    {
        self.create_callback_with_env::<A, M>(())
    }

    pub fn create_callback_with_env<A, M>(&mut self, env: A::Env) -> CallbackId<M>
    where
        A: Callback<M>,
    {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct CallbackId<M> {
    pub id: u32,
    _marker: PhantomData<M>,
}

impl<M> CallbackId<M> {
    // Note: If we want to support this a CallbackId needs to hold a reference to the dispatcher.
    // That would require some interior mutability. Also that would mean that a CallbackId is not
    //Alternative one can have a `call` function in dispatcher.
    pub fn call(&self, args: M) {
        todo!()
    }
}

pub trait Callback<M>: PersistentActor {
    type Env;

    fn handle(&self, cx: &mut Context<Self>, env: Self::Env, msg: M);
}
