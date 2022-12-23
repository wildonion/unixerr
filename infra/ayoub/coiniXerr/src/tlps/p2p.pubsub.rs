







use crate::*;






//// the backbone of the libp2p is something like ZMQ with pub/sub 
//// socket connections each of which is an actor communicate 
//// with each other using a socket or an RPC channels.
//
//// each lip2p node is a ZMQ socket which is an actor with concepts of
//// worker threadpool (like tokio::spawn() green based worker threadpool + 
//// tokio channels for sharing messages and tasks between threads), job or task queue for 
//// task scheduling, pub/sub channels for broadcasting messages to other actors 
//// like socket, RPC or tokio like channels (if actors are in same machine) and mailbox 
//// to receive from other actor or outside of the actor system under the hood.
//
//// in distributed networks like the one we build with libp2p, every node or socket is a pub/sub actor 
//// which will communicate with each other through message passing protocols like ZMQ sockets or RPC channels.
//// since each node is an actor object with pre defined methods encoded with a distributed object protocol 
//// like Cap'n Proto RPC or Protobuf gRPC hence to communicate with other node or actors 
//// and call each other methods directly on different machines without proxying they must use pub/sub 
//// channels through RPC like the one in chatroom, file sharing, twitter push update notifications.  
//// by using Cap'n Proto or Protobuf as the object serialization both pub/sub actors knwo the exact 
//// structure of the realtime request/response streaming between them and if they are on 
//// the same machine they can use tokio channels like mpsc, watch, oneshot and broadcast to
//// share an encoded, Send and Sync (Arc<Mutex<T>>) data between tokio workers' threadpool.
//
//// in here we'll send all the decoded transactions 
//// to the downside of the mempool channel 
//// for mining and veifying process.
pub async fn bootstrap(
        storage: Option<Arc<Storage>>, 
        env_vars: HashMap<String, String>,
        current_slot: Slot, 
        validator_joined_channel: ChannelRef<ValidatorJoined>,
        default_parachain_uuid: Uuid,
        cloned_arc_mutex_runtime_info_object: Arc<Mutex<RafaelRt>>,
        meta_data_uuid: Uuid,
        cloned_arc_mutex_validator_update_channel: Arc<Mutex<ChannelRef<ValidatorUpdated>>>,
        cloned_arc_mutex_validator_actor: Arc<Mutex<ActorRef<ValidatorMsg>>>, //// the validator actor
        coiniXerr_sys: ActorSystem
    ){

    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    ///////           setting up libp2p pub/sub stream to broadcast actors' events to the whole networks
    /////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈
 
    // ----------------------------------------------------------------------
    //                          SERVICE VARS INITIALIZATION
    // ----------------------------------------------------------------------

    let buffer_size = env_vars.get("BUFFER_SIZE").unwrap().parse::<usize>().unwrap();
    let (mempool_sender, mempool_receiver) = *MEMPOOL_CHANNEL;




    // TODO - musiem file sharing
    // TODO - libp2p setup here
    // https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/#peer-to-peer-basics
    // https://blog.logrocket.com/libp2p-tutorial-build-a-peer-to-peer-app-in-rust/
    // https://github.com/libp2p/rust-libp2p/blob/f6f42968e21d6fa1defa0e4ba7392f1823ee055e/examples/file-sharing.rs
    // https://github.com/libp2p/rust-libp2p/blob/f6f42968e21d6fa1defa0e4ba7392f1823ee055e/examples/chat-tokio.rs
    // https://github.com/libp2p/rust-libp2p/blob/master/examples/gossipsub-chat.rs 
    // ...









}