use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

use std::time::Instant;

use tokio::runtime::Runtime;
use tokio::runtime::Builder;
use tokio::task;

//disabled for now see ~line 472
//use futures::future::{join, join_all};
use futures::future::{join_all};
use futures::Future;
use std::pin::Pin;

use std::io::{ stdout, Write};

//Originally FROM: https://www.slingacademy.com/article/introduction-to-concurrency-in-rust-understanding-the-basics/
//but later added info from other sources..

/*
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Hello from the spawned thread! Counting up: {}", i);
        }
    });

    handle.join().unwrap();
    println!("Thread has finished execution.");
}
*/

//Rust provides safe abstractions such as Mutex and Arc for shared ownership and mutation across threads.

//The Mutex (mutual exclusion) ensures that only one thread can access the data at a time. The Arc (atomic reference counting) enables multiple owners of the same piece of data. Here's an example:

//In this code, Arc (Atomic Reference Counting) is used to allow multiple ownership over a single piece of data. Combine it with Mutex to ensure that only one thread accesses the data at a time, preventing data races.
//can't be an async func with tokio because can't have nested runtimes...
fn main() {
    println_header("Running: run_threaded_code()");
    run_threaded_code();

    let data = Arc::new(Mutex::new(0));

    println_header("Running: looped map with incremented value()");

    let handles: Vec<_> = (0..10).map(|i| {
        println!("Hello from range map! {}", i);
        //The Arc::clone(&data) allows each thread to have a reference-counted pointer to the data.
        let data = Arc::clone(&data);
        thread::spawn(move || {
            //Within each thread, we lock the Mutex using data.lock().unwrap() to gain safe access and mutate the value.
            let mut num = data.lock().unwrap();
            *num += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    //Thus, the short version is: the result of locking a mutex is a "smart pointer" wrapping the actual value, not the value itself. Thus you have to dereference it to access the value. https://stackoverflow.com/questions/46314582/confusing-automatic-dereferencing-of-arc

    println_header(format!("Shared number: {}", *data.lock().unwrap()).as_str());

    println_header("Running: move_example()");
    move_example();
    println_header("Running: channel_handle()");
    channel_handle();
    println_header("Running: move_take_ownership()");
    move_take_ownership();
    println_header("Running: move_string_btw_threads()");
    move_string_btw_threads();
    println_header("Running: multiple_messages()");
    multiple_messages();

    println_header("Running tokio_multiple_tasks");
    tokio_multiple_tasks();

    println_header("Running: transit()");
    transit();

    if false {  //don't do it unless needed for following code to run...

        println!("Done before deadlock!");

        //for whatever reason, doesn't always deadlock but fairly consistent with the for loop otherwise too hard...
        for i in 0..5 {
            println!("deadlock {}", i);
            deadlock();
        }
    }

    println_header("Running: counter_example()");
    counter_example();
    println_header("Running: no_locks_with_channels()");
    no_locks_with_channels();
    println_header("Running: read_optimize()");
    read_optimize();


    println_header("Running: Sequential and Parallel sum of squares");
    //sequential vs parallel    await

    let now = Instant::now();

    let numbers: Vec<u128> = (1..10_000_000).collect();
    let result = sum_of_squares(&numbers);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Sequential sum of squares: {}", result);


    let now = Instant::now();

    let numbers: Vec<u128> = (1..10_000_000).collect();
    let result = parallel_sum_of_squares(&numbers);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("parallel sum of squares: {}", result);

    println_header("Running: tokio_runtime_block_on_example()");
    tokio_runtime_block_on_example();
    println_header("Running: tokio_run_blocking_examples()");
    tokio_run_blocking_examples();
    println_header("Running: tokio_sequential_tasks()");
    tokio_sequential_tasks();
    println_header("Running: combined_task()");
    combined_task();

    println_header("Running: futures executor block_on()");
    futures::executor::block_on( handle_async() );

    println_header("Running: Tokio spawn stuff");


    //note delay in between to show no work done since pending await
    tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap().block_on( async {
        say_hello().await;
        // An async sleep function, puts the current task to sleep for 1s.
        tokio::time::sleep(Duration::from_millis(1000)).await;
        say_world().await;
    });


        // note you should see no delay but in different orders!
        tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap().block_on( async {
        tokio::spawn(say_hello());
        tokio::spawn(say_world());
        // Wait for a while to give the tasks time to run.
        tokio::time::sleep(Duration::from_millis(300)).await;

        for _ in 0..15 {
            let handle1 = tokio::spawn(say_hello());
            let handle2 = tokio::spawn(say_world());
            
            //note what happens if you comment these out...  no hello worlds!
            let _ = handle1.await;
            let _ = handle2.await;
        
            println!("!");
        }

        });
}

fn println_header(header: &str) {
    let length: usize = header.len(); 
    let str = "*".repeat(length + 4);
    println!("\n{str}\n* {header} *\n{str}\n");
}
#[allow(unused)]
fn run_threaded_code() {
    //what happens?
    let mut n = 1;

    let t = thread::spawn(move || {

        n = n + 1;

        thread::spawn(move || {
            n = n + 1;
        })

    });

    n = n + 1;

    t.join().unwrap().join().unwrap();

    //n has copy trait so the threads move a copy of n which mutate the copy and lost 
    println!("threaded result multiple increments: {n}");
}

//a vector is moved into a thread using the move keyword, transferring its ownership to the thread, thus avoiding data races.
fn move_example() {
    let v = vec![1, 2, 3];

    let handle = std::thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();
}

//lock free messages are sent between threads using mpsc::channel, making it a safe and efficient way to implement producer-consumer patterns without shared state.
fn channel_handle() {
    let (tx, rx) = mpsc::channel();


    let handle = thread::spawn(move || {
        let val = "hi";
        tx.send(val).unwrap();
    });

    println!("Received: {}", rx.recv().unwrap());

    handle.join().unwrap();
}
//Notice the use of the move keyword, which allows the spawned thread to take ownership of the variables used in its environment. This is essential to avoid data races or borrowing issues.
fn move_take_ownership() {
    let text_to_print = "Printing from the new thread!";

    let handle = thread::spawn(move || {
        println!("{}", text_to_print);
        panic!("Foo Bar!");
    });

    //handle.join().unwrap();
    let result = handle.join();

    //The join method returns a Result which can be Ok if the thread completes successfully or Err if it panics, allowing for proper error handling.
    match &result {
        Ok(_) => println!("Thread finished without panic."),
        Err(e) => println!("Thread had an error: {:?}", e),
    }

    println!("Unwrapped Error: {:?}", result.err().unwrap() );
}

fn move_string_btw_threads() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("foo");
        tx.send(val).unwrap();

/*
    |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
103 |         tx.send(val).unwrap();
    |                 --- value moved here
104 |         println!("val is {val}");
    |                          ^^^^^ value borrowed here after move
        println!("val is {val}");
*/
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

//do the next two functions work only because there is code that runs after it?
//if they were the only things in main would the thread end early and kill the rest of processing?

fn multiple_messages() {
    let (tx, rx) = mpsc::channel();

    let counter = Arc::new(Mutex::new(0));

    let cnt = 5;

//odd the recieve messages are received in sequential order?
//note next example below is not sequential  

    for i in 0..cnt {
        let tx = tx.clone();
        let counter = Arc::clone(&counter);
        println!("Sending message: {}", i);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            let message = format!("Message {}", i);
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(1000));
            println!("Ended sending message: {}", i);
        });
    }
    /*
    why is this needed to get for loop to finish?
    because the original tx is never disposed/dropped, only the 5 cloned due to thread scope out of scope drop?
    drop(tx);
    */

    //THIS CAN NEVER FINISH because source tx is not dropped only the cloned tx's?
    //unless you explicitly drop or only take a many as sent!
    for received in rx.iter().take(cnt) {
        println!("Received: {}", received);
    }

    println!("Total Number of messages: {}", counter.lock().unwrap());

    println!("Ended multiple messages.");
}


fn tokio_multiple_tasks() {

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    let (tx, rx) = mpsc::channel();

    let counter = Arc::new(tokio::sync::Mutex::new(0));

    let cnt = 5;

//odd the received message are not all sequential?

    for i in 0..cnt {
        let tx = tx.clone();
        let counter = Arc::clone(&counter);
        println!("Task Sending message: {}", i);
        rt.spawn( async move {
            let mut num = counter.lock().await;
            *num += 1;
            let message = format!("Message {}", i);
            tx.send(message).unwrap();
            tokio::time::sleep(Duration::from_millis(1000)).await;
            println!("Ended task sending message: {}", i);
        });
    }


    /*
    why is this needed to get for loop to finish?
    because the original tx is never disposed/dropped, only the 5 cloned due to thread scope out of scope drop?
    drop(tx);
    */

    //THIS CAN NEVER FINISH because source tx is not dropped only the cloned tx's?
    //unless you explicitly drop or only take a many as sent!
    for received in rx.iter().take(cnt) {
        println!("Task Received: {}", received);
    }

    println!("Total Task Number of messages: {}", counter.blocking_lock());

    println!("Ended multiple task messages.");

}

fn transit() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }); 

    for received in rx {
        println!("Got: {}", received);
    }
}

