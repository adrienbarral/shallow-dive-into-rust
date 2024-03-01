# `String` vs `str` and the slice type

## The slice type

A **slice** represent a reference to a contiguous sequence of elements in a collection.

```rust
let vector: Vec<u32> = vec![1,2,3,4,5,6,7];
let first_half = &vector[0..3]; // here, type is &[u32]
    
println!("{:?}", first_half); // will display 1,2,3
```

We can create a slice for every type : 
```rust
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
}
fn main() {
    let students = vec![
        Student { name: "Alice".to_string(), age: 20 },
        Student { name: "Bob".to_string(), age: 22 },
        Student { name: "Charlie".to_string(), age: 24 },
    ];

    let first_two_students = &students[0..2];
    println!("{:?}", first_two_students);
}
```

A slice contains a pointer to the beginning of a memory region and a length.

Slice are very usefull when you want to return a reference to a part of a collection, and you want to be sure that this collection will not be modified while you use this reference.
As an exemple, the following program **does not compile** : 
```rust
let mut students = vec![
        Student { name: "Alice".to_string(), age: 20 },
        Student { name: "Bob".to_string(), age: 22 },
        Student { name: "Charlie".to_string(), age: 24 },
    ];

let first_two_students = &students[0..2];

students.push(Stundent{name: "Romeo".to_string(), age: 20});

println!("{:?}", first_two_students);

```
Because when we do a push into a vector, the whole memory can be moved. So the reference to the first two students is no longer valid after a push operation. The borrow checker detect that we use this reference in the println call, so it forbid to mutate the student vector while this reference live.


## `String` or `str` ?

`str`is a primitive type representing a slice to UTF8 characters.

If we write : 
```rust
let toto = "Hello world"; 
```
Here toto type is : `&str 'static` (we will see later what does `static` mean, for now just note that this is a slice to str.)

The following example from rustlang book explain what is the difference between a `String` and `str` :

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

![String vs str](images/string_vs_str.svg)

> **Exercice**
>
> Make passing the test in exercice 5_1.
>
> Hint 1 : A method called `chars()` exists on `str`type that allow to iterate over all characters.
>
> Hint 2 : A more elegant (but more tricky) way to solve this problem is to use the methd `split_whitespace()`.
