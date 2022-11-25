




// example command of training an async and multithreading version of mlp network
// CLI COMMAND : ./psychoder --network mlp --dataset /path/to/dataset --device cpu --epochs 200 --batch-size 64 --output-neurons 10



use log::info;
use std::thread;
use utilser::Info;
use crate::schemas::brain::Neuron;
use actix::prelude::*;
use std::sync::{Arc, mpsc::channel};
use futures::{executor::block_on, join, StreamExt, TryStreamExt};
use rayon::prelude::*;
use crate::schemas::brain::Synapse; //-- based on the orphan rule this should be used in here cause the communication() method of each neuron is implemented inside the Synapse trait - items from traits can only be used if the trait is in scope






impl Info for Neuron{
    fn what(&self) -> String{
        format!("Neuron [{}] fired at time [{}] with id [{}]", self.timestamp, self.name.to_string(), self.id.to_string())
    }
}


pub struct Linear{
    pub neural_circuit: Vec<Neuron>, //-- neural circuit is a collection of connected neurons which are our features or columns of our input data matrix and form the whole matrix of the data input data
}

impl Linear{

    pub async fn forward(&self, x_train: Arc<Vec<Vec<f64>>>) -> f64{ //-- without &mut self would be an associated function not a method
        
        
        ////////////////////////////////// multi threading ops
        let thread = thread::spawn(|| async move{ //-- the body of the closure is an async block means it'll return a future object (trait Future has implemented for that) for with type either () or a especific type
            info!("inside the native thread");
            let async_task = tokio::spawn(async move{ //-- spawning async task to solve it on the background using tokio green threads based on its event loop model - 
                info!("inside tokio green thread");
                ////////
                // ....
                ////////
           });
        });
        //////////////////////////////////
        

        let mut linear_neural_circuit = self.neural_circuit.iter();
        linear_neural_circuit.next().unwrap().communicate(linear_neural_circuit.next()); //-- communicate method through synapse trait
        let mat = x_train;
        let NTHREADS: usize = 4; // number of threads inside the pool
        let NJOBS: usize = mat.len(); // number of tasks of the process (incoming x_train matrix) to share each one between threads inside the pool
        let (sender, receiver) = channel();
        let arc_mat = Arc::new(mat);
        let arc_recv = Arc::new(&receiver); //-- take a reference to the receiver to borrow it for putting it inside an Arc
        let mut mult_of_all_sum_cols = 1.0; 
        let future_res = async { //-- we can also use tokio::spawn() to run the async task in the background using tokio event loop and green threads
            for i in 0..NJOBS{ //-- iterating through all the jobs of the process - this can be an infinite loop like waiting for a tcp connection
                let cloned_receiver = Arc::clone(&arc_recv); // can't clone receiver, in order to move it between threads we have to clone it using Arc
                let cloned_sender = sender.clone(); // NOTE - sender can be cloned because it's multiple producer
                let cloned_mat = Arc::clone(&arc_mat);
                rayon::spawn(move || { // NOTE - rayon::spawn() contains a threadpool and will spawn threads or workers to solve the incoming job inside a free thread
                    let sum_cols = cloned_mat[0][i] + cloned_mat[1][i] + cloned_mat[2][i];
                    cloned_sender.send(sum_cols).unwrap();
                });
                println!("job {} finished!", i);
            }
            // NOTE - recv() will block the current thread if there are no messages available
            // NOTE - receiver can't be cloned cause it's single consumer
            let ids: Vec<f64> = receiver.iter().take(NJOBS).collect();
            println!("the order that all messages were sent => {:?}", ids);
            ids.into_iter().map(|s_cols| mult_of_all_sum_cols *= s_cols).collect::<Vec<_>>();
            mult_of_all_sum_cols
        };
        let res = block_on(future_res); //-- will block the current thread to run the future to completion
        // let res = future_res.await; //-- .awaiting a future will suspend the current function's execution until the executor has run the future to completion means doesn't block the current thread, allowing other tasks to run if the future is currently unable to make progress
        // let res = join!(future_res); //-- join! only allowed inside `async` functions and blocks and is like .await but can wait for multiple futures concurrently
        println!("multiplication cols sum {:?}", res);
        let loss = 0.3535;
        loss

        
    }

    pub async fn backward(&self, loss: f64){
        //-- without &mut self would be an associated function not a method
        // TODO - 
        // ...
    }
}


impl Actor for Linear{
    type Context = Context<Linear>;
}