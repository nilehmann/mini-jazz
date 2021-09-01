mod actor;
mod client_server;
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

use actor::PersistentActor;
use handler::Handler;
use log::DummyLog;
use message::Message;
use runtime::Runtime;
use serde::{Deserialize, Serialize};

struct Counter {
    i: u32,
}

impl PersistentActor for Counter {
    const NAME: &'static str = "Counter";

    fn init(&self, cx: &mut context::Context<Self>) {
        if self.i > 0 {
            cx.storage.put("i", self.i);
        }
        cx.dispatcher.send(cx.actor_id, Dec);
    }
}

#[derive(Serialize, Deserialize)]
struct Dec;

impl Message for Dec {
    const NAME: &'static str = "Dec";
}

impl Handler<Dec> for Counter {
    fn handle(&self, cx: &mut context::Context<Self>, _: Dec) {
        let counter: &mut u32 = cx.storage.borrow_mut("counter");
        if *counter > 0 {
            *counter -= 1;
            cx.dispatcher.send(cx.actor_id, Dec);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Inc {
    value: u32,
}

impl Message for Inc {
    const NAME: &'static str = "Inc";
}

impl Handler<Inc> for Counter {
    fn handle(&self, cx: &mut context::Context<Self>, Inc { value }: Inc) {
        let counter: &mut u32 = cx.storage.borrow_mut("counter");
        *counter -= value;
        cx.dispatcher.send(cx.actor_id, Inc { value: 1 });
    }
}

fn do_thing(mut runtime: Runtime<DummyLog>) {
    runtime.register_actor::<Counter>();
    runtime.register_handler::<Counter, Inc>();
    runtime.register_handler::<Counter, Dec>();

    runtime.add_actor(Counter { i: 0 }, DummyLog);

    runtime.run();
}

fn main() {}
