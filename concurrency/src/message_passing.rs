 // One increasingly popular approach to ensuring safe concurrency is message passing,
 // where threads or actors communicate by sending each other messages containing data.
 // Here’s the idea in a slogan from the Go language documentation:
 //
 // “Do not communicate by sharing memory; instead, share memory by communicating.”
 //
 // To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels.
 // A channel is a general programming concept by which data is sent from one thread to another.
 // A channel has two halves: a transmitter and a receiver.
 // The transmitter half is the upstream location where you put rubber ducks into the river,
 // and the receiver half is where the rubber duck ends up downstream.
 // One part of your code calls methods on the transmitter with the data you want to send,
 // and another part checks the receiving end for arriving messages.
 // A channel is said to be closed if either the transmitter or receiver half is dropped.

 use std::sync::mpsc;
 use std::thread;
 use std::time::Duration;

 pub fn mpsc_channel() {
     println!();
     println!("Using Message Passing to Transfer Data Between Threads");

     // Again, we’re using thread::spawn to create a new thread and then using move to move tx into the closure so the spawned thread owns tx.
     // The spawned thread needs to own the transmitter to be able to send messages through the channel.
     // The transmitter has a send method that takes the value we want to send.
     // The send method returns a Result<T, E> type, so if the receiver has already been dropped
     // and there’s nowhere to send a value, the send operation will return an error.
     // In this example, we’re calling unwrap to panic in case of an error.
     let (tx, rx) = mpsc::channel();
     thread::spawn(move || {
       let val = String::from("Hello");
         println!("Send: {}", val);
         tx.send(val).unwrap();

         // println!("Send: {}", val); throws: value borrowed here after move b/c ownership has been transferred to send
         // Our concurrency mistake has caused a compile time error.
         // The send function takes ownership of its parameter, and when the value is moved,
         // the receiver takes ownership of it.
         // This stops us from accidentally using the value again after sending it;
         // the ownership system checks that everything is okay.

     });

     // The receiver has two useful methods: recv and try_recv.
     // We’re using recv, short for receive, which will block the main thread’s execution
     // and wait until a value is sent down the channel.
     // Once a value is sent, recv will return it in a Result<T, E>.
     // When the transmitter closes, recv will return an error to signal that no more values will be coming.
     let received = rx.recv().unwrap();
     println!("Got: {:?}", received);

     //The try_recv method doesn’t block, but will instead return a Result<T, E> immediately:
     // an Ok value holding a message if one is available and an Err value if there aren’t any messages this time.
 }


 pub fn mpsc_multi_val_channel() {
     println!();
     println!("Sending Multiple Values and Seeing the Receiver Waiting");

     let (tx, rx) = mpsc::channel();

     // here we are sending a series of values, each as a message
     thread::spawn(move || {
         let vals = vec![
             String::from("hi"),
             String::from("from"),
             String::from("the"),
             String::from("thread"),
         ];
         for val in vals {
             tx.send(val).unwrap();
             thread::sleep(Duration::from_millis(10));
         }
     });

     for received in rx{
         println!("Got: {}", received);
     }

 }


 pub fn mpsc_multi_producers_single_consumer() {
     println!();
     println!("Creating Multiple Producers by Cloning the Transmitter");

     let (tx, rx) = mpsc::channel();
     let tx1 = tx.clone(); // clone the transmitter before move to the processing thread


     // start first producer thread using first sender
     thread::spawn(move || {
         let vals = vec![
             String::from("hi"),
             String::from("from"),
             String::from("the"),
             String::from("thread"),
         ];
         for val in vals {
             tx.send(val).unwrap();
             thread::sleep(Duration::from_millis(10));
         }
     });

     // start second producer thread using second sender
     thread::spawn(move || {
         let vals = vec![
             String::from("more"),
             String::from("messages"),
             String::from("for"),
             String::from("you"),
         ];
         for val in vals {
             tx1.send(val).unwrap();
             thread::sleep(Duration::from_millis(10));
         }
     });

     for received in rx{
         println!("Got: {}", received);
     }

 }

