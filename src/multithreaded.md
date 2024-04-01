# Multi threaded application

In a multithread application, we need two things : 
1. Share a state
2. Exchange information

In this section we will conver thread of the standard rust library. We will see that this standard library have some drawbacks (but is already better than thread management standard library of other languages like C++). This is why it exists other libraries to manage thread (like tokio), you will discover it when you will need it, concept of the standards library and tokio are the sames.

## Create a thread 

To create a trehad use ```std::thread::spawn``` this method has the following signature : 
```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

That means, spawn take on argument of type F where F is a function (FnOnce) taking no argument and returning something of type T where T implement the Send trait with a lifetime static (that means that can have the whole life time of the application, I will detail it later).
The spawn method return a Join Handle containig a value of type T as defined previously.

So the following code will spawn 2 threads that will infinitelly display Hello ... World ...

```rust
fn thread1() {
    loop {
        println!("Hello ...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }    
}

fn thread2() {
    loop {
        println!("World !");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
fn main() {

    let j1 = std::thread::spawn(thread1);
    let j1 = std::thread::spawn(thread2);
    
}
```

### Introduction to closure : 

It's a very command practice to use closures to define method of a trait, because closure capture their environment. A closure is also called an **anonymous function** and is defined like this in rust : 

```rust
    let name = String::from("Adrien");
    let f = || {println!("Hello : {}", name);};

    f();
    
    println!("Hi again : {}", name);
```

By default, variables are capture by reference. But sometimes, we would like the closure take the ownership of the variables, this is possible by adding the `move` keyword.

```rust
    let name = String::from("Adrien");
    let f = move || {println!("Hello : {}", name);};

    f();
    
    // println!("Hi again : {}", name);
    // If I uncomment the line above, I have a compilation error because "name" was moved into the closure, and can no longer be used
```

### Exercice :
Create two threads with a closure. Both threads contains a loop with a sleep of 500 ms. The first one will display even numbers 0 2 4 ... the second will display odd numbers 1 3 5 ...

## Sharing a state

To share a state, the state must be protected by a mutex.

For example, in my previous example, if I want that both thread increment a same number, I would like to write : 

```rust

let counter = Mutex::new(0);
    
let j1 = thread::spawn(|| {
   let mut c = counter.lock().unwrap(); 
   c += 1;
});
    
let j2 = thread::spawn(|| {
    let mut c = counter.lock().unwrap(); 
    c += 1;
});
```

This fail to compile with the following error message : 
```
closure may outlive the current function, but it borrows `counter`, which is owned by the current function
  |
5 |     let j1 = std::thread::spawn(|| {
  |                                 ^^ may outlive borrowed value `counter`
6 |        let mut c = counter.lock().unwrap(); 
  |                    ------- `counter` is borrowed here
  |
note: function requires argument type to outlive `'static`
```

Because remember the signature of the spawn method that require its content to be ``` 'static ```. The function executed in a thread can outlive the function that create this thread.

So we must use an Arc (Atomic Reference Count) to store the mutex. Atomic Reference Count is an implementation of the [RAII](https://fr.wikipedia.org/wiki/Resource_acquisition_is_initialization) design pattern.

The final working exemple is :
```rust
let counter = Arc::new(Mutex::new(0));
    
let counter_for_t1 = counter.clone();
let _j1 = std::thread::spawn(move || {
   let mut c = counter_for_t1.lock().unwrap(); 
   *c += 1;
});
    
let _j2 = std::thread::spawn(move || {
   let mut c = counter.lock().unwrap(); 
   *c += 1;
});
```

### Exercice write an application that will create two threads : 
* The first one will increment a number every seconds
* the second one will wait for a number in stdin. This number will be the incremented value. If the user type "exit", both thread should return.

## Exchanging information

Imagine we want to have a thread generating a text, and another thread writing this text on the standard output. This is a very common use case, in a realworld application, the second thread will write on a network socket, or in a file...

To do this, we will use Channel. A channel have two parts, the sender and the receiver.

```rust
    let (tx, rx) = sync::mpsc::channel();

    let j1 = thread::spawn(move || {
        for i in 1..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let j2 = thread::spawn(move || {
        for i in 1..10 {
            let received = rx.recv().unwrap();
            println!("Thread 2: {}", received);
        }
    });
```

This channel is called MPSC for Multiple Producer Single Consummer. That means that the `tx` is clonable and can be sent to several threads.

### Exercice : 
Modify the previus program to use a channel to notify that threads must exit.


## Other synchronization primitives

Mutex and Channel are known as **synchronization primitive**. It exists a lot of different synchronization primitive (in rust and that we can find in all other languages). A lesson could be focused on all these primitives. Let me show you an overview.

### In the standard Library

List copied from [the official documentation](https://doc.rust-lang.org/nightly/std/sync/index.html).

* `Arc`: Atomically Reference-Counted pointer, which can be used in multithreaded environments to prolong the lifetime of some data until all the threads have finished using it.
* `Barrier`: Ensures multiple threads will wait for each other to reach a point in the program, before continuing execution all together.
* `Condvar`: Condition Variable, providing the ability to block a thread while waiting for an event to occur.
* `mpsc`: Multi-producer, single-consumer queues, used for message-based communication. Can provide a lightweight inter-thread synchronisation mechanism, at the cost of some extra memory.
* `Mutex`: Mutual Exclusion mechanism, which ensures that at most one thread at a time is able to access some data.
* `Once`: Used for a thread-safe, one-time global initialization routine
* `OnceLock`: Used for thread-safe, one-time initialization of a global variable.
* `RwLock`: Provides a mutual exclusion mechanism which allows multiple readers at the same time, while allowing only one writer at a time. In some cases, this can be more efficient than a mutex.

* `atomic` : native datatypes (i32, i8, ...) that doesn't require to be stored in a mutex to be protected

### In Tokio, a very famous crate

List from the [tokio's documentation](https://docs.rs/tokio/latest/tokio/sync/index.html).

* `mpsc` : sending multiple value from Multpile Producer Single Consumer
* `oneshot` : Sending a single value from a single producer to a single consumer
* `broadcast`: sending many values from many producers to many consumers
* `watch` : sending many values from a many producer to many consumers