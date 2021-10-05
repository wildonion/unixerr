


/*


                                                WOLFIA FEATURES

        https://medium.com/@chataks93/predicting-human-behaviour-activity-using-deep-learning-lstm-fff9030b82e7
        https://www.biorxiv.org/content/biorxiv/early/2017/12/30/240317.full.pdf
        https://www.youtube.com/watch?v=YrO1v7-KcXs



        [!!!] audio streaming like clubhouse with low latency and bandwidth
        [!!!] voice recognition for judging graph of each game
        [!!!] credit score generation using position clustering and voice recognition => use core/coin_generation algos 
        [!!!] player actions and talks inside the game for generating the credit
        [!!!] realtime status diagram inside the game
        [!!!] playing based on the level of each player and mined credit
        [!!!] implement my master thesis idea for online video streaming
        [!!!] a game based on MIND BEHAVIOUR ACTIVITY to deal with their fears! like black mirrors S03E02.


*/



use serde::{Serialize, Deserialize};
use uuid::Uuid;


pub trait Synapse{
    fn communicate() -> Self;
}



pub struct Neuron; //-- unit like struct


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaData{
    pub id: Uuid,
    pub neuron_name: String,
    pub time: i64,
}




impl Synapse for Neuron{ //-- it's like implementing a behaviour for a raw object without any meta data
    fn communicate() -> Self{ //-- this is not object safe trait cause it's returning an associated type which is Self
        Neuron
    }
}
impl Default for Neuron{
    fn default() -> Self{
        todo!()
    }
}



impl Default for MetaData{
    fn default() -> Self{
        MetaData{
            id: Uuid::new_v4(),
            neuron_name: "AJG7$%".to_string(),
            time: chrono::Local::now().naive_local().timestamp(),
        }
    }
}