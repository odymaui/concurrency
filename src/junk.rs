/* 

this has duplicate code but leaving for now
todo: cleanup and remove junk and duplicate stuff...

*/

use rand::Rng;

use serde::{Serialize, Deserialize};

use std::{path::PathBuf, time::Duration};
use std::{thread};

use env_logger::{Builder};
use log::LevelFilter;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}


async fn test() -> bool {
    let sleep1 = sleep(5);
    let task2 = async {
        let _sleep2 = sleep(1).await;
        let sleep_arr = vec![sleep(2), sleep(2)];
        futures::future::join_all(sleep_arr).await;
    };
    futures::join!(sleep1, task2);
    return true;
}#[warn(unused_must_use)]

async fn sleep(sec: u64) -> () {
    tokio::time::sleep(std::time::Duration::from_secs(sec)).await;
}

mod foo {
    #[derive(Debug)]    
    pub struct Bar {
        pub foo: String,
        bar: String,
    }
    
    
    impl Bar {
        pub fn new(foo: String, bar: String) -> Self{
            Bar {
                foo: foo,
                bar: bar,
            }
        } 
        
        
    }
    
    pub fn get_bar(bar: &Bar) -> &String {
        &bar.bar
    } 
}

const _CONST_ITEM: u32 = 2;

macro_rules! crate_name_util { 

    (@as_foo $e:expr) => {$e + 5}; 

    (@as_bar $e:expr) => {$e + 10}; 

    //(@as_barbaz $i:item) => {$i}; 

    (@as_baz) => {0usize}; 

    () => {"Okay default applied"}; 

    // ... 

} 

macro_rules! call_with_larch {
    ($callback:ident) => { $callback!(larch) };
}

macro_rules! expand_to_larch {
    () => { 
    //larch
        println!("In Expand To Larch!");
        recognise_tree!(larch);
    };

}

macro_rules! recognise_tree {
    (larch) => { println!("#1, the Larch.") };
    (redwood) => { println!("#2, the Mighty Redwood.") };
    (fir) => { println!("#3, the Fir.") };
    (chestnut) => { println!("#4, the Horse Chestnut.") };
    (pine) => { println!("#5, the Scots Pine.") };
    (expand_to_larch) => { println!("Expand To Larch!") };
    () => { println!("Nothing Found!") };
    (other:tt $(,$other_other:tt)*) => {
        println!("I don't know; some kind of birch maybe?, {:?}", $other);
        recognise_tree($other_other); 
        
    };
}



/// Unique identifier for tensors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TensorId(usize);

impl TensorId {
    fn new() -> Self {
        // https://users.rust-lang.org/t/idiomatic-rust-way-to-generate-unique-id/33805
        use std::sync::atomic;
        static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(100);
        Self(COUNTER.fetch_add(1, atomic::Ordering::Relaxed))
    }
}

/*
use futures::future::{join, join_all};

async fn handle_async() {
    let a = async { 42 };
    let b = async { 43 };
    let c = async { 44 };

    let results = join_all(vec![a, b, c]).await;

    for result in results {
        println!("Result: {result}");
    }
}
*/

async fn some_function(input_arg: Option<String>) {
    if let Some(input) = input_arg {
    println!("Some: {}", input);
    } else {
        println!("Nothing, nodda");
        return;
    }
}

//will fail if in a module due to default of private in a mod
//pub mod foobarbaz {
 //   use std::{path::PathBuf, time::Duration};
    // note that we can simply auto-derive Default here.
    #[derive(Default, Debug, PartialEq)]
    pub struct MyConfiguration {
        // Option defaults to None
        output: Option<PathBuf>,
        // Vecs default to empty vector
        search_path: Vec<PathBuf>,
        // Duration defaults to zero time
        timeout: Duration,
        // bool defaults to false
        check: bool,
    }

//}

fn demo1() {

  let x = &mut 1u8;
  // Create several shared references, and we can also still read from `x`
  let y1 = &*x;
  let _val = *x;
  let y2 = &*x;
  let _val = *y1;
  let _val = *y2;
}