fn deadlock() {
    println!("Start deadlock!");
    let lock1 = Arc::new(Mutex::new(1));
    let lock2 = Arc::new(Mutex::new(2));

    // Clone the Arc pointers to share ownership between threads
    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);

    let handle1 = thread::spawn(move || {
        let _guard1 = lock1_clone.lock().unwrap();
        // Force context switch
        thread::yield_now();
        let _guard2 = lock2_clone.lock().unwrap();
    });

    let handle2 = thread::spawn(move || {
        let _guard2 = lock2.lock().unwrap();
        // Force context switch
        thread::yield_now();
        let _guard1 = lock1.lock().unwrap();
    });

    thread::sleep(Duration::from_secs(1));

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("End deadlock!");
}

fn counter_example() {
    //simple arc and mutex example
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || { 
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn no_locks_with_channels() {
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i).unwrap();
        });
    }

    //fails to finish unless there is a break
    for _ in 0..5 {
    //definately doesn't complete linked to rx.recv?
    //for i in rx.iter() {
        //println!("rx receive index: {}", i);
        let received = rx.recv().unwrap();
        println!("Received: {}", received);
    }
}

//this is used to delay so threading read/writes will overlap 
fn sleep(timems: u64)
{
            let timems = rand::random_range(0..timems);
            println!("sleep time ms is : {}", timems);
            //sleep between 0 and 20 seconds so there is read / write overlap
            thread::sleep(Duration::from_millis(timems));
}

