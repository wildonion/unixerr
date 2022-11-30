






//// futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by using .await on it also try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
//// StreamExt is a trait for streaming utf8 bytes data - RemoteHandle is a handler for future objects which are returned by the remote_handle() method
use futures::{StreamExt, FutureExt}; 
use std::env;
use uuid::Uuid;
use dotenv::dotenv;
use std::sync::Arc;
use log::{info, error, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;













#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //// bounding the type that is caused to error to Error, Send and Sync traits to be shareable between threads and have static lifetime across threads and awaits; Box is an smart pointer which has valid lifetime for what's inside of it, we're putting the error part of the Result inside the Box since we have no idea about the size of the error or the type that caused this error happened at compile time thus we have to take a reference to it but without defining a specific lifetime
    


    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                   env vars setup
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈     
    
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
                                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                                    .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
                                    .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    dotenv().expect("⚠️ .env file not found");
    let host = env::var("HOST").expect("⚠️ no host variable set");
    let port = env::var("RPC_PORT").expect("⚠️ no rpc port variable set");
    let rpc_addr = format!("{}{}", host, port).as_str();









    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
    ///////                   capnp rpc client
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    

    Ok(())




    
}