use std::sync::Mutex;

struct ThreadSafeInt {
    value: Mutex<i32>,
}

impl ThreadSafeInt {
    fn new(val: i32) -> Self {
        Self {
            value: Mutex::new(val),
        }
    }
    fn add(&self, delta: i32) {
        let mut v = self.value.lock().unwrap();
        *v += delta;
    }
}

impl ThreadSafeInt {
    fn add_with_extras(&self, delta: i32) {
        // ... more code here that doesn't need the lock
        
        //maintain a copy of the int for this method
        let mut v;
        //scope and unlock the wrapped value
        {
            v = *self.value.lock().unwrap();
        }
        
        println!("Before mut -> {v}");
        
        //now seperately get again and update the source value
        //as well as the local copy of the int
        {
            let mut vv = self.value.lock().unwrap();
            *vv += delta;
            v = *vv;
            
            println!("vv: {vv}");
        }
        // ... more code here that doesn't need the lock
        
        //println!("After mut -> {}", v);
        println!("After mut -> {}", v);
    }
    
    fn printit(&self) {
        //get the wrapped value
        println!("Printit -> {}", *self.value.lock().unwrap());
    }
}

struct AppData {
    count: u32,
}


struct ToDrop {idx: i32}

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop Idx: {} is being dropped", self.idx);
    }
}

impl ToDrop {
    fn new(idx: i32) -> Self {
        ToDrop {
            idx: idx
        }
    }
}

struct Number {
    value: usize,
}


trait AddOne {
    type SelfType;

    fn add_one(&self) -> Self::SelfType;
    fn value(&self) -> Self::SelfType;
}

impl AddOne for Number {
    type SelfType = usize;
    
    fn add_one(&self) -> Self::SelfType {
        self.value + 1
    }
    
    fn value(&self) -> Self::SelfType {
        self.value
    }
}

struct NumRef<'a> {
    x: &'a i32,
}

fn as_num_ref(x: &i32) -> NumRef<'_> {
    NumRef { x: &x }
}

impl<'a> NumRef<'a> {
    fn as_i32_ref(&'a self) -> &'a i32 {
        self.x
    }
}

pub async fn junk_main() {

    let x: i32 = 99;
    let _x_ref = as_num_ref(&x);
    // `x_ref` cannot outlive `x`, etc.
    let x_num_ref = NumRef { x: &x };
    let _x_i32_ref = x_num_ref.as_i32_ref();
    // neither ref can outlive `x`

  let num = Number { value: 4 };
  
  num.add_one();
  
  println!("Private Value is: {} : update: {}", num.value(), num.add_one());

  let result2see = vec![1, 2, 3, 4, 5, 6, 7, 8]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y);
    
    println!("{}", result2see);

    let mut data = AppData {
      count: 0
    };
    
    let fx = |data: &mut AppData| data.count +=1;

    fx(&mut data);
    fx(&mut data);
    fx(&mut data);

    
    println!("Count: {}", data.count);

    let _x = ToDrop { idx: 99 };
    println!("Made a ToDrop!");

    for idx in 0..10 {
        let _x = ToDrop::new(idx);
        println!("Idx: {} To Drop Is Being create in a loop",idx);
    }
    
    //can manually drop whenever you want with this:
    //drop(x);
    
        // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    
demo1();

let v = ThreadSafeInt::new(7);
v.add_with_extras(7);
v.printit();
v.add(5);
v.printit();


//use foobarbaz::MyConfiguration;

//    Builder::new()
//        .target(Target::Stdout)
//        .init();


    //Builder::from_env("MY_APP_LOG").init();

    //Builder::from_default_env().filter_level(LevelFilter::max()).init();
Builder::from_default_env().filter(None, LevelFilter::Info).init();


    log::error!("This error has been printed to Stdout");

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");


