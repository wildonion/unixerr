






//// onion actor



// actors have:
//     - task scheduling algos
//     - worker threadpool like tokio::spawn()
//     - pub sub channels for broadcasting messages
//     - jobq like celery and the one inside the rabbitmq and zmq 





use crate::*;






#[derive(Clone)]
pub struct Message;


#[derive(Clone)]
pub struct Publish{ //// use to publish message to the built channel
    pub msg: Message,
    pub topic: String,
} 


#[derive(Clone)]
pub struct Subscribe{ //// use to subscribe to a specific topic which has published by another actor
    pub actor: Actor,
    pub topic: String,
} 


#[derive(Clone)]
pub struct Actor; // https://ryhl.io/blog/actors-with-tokio/
    
impl Actor{

    pub async fn schedule(){

        todo!() // ➔ schedule attack every 40 seconds after any error

    }


    pub async fn broadcast(){
        
        todo!() // ➔ use tokio::sync::broadcast
    
    }
    
    pub async fn run(){

        todo!()

    }

    pub async fn handle_message(){

        // use jobq.rs algos to put the incoming tasks and messages inside the queue
        // ...

        todo!()

    }

    pub async fn communicate(){ // each actor (neuron in uniXerr brain schema) can communicate with each other through some kinda channel (synapse)

        todo!() // ➔ use the jobqs algos the ones inside jobq.rs for communicating between neuron actors

    }


}