fn read_optimize() {
    let lock = Arc::new(RwLock::new(5));

    // Create 10 reader threads
    let handles: Vec<_> = (0..10).map(|_| {
        let lock = Arc::clone(&lock);
        thread::spawn(move || {
            sleep(150);
            let r = lock.read().unwrap();
            println!("Reader got: {}", *r);
        })
    }).collect();

    // Single writer thread
    {
        sleep(75);
        println!("Locking to write!");
        let mut w = lock.write().unwrap();
        *w += 1;
    }

    // Join reader threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *lock.read().unwrap());
}


fn sum_of_squares(numbers: &[u128]) -> u128 {
    numbers.iter().map(|&x| x * x).sum()
}

fn parallel_sum_of_squares(numbers: &[u128]) -> u128 {
    numbers.par_iter().map(|&x| x * x).sum()
}

//ran cargo add tokio --features full

fn tokio_runtime_block_on_example() {
    let rt = Runtime::new().unwrap();
    thread::spawn(move || {
        rt.block_on(async {
            let now = Instant::now();
            // Simulate a long blocking operation
            println!("Blocking task started");
            let _result = long_computation().await;
            println!("Blocking task completed");
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
        });
    }).join().unwrap();
}

async fn long_computation() -> i32 {
    // Emulates a long computation
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    10
}

