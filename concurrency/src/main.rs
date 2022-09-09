use thread::*;
use message_passing::*;
use state::*;

mod thread;
mod message_passing;
mod state;

fn main() {
    // new_base_thread();
    // thread_with_join();

    thread_with_join_and_move();
    mpsc_channel();
    mpsc_multi_val_channel();
    mpsc_multi_producers_single_consumer();

    basic_mutex();
    shared_mutex();


}
