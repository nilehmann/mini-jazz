use actor::PersistentActor;

mod actor;
mod context;
mod database;
mod dispatcher;
mod dyn_table;
mod errors;
mod global_storage;
mod handler;
mod log;
mod message;
mod runtime;

struct Counter {
    i: u32,
}

impl PersistentActor for Counter {
    const NAME: &'static str = "Counter";

    fn init(&self, cx: &mut context::Context) {
        cx.storage.put("i", i);
    }
}

fn main() {}
