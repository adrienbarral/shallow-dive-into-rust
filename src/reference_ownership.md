# Reference and ownership : 

The rustlang book says : 

*Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector* 

## What is memory safety ?

Memory safety is the science to avoid : 
* Memory leak
* Dangling pointer or reference
* Usage of freed memory

### Stack vs Heap

There are two types of memory :
 * Stack : fast to access, size limited memory (8192 kbyte for each thread by default on linux). Easy to manage (compiler release memory as the variables goes out of scope).
 * Heap : only limited by hardware memory, but slower to access and tricky to manage.

In C++ we can create and release data on heap by using the `new` and `delete` keywords.

```c++
int* create_and_fill_array(int size, int value) {
    int *array = new int[size];
    for(int i = 0 ; i < size ; i++){
        array[i] = value;   
    }
    
    return array;
}

int main()
{
    int expected_size = 100;
    int *array = create_and_fill_array(expected_size, 1);
    std::cout << array[0] << std::endl;
    delete[] array
}
```

![StackVsHeap](images/StackVsHeap.drawio.png)


### Unsafe memory usage : 

In the above exemple, if programmer forgot to `delete[] array` we have a **memory leak**.

In the above exemple, if programmer swap the `cout` and the `delete` line, we have a **freed memory usage**.

In the bellow exemple (that compile, with a warning) `array` in main function refer to a memory area that is not the effective array ! We use a **dangling pointer**

```c++
int* create_and_fill_array_buggy(int size, int value) {
    int array[size];
    for(int i = 0 ; i < size ; i++){
        array[i] = value;   
    }
    
    return array;
}

int main()
{
    int expected_size = 100;
    int *array = create_and_fill_array(expected_size, 1);
    std::cout << array[0] << std::endl;
}
```

### Memory Management

All programming language have their own memory management strategy. Languages like Java, Javascript use a garbage collector to release memory of no longer used variables. C let the developper manage memory. Developer create an object with `malloc` and release memory with `free`. C++ has quite the same strategy with the `new` and `delete` keywords.

Patterns like RAII (Resource Acquisition Is Initialization) can be implemented in various language to ease memory management.

```c++
class IntArray {
public :
    IntArray(int size, int value) {
        array = new int[size];
        for(int i = 0 ; i < size ; i++){
            array[i] = value;
        }
    }
    ~IntArray() {
        delete[] array;
    }
    int operator[](size_t i){
        return array[i];
    }
    
private :
    int *array;
};

int main()
{
    IntArray array(100, 1);
    std::cout << array[0] << std::endl;
}
```

A generalized version for any type : 
```c++
template<typename T>
class Pointer {
public :
    Pointer(T* value):value(value) {}
    ~Pointer() {
        delete value;
    }
    T* operator->(){return value;}
    
private :
    T* value;
};

struct Position {
    double latitude;
    double longitude;
};

int main()
{
    Pointer<Position> position_ptr(new Position{43.0,6.0});
    std::cout << position_ptr->latitude << std::endl;
}
```
In rust, there is no `new` or `delete` keyword, we will use `Box` to create data on the heap. A `Box` is more or less the `Pointer` class described above.

```rust
struct Position{
    latitude: f64,
    longitude: f64
}

fn main() {
    let position = Box::new(Position{latitude: 43.0, longitude: 6.0});
    println!("Latitude : {}", position.latitude);
}
```

# Ownership

Example of this section is a copy of the [rustlang book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).

When a data is on the heap, and we want to copy it, it exists 3 way to copy it. Consider the following example : 
```rust
let s1 = String::from("hello");
let s2 = s1;
```
## Copy : 
![copy](images/copy.svg)
Here both S1 and S2 refer to the same data. We need to manually ensure that the data is freed when no longer used !

## Clone : 
![clone](images/clone.svg)
This will duplicate data. This is safe, but time and memory consuming for non trivially copyable objects.

## Move :
![move](images/move.svg)

After this, `s1` is no longer available.

**This is how rust manage the copy of objects**

Here we can say that `s1` give **ownership** of data to `s2`