async fn main_task() {
    println!("Main task started");
    let result = tokio::join!(quick_task(), quick_task());
    println!("Main task result: {:?}", result);
}

async fn quick_task() -> &'static str {
    println!("Quick task running");
    // A short non-blocking delay
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("Done running quick task");
    "Done"
}

/*
when some operations block inherently

The spawn_blocking utility allows you to run blocking operations on a specialized thread which is different from the async task's thread. This technique ensures the task does not block the async runtime's main loop.

*/

async fn process_io_bound() {
    let result = task::spawn_blocking(|| {
        println!("Starting blocking I/O");
        // Long blocking I/O operation, e.g., file processing
        std::thread::sleep(std::time::Duration::from_secs(5));
        "Finished processing I/O"
    }).await.expect("The task failed to complete");
    println!("{}", result);
}

fn tokio_run_blocking_examples() {
    println!("Tokio run blocking thread/IO");
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(main_task());

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(process_io_bound());
    println!("Done with Tokio run blocking thread/IO");
}

//note sequence and where starting output for tasks are shown vs in between message!

fn tokio_sequential_tasks() {

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    let result1 = rt.spawn(quick_task());
    println!("In between seq tasks!");
    let result2 = rt.spawn(quick_task());

    let result1 = rt.block_on(result1);
    let result2 = rt.block_on(result2);

    println!("Results: {} and {}", result1.unwrap(), result2.unwrap());

}

fn combined_task() {

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    let (result1, result2) = futures::executor::block_on(
                                                            rt.spawn( async {
                                                                tokio::join!(quick_task(), quick_task())
                                                                                    } 
                                                                    ) 
                                                        ).unwrap();

    println!("Fetched data: {} and {}", result1, result2);
}

//from:  https://www.slingacademy.com/article/pinning-and-the-future-trait-in-async-rust/
//but not working...

//not working because async blocks are each different.   need to pin as type

//Need to pin them?  https://stackoverflow.com/questions/71070434/expected-async-block-found-a-different-async-block

/*
error[E0308]: mismatched types
   --> src/main.rs:487:36
    |
476 |     let a = async { 42 };
    |             ----- the expected `async` block
...
484 |     let b = async { 43 };
    |             ----- the found `async` block
...
487 |     let results = join_all(vec![a, b, c]).await;
    |                                    ^ expected `async` block, found a different `async` block
    |
    = note: expected `async` block `{async block@src/main.rs:476:13: 476:18}`
               found `async` block `{async block@src/main.rs:484:13: 484:18}`
    = note: no two async blocks, even if identical, have the same type
    = help: consider pinning your async block and casting it to a trait object

For more information about this error, try `rustc --explain E0308`.
*/

async fn handle_async() {
    let a = async { 42 };
    /*
    let b = async { 
        //tokio::time::sleep(std::time::Duration::from_secs(2)).await; 
        //return 43;
        43
    };
    */
    let b = async { 43 };
    let c = async { 44 };


let futures: [Pin<Box<dyn Future<Output = i32>>>; 3] = [
        Box::pin(a),
        Box::pin(b),
        Box::pin(c),
    ];

    let results = join_all(futures).await;

    for result in results {
        println!("Result: {result:?}");
    }
}

async fn say_hello() {
    print!("hello, ");
    // Flush stdout so we see the effect of the above `print` immediately.
    stdout().flush().unwrap();
}

async fn say_world() {
    println!("world!");
        // Flush stdout so we see the effect of the above `print` immediately.
        stdout().flush().unwrap();
}
