use serde::{Deserialize, Serialize};

use crate::{
    actor::{PersistentActor, PersistentActorId},
    context::Context,
    dispatcher::{Callback, CallbackId},
    handler::Handler,
    message::Message,
};

struct Client {
    server_id: PersistentActorId<Server>,
    i: u32,
    n: u32,
}

impl PersistentActor for Client {
    const NAME: &'static str = "Client";

    fn init(&self, cx: &mut Context<Self>) {
        if self.i > 1 {
            cx.storage.put("counter", self.i - 1);
            let callback = cx.dispatcher.create_callback::<Self, ServerResponse>();
            cx.dispatcher.send(
                self.server_id,
                Double {
                    callback,
                    n: self.n,
                },
            );
        }
    }
}

struct ServerResponse {
    n: u32,
}

impl Callback<ServerResponse> for Client {
    type Env = ();

    fn handle(&self, cx: &mut Context<Self>, env: (), msg: ServerResponse) {
        let counter: &mut u32 = cx.storage.borrow_mut("counter");
        if *counter > 1 {
            *counter = *counter - 1;
            let callback = cx.dispatcher.create_callback::<Self, ServerResponse>();
            cx.dispatcher.send(
                self.server_id,
                Double {
                    callback,
                    n: *counter,
                },
            );
        }
    }
}

struct Server;

impl PersistentActor for Server {
    const NAME: &'static str = "Server";
}

#[derive(Serialize, Deserialize)]
struct Double {
    n: u32,
    callback: CallbackId<ServerResponse>,
}

impl Message for Double {
    const NAME: &'static str = "Message";
}

impl Handler<Double> for Server {
    fn handle(&self, cx: &mut Context<Self>, Double { callback, n }: Double) {
        callback.call(ServerResponse { n: 2 * n });
    }
}

/*
struct Client;

#[actor];
impl PersistentActor for Client { }

#[derive(Message)]
struct ServerResponse;

#[callback(Client)]
fn server_response(cx: &mut Context<Self>, msg: ServerResponse) {
    let counter: &mut u32 = cx.storage.borrow_mut("counter");
    if *counter > 1 {
        *counter = *counter - 1;
        let callback = callback!(cx, server_response);

    }
}


*/
