use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//FROM: https://www.slingacademy.com/article/introduction-to-concurrency-in-rust-understanding-the-basics/

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
fn main() {
    let data = Arc::new(Mutex::new(0));

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

    println!("Shared number: {}", *data.lock().unwrap());

    move_example();
    channel_handle();
    move_take_ownership();
    move_string_btw_threads();
    multiple_messages();

    transit();

    if false {  //don't do it unless needed for following code to run...

        println!("Done before deadlock!");

        //for whatever reason, doesn't always deadlock but fairly consistent with the for loop otherwise too hard...
        for i in 0..5 {
            println!("deadlock {}", i);
            deadlock();
        }
    }

    counter_example();
    no_locks_with_channels();
    read_optimize();



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

fn multiple_messages() {
    let (tx, rx) = mpsc::channel();

    let counter = Arc::new(Mutex::new(0));

    let cnt = 5;

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
    //why is this needed to get for loop to finish?
    //because the original tx is never disposed/dropped, only the 5 cloned due to thread scope out of scope drop?
    //drop(tx);

    //THIS CAN NEVER FINISH because source tx is not dropped only the cloned tx's?
    //unless you explicitly drop or only take a many as sent!
    for received in rx.iter().take(cnt) {
        println!("Received: {}", received);
    }

    println!("Total Number of messages: {}", counter.lock().unwrap());

    println!("Ended multiple messages.");
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