    // construct a new instance with default values
    let mut conf = MyConfiguration::default();
    // do something with conf here
    conf.check = true;
    println!("conf = {conf:#?}");


   thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }


(1..11).fold(0, |a, b| {
    println!("{}", format!("a: {a} b: {b} result: {}", a + b));
    a + b
});

//println!("{}", );


println!("{}", (1..11).fold(0, |a, b| a + b));


use std::ops::ControlFlow;

let r = (2..100).try_for_each(|x| {
    if 403 % x == 0 {
        return ControlFlow::Break(x)
    }

    ControlFlow::Continue(())
});
assert_eq!(r, ControlFlow::Break(13));

println!("Control Flow is: {:?}", r);

//does nothing because not awaited
some_function(Some("Foo".to_string())).await;
some_function(None).await;

//these will print...
some_function(Some("Foo".to_string())).await;
some_function(None).await;

//handle_async().await;

let nums:[i32;10] = [1,2,3,4,5,6,7,8,9,10];

let nums_result: Result<i32, &str> = nums.iter()
        .map(|&x| {
            println!("x is {}", x);
            x.checked_add(x).ok_or("overflow")
        })
        .sum::<>();
        
        println!("Sum is 55? {:?}", nums_result);


let nums:[i32;1] = [1]; //,2,3,4,5,6,7,8,9,10

let nums_result: Result<i32, &str> = nums.iter()
        .map(|&x| {
            println!("&x is {}", x);
            x.checked_add(x).ok_or("overflow")
        })
        .sum::<>();
        
        println!("Sum is 55? {:?}", nums_result);



let values: Vec<u64> = vec![1, 1, 2, 3, 5 /* ... */];

let mut _even_sum_squares = 0;
let mut even_count = 0;
for i in 0..values.len() {
    println!("item is: {}", i);
    if values[i] % 2 != 0 {
        continue;
    }
    _even_sum_squares += values[i] * values[i];
    even_count += 1;
    if even_count == 5 {
        break;
    }
}


let _even_sum_squares: u64 = values
    .iter()
    .filter(|x| *x % 2 == 0)
    .take(5)
    .map(|x| x * x)
    .sum();



    let start = std::time::Instant::now();

    let jagged_array: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5],
        vec![6, 7, 8, 9]
    ];

    for row in jagged_array {
        for value in row {
            print!("{} ", value);
        }
        println!();
    }



    for _ in 0 .. 10{
        let t = TensorId::new();
        println!("Tensor: {:?}", t);
    }


    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);


    recognise_tree!(expand_to_larch);
    expand_to_larch!();
    call_with_larch!(recognise_tree);


    let rslt = crate_name_util!(@as_foo 222); 

    println!("@ is {} ", rslt); 

    let rslt = crate_name_util!(@as_bar 222); 

    println!("@ is {} ", rslt); 
    
    let rslt = crate_name_util!(); 

    println!("@ is {} ", rslt); 

    let mut rng = rand::rng();
    let tst_random_number = rng.random_range(0.0..1.0);
    let txt_range = vec![rng.random_range(0.0..1.0), rng.random_range(0.0..1.0)];

    println!("Rnd number: {:?}", tst_random_number);    
    println!("Test Range: {:?}", txt_range);


    let instant = std::time::Instant::now();
    test().await;
    dbg!(instant.elapsed());
    
    /*
    let bar = Foo::Bar {
        foo: "foo".to_string(),
        //error[E0451]: field `bar` of struct `Bar` is private
        //bar: "bar".to_string()
    };
    */
    //let bar = Foo::Bar::new(foo:"foo".to_string(), bar:"bar".to_string());
    let bar = foo::Bar::new("foo".to_string(), "bar".to_string());
    let bar2 = foo::Bar::new("foo".to_string(), "baz".to_string());
    println!("{:?} -> {}", bar, bar.foo);
    println!("bar -> {}", foo::get_bar(&bar));
    println!("bar2 -> {}", foo::get_bar(&bar2));

    println!("Duration: {:?}", start.elapsed());
}

