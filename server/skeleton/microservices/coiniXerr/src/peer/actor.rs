




use actix::prelude::*; //-- loading actix actors and handlers
use std::time::Duration;
use liby;



#[derive(Message)]
#[rtype(result = "()")]
struct Ping {
    pub id: usize,
}

// Actor definition
struct Miner {
    counter: usize,
    name: String,
    recipient: Recipient<Ping>,
}

impl Actor for Miner {
    type Context = Context<Miner>;
}

// simple message handler for Ping message
impl Handler<Ping> for Miner {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        self.counter += 1;

        if self.counter > 10 {
            System::current().stop();
        } else {
            println!("[{0}] Ping received {1}", self.name, msg.id);

            // wait 100 nanoseconds
            ctx.run_later(Duration::new(0, 100), move |act, _| {
                act.recipient.do_send(Ping { id: msg.id + 1 });
            });
        }
    }
}

pub async fn run() {
    let mut system = System::new();

    // To get a Recipient object, we need to use a different builder method
    // which will allow postponing actor creation
    let addr = system.block_on(async {
        Miner::create(|ctx| {
            // now we can get an address of the first actor and create the second actor
            let addr = ctx.address();

            let addr2 = Miner {
                counter: 0,
                name: String::from("Miner 2"),
                recipient: addr.recipient(),
            }
            .start();

            // let's start pings
            addr2.do_send(Ping { id: 10 });

            // now we can finally create first actor
            Miner {
                counter: 0,
                name: String::from("Miner 1"),
                recipient: addr2.recipient(),
            }
        });
    });

    system.run();
}