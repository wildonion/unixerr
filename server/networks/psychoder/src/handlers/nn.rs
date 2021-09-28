




pub mod cnn;
pub mod mlp;
pub mod graph;
pub mod transformers;
pub mod lstm;




// TODO - impl Send (for traits and types) and Sync (for references) for all sub types of the struct to move it between threads like db connections and AI models
// TODO - codec for streaming of binary data like mapping incoming utf8 bytes (&[u8]) into a strcut using enum or serde_json::from_slice
// TODO - training inputs using streaming concept like: while let Some(input_data) = batches.next().await{}
// TODO - train the models based on job queue channel protocols and multithreading concepts like: tokio::spawn(async move{model.train().await})
// TODO - write proc macros for all nn variants
// EXAMPLE - let network = model!(mlp_1(20) -> mlp_2(10) -> cnn(3, 16, 2, 5, 1) -> mlp_3(10))





// https://stackoverflow.com/questions/2490912/what-are-pinned-objects
// https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
// https://github.com/zupzup/warp-websockets-example
// https://github.com/tokio-rs/tokio/tree/master/examples
// https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
// https://danielkeep.github.io/tlborm/book/
// https://cetra3.github.io/blog/implementing-a-jobq/
// https://cetra3.github.io/blog/implementing-a-jobq-with-tokio/
// https://docs.rs/tokio/1.7.1/tokio/sync/index.html
// https://docs.rs/tokio-stream/0.1.7/tokio_stream/
// https://doc.rust-lang.org/std/pin/index.html
// https://doc.rust-lang.org/std/sync/struct.Arc.html
// https://doc.rust-lang.org/std/sync/struct.Mutex.html
// https://doc.rust-lang.org/std/sync/struct.RwLock.html
// https://doc.rust-lang.org/std/cell/struct.RefMut.html
// https://doc.rust-lang.org/std/cell/struct.RefCell.html
// https://doc.rust-lang.org/std/rc/struct.Weak.html
// https://doc.rust-lang.org/std/rc/struct.Rc.html
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1#file-mactrait-rs
// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1#file-jobq-rs
// https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1#file-mpsc-rs
// http://gradebot.org/doc/ipur/trait.html
// https://pytorch.org
// https://cheats.rs/#behind-the-scenes
// https://github.com/LaurentMazare/tch-rs