This why in rust, writting the following code produce an error : 

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{} world !", s1);
```
# Why the moving strategy is the good ?

That assure that there is only one owner of a data at a time. This is a way to avoid conccurency issue.

# Reference
But we can write : 
```rust
let s1 = String::from("hello");
let s2 = &s1;
println!("{} world !", s1);
```
Here type of `s2` is not a `String` but a `String&`, an reference to a string. We will say that `s2` **borrow** the string to `s1`.

# Behaviour is different for stack variables

Variables created on stack (scalar, boolean, and all variables implementing the Copy Traits), are not moved by default. They are cloned.
So this is correct to write : 

```rust
let a = 1;
let b = a;
println!("A: {}", a);
```

> **Exercice**
>
> Create the content of the unit test `ex3_can_generate_full_name`
>
> Do the exercice explained in the main function of ex3.

# Reference and mutability : 

Reference allows aliasing, that means that multiple variables can refer to the same data. Aliasing is intrinsically safe. But it can be dangerous when coupled to mutability. Rust prevent this by adding the following rules :

* It could be several immutable references to a variable
* But only one mutable reference to a variable

Why it is dangerous to have a mutable reference and some immutable references at the same time ?

Imagine the following code : 

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];
v.push(4);
println!("Third element is {}", *num);
```

When we push a new element in the vector, the vector could be reallocated in memory. So the reference `num` could be invalidated.

![copy](images/Reallocate_vector.png)

In rust the following code produce an error :
```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:1
  |
4 | let num: &i32 = &v[2];
  |                  - immutable borrow occurs here
5 | v.push(4);
  | ^^^^^^^^^ mutable borrow occurs here
6 | println!("Third element is {}", *num);
  |                                 ---- immutable borrow later used here
```

`v`is borrewed as mutable because signature of the "push" method is : 
```
pub fn push(&mut self, value: T)
```

When we call push, we borrow `v` as mutable, and we can't borrow `v` as immutable at the same time. We can't borrow a variable as mutable if it is already borrowed.

But, the following code is perfectly valid :
```rust
let mut v: Vec<i32> = vec![1, 2, 3];
{
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);
}
v.push(4);
```

Because rust (the borrow checker), understand that the scope of the immutable borrow is limited to the block `{}`. So, the mutable borrow is not in conflict with the immutable borrow when we call the push method.


Let's recap by citing the [rustlang book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) : 

> * At any given time, you can have **either** one mutable reference **or** any number of immutable references.
> * References must always be valid.

To deeply understand everyhting, refer to this [very very powerfull explanation](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html).

# Reference and mutability in one code snippest : 

```rust
fn main() {
  let mut s = String::from("Toto");
  let mut s2 = String::from("Tata");
  let s3 = String::from("Titi");
  let r1 = &s;      // r1 has type "reference to a imutable string",
                    // So we can't neither modify the string "Toto" nor affect r1
                    // to another reference (we can't write r1 = s2)
  println!("{r1}");

  let mut r2 = &s; // Here r2 refer to the string Toto, but we can affect r2 to another string
  r2 = &s2;        // has we do here
  
  let r3 = & mut s; // Here r3 is a reference to a mutable string, we can modify toto !
  r3.push('!');
  
  // println!("{r1}");  // If we uncomment the following line we have a compilation error.
  // Because r1 is a immutable reference of s, and r3 a mutbale reference of s. 
  // It is not possible to have a mutable reference and an immutable reference on a same object
  // Lifetime of r1 end the last time we use this variable. So at line 7. At line 14, when we create the 
  // reference to mutable string, all immutable reference are "dead" (their lifetime has expired).
  // So there is no problem.
  // If we uncomment this println, lifetime of r1 end after the creation of mutable reference, this is not allowed
  

  
  // let s4 = & mut s3; // This line doesn't compile because s3 is an immutable string, 
                        // so we can't borrow it as a mutable reference !
}
```

# Reference as struct attribute.

A structure can have a reference as attribute. But doing this will imply that the lifetime of the variable referenced will be explicitly set : 

```rust
struct Person<'a> {
    name: &'a String,
    age: u8
}

impl<'a> Person<'a> {
    pub fn say_hello(&self) {
        println!("Hello, my name is : {}", self.name);
    }
}

fn main() {
    let adrien = String::from("Adrien");
    
    let me = Person{name: &adrien, age: 39};

    me.say_hello();
}
```
From the excellent course [here](https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html), the lifetime annotation can be read as : *A reference to the Person struct can't outlive the reference it hold into its attribute name*

This allow to create struct that doesn't own data in its attributes, but refer to them.