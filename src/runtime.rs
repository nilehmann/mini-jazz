use std::{
    any::Any,
    collections::HashMap,
    panic::{AssertUnwindSafe, RefUnwindSafe, UnwindSafe},
};

use crate::{
    actor::PersistentActorId,
    context::AnyContext,
    database::Database,
    dyn_table::{DispatchError, DispatchResult, DynTable},
    global_storage::{GlobalStorageCache, StorageError},
};

use super::{
    actor::{AnyActorId, PersistentActor},
    context::Effects,
    handler::Handler,
    log::{Log, LogIndex},
    message::{AnyMessage, Message},
};

pub struct ActorData<L>
where
    L: Log,
{
    id: AnyActorId,
    actor: Box<dyn Any + RefUnwindSafe>,
    cache: GlobalStorageCache,
    curr_log_index: L::LogIndex,
    log: L,
}

pub struct Runtime<L>
where
    L: Log,
{
    table: DynTable,
    actors: HashMap<AnyActorId, ActorData<L>>,
    next_id: u32,
    // TODO: The same database is used across all actors, should we ensure isolation?
    db: Database,
}

impl<L> Runtime<L>
where
    L: Log,
{
    pub fn new(db: Database) -> Self {
        Self {
            table: DynTable::new(),
            actors: HashMap::new(),
            next_id: 0,
            db,
        }
    }

    pub fn register_actor<A>(&mut self)
    where
        A: PersistentActor,
    {
        self.table.register_actor::<A>();
    }

    pub fn register_handler<A, M>(&mut self)
    where
        A: Handler<M>,
        M: Message,
    {
        self.table.register_handler::<A, M>();
    }

    pub fn add_actor<A>(&mut self, actor: A, log: L) -> PersistentActorId<A>
    where
        A: PersistentActor,
    {
        assert!(self.table.is_registered::<A>());
        self.next_id += 1;
        let id = PersistentActorId::new(self.next_id);
        let data = ActorData::new(id.into_any(), Box::new(actor), log);
        self.actors.insert(data.id, data);
        id
    }

    pub fn run(mut self) {
        for actor in self.actors.values_mut() {
            let result = actor.run_init(&self.table, &mut self.db);
            // TODO: apply_effects
        }

        // TODO: all this actors should run in parallel, that would require
        // transactionallity support in the database
        loop {
            let mut done = true;
            for actor in self.actors.values_mut() {}
        }
    }

    async fn do_step(&mut self, log: &mut L, actor_id: AnyActorId) -> Result<bool, L::Error> {
        let actor_data = self.actors.get_mut(&actor_id).unwrap();
        let entry = log.read(actor_id, actor_data.curr_log_index).await?;
        match entry {
            Some(entry) => {
                actor_data.curr_log_index = entry.next_idx;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

enum RuntimeError {
    StorageError(StorageError),
    Dispatch(DispatchError),
    Other(Box<dyn Any>),
}

impl<L> ActorData<L>
where
    L: Log,
{
    fn new(id: AnyActorId, actor: Box<dyn Any + RefUnwindSafe>, log: L) -> Self {
        Self {
            id,
            actor,
            log,
            curr_log_index: L::LogIndex::ZERO,
            cache: GlobalStorageCache::new(),
        }
    }

    fn run_init(&mut self, table: &DynTable, db: &Database) -> Result<Effects, RuntimeError> {
        let mut cx = AnyContext::new(self.id, db, &mut self.cache);
        catch_unwind_and_dispatch_errors(AssertUnwindSafe(|| {
            table.dispatch_init(self.id.name, &self.actor, cx)
        }))
    }

    fn run_handler(
        &mut self,
        table: &DynTable,
        message: AnyMessage,
        db: &Database,
    ) -> Result<Effects, RuntimeError>
    where
        L: Log,
    {
        let mut cx = AnyContext::new(self.id, db, &mut self.cache);
        catch_unwind_and_dispatch_errors(AssertUnwindSafe(|| {
            table.dispatch_handler(self.id.name, &self.actor, cx, message)
        }))
    }
}

fn catch_unwind_and_dispatch_errors<T, F>(f: F) -> Result<T, RuntimeError>
where
    F: FnOnce() -> DispatchResult<T> + UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        Ok(result) => result.map_err(RuntimeError::Dispatch),
        Err(err) => {
            let err = err
                .downcast::<StorageError>()
                .map(|err| RuntimeError::StorageError(*err))
                .unwrap_or_else(|err| RuntimeError::Other(err));
            Err(err)
        }
    }
}
