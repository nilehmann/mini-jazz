use std::collections::HashMap;

use crate::{
    database::Database,
    global_storage::{GlobalEffect, GlobalStorage, GlobalStorageCache},
};

use super::{actor::AnyActorId, dispatcher::Dispatcher, message::AnyMessage};

pub struct Effects {
    messages: HashMap<AnyActorId, AnyMessage>,
    global_effects: HashMap<String, GlobalEffect>,
}

pub struct Context<'a> {
    pub storage: GlobalStorage<'a>,
    pub dispatcher: Dispatcher,
}

impl<'a> Context<'a> {
    pub fn new(db: &'a dyn Database, cache: &'a mut GlobalStorageCache) -> Self {
        Self {
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
