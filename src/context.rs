use std::collections::HashMap;

use crate::{
    actor::{PersistentActor, PersistentActorId},
    database::Database,
    global_storage::{GlobalEffect, GlobalStorage, GlobalStorageCache},
};

use super::{actor::AnyActorId, dispatcher::Dispatcher, message::AnyMessage};

pub struct Effects {
    messages: HashMap<AnyActorId, AnyMessage>,
    global_effects: HashMap<String, GlobalEffect>,
}

pub struct Context<'a, A>
where
    A: PersistentActor,
{
    pub actor_id: PersistentActorId<A>,
    pub storage: GlobalStorage<'a>,
    pub dispatcher: Dispatcher,
}

pub struct AnyContext<'a> {
    pub id: AnyActorId,
    pub storage: GlobalStorage<'a>,
    pub dispatcher: Dispatcher,
}

impl<'a, A> Context<'a, A>
where
    A: PersistentActor,
{
    pub fn new(
        actor_id: PersistentActorId<A>,
        db: &'a Database,
        cache: &'a mut GlobalStorageCache,
    ) -> Self {
        Self {
            actor_id,
            storage: GlobalStorage::new(db, cache),
            dispatcher: Dispatcher::new(),
        }
    }

    pub fn into_effects(self) -> Effects {
        Effects {
            messages: self.dispatcher.messages,
            global_effects: self.storage.effects,
        }
    }
}

impl<'a> AnyContext<'a> {
    pub fn new(id: AnyActorId, db: &'a Database, cache: &'a mut GlobalStorageCache) -> Self {
        Self {
            id,
            storage: GlobalStorage::new(db, cache),
            dispatcher: Dispatcher::new(),
        }
    }

    pub fn downcast<A>(self) -> Option<Context<'a, A>>
    where
        A: PersistentActor,
    {
        self.id.downcast::<A>().map(|actor_id| Context {
            actor_id,
            storage: self.storage,
            dispatcher: self.dispatcher,
        })
    }
}
