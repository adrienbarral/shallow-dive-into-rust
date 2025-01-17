# Variables type

All variables are strictly typed, most of the time type is infered.


```
let <variable name>[: <variable type>] = <variable value>;
```

Both lines are strictly equivalents : 

```rust
let a: i32 = 8;
```

```rust
let a = 8; // by default integer number is infered as i32
```

**In rust we do an extreme usage of type inference, this why vscode extension that display type are very usefull**

## Scalar types :

### Integers

|rust type|range|name|
|---|---|---|
|i8|-128 &rarr; 127| signed 8 bits integer|
|i16|-32768 &rarr; 32767| signed 16 bits integer|
|i32|-2,147,483,648 &rarr; 2,147,483,647| signed 32 bits integer|
|i64|-9,223,372,036,854,775,808 &rarr; 9,223,372,036,854,775,807| signed 64 bits integer|
|isize| depend of the CPU. i32 for a 32Bits CPU, i64 for a 64Bits|pointer size|
|u8|0 &rarr; 255| signed 8 bits integer|
|u16|0 &rarr; 65535| signed 16 bits integer|
|u32|0 &rarr; 4,294,967,295| signed 32 bits integer|
|u64|0 &rarr; 18,446,744,073,709,551,615| signed 64 bits integer|
|usize| depend of the CPU. u32 for a 32Bits CPU, u64 for a 64Bits|pointer size|

### Real numbers : 
|rust type|range|name|
|---|---|---|
|f32|-3.4E38 &rarr; 3.4E38|Single precision floating point|
|f64|-1.7E308 &rarr; 1.7E308|Double precision floating point|

### Characters

In rust a character is coded on 4 bytes and is UTF-8 encoded.
So writing the following line is perfectly correct :

```rust
let smiley: char = '😀';
let sentence = String::new("I am so happy ! 😀");
```
Note that `char` is initialized with simple quotes and string with double quote. In both case, we can use UTF-8 charset.

### Booleans and `if` expression

As in all languages booleans values can be either `true` or `false`

```rust
let t = true; // with type inference
let a: bool = false; // with explicit type annotation.
```

Boolean can be used in `if` expression combined with `&&` and `||` operators.

## There is no "implicit cast" like in C/C++

In c++ you can write : 
```c++
uint32_t foo = 2.1;
```

In rust, you can't. The following line produce an error : 
```rust
let foo: u32 = 2.1;
```

```
error[E0308]: mismatched types
 --> ex1/src/main.rs:4:20
  |
4 |     let foo: u32 = 2.1;
  |              ---   ^^^ expected `u32`, found floating-point number
  |              |
  |              expected due to this
```

# Variables mutability

By default in rust, a variable is immutable. The following lines are incorrects :
```rust
let foo = 2;
foo = 3;
```

```
error[E0384]: cannot assign twice to immutable variable `foo`
 --> ex1/src/main.rs:5:1
  |
4 |     let foo = 2;
  |         ---
  |         |
  |         first assignment to `foo`
  |         help: consider making this binding mutable: `mut foo`
5 | foo = 3;
  | ^^^^^^^ cannot assign twice to immutable variable
```

A variable is mutable if it is defined like this : 

```rust
let mut foo = 2;
foo = 3;
```

# Function

Exemple of a rust function : 

```rust
fn is_odd(number: i32) -> bool {
    return number % 2 != 0;
}
```

`return` keyoword is not mendatory. The previous function can  be written :

```rust
fn is_odd(number: i32) -> bool {
    number % 2 != 0
}
```

> **Exercice**
>
> Create a function called `addition` taking into arguments two `u32` and returning sum of this two numbers as a `u32`.
> This must make passing the unit test called `ex1_addition`.

# Structures and basic objects


## `struct`

A structure in rust is used to package data together.

```rust
struct Person {
    name: String,
    birth_date: chrono::NaiveDate,
    gender: Gender
}
```

A structure can be instanciated :

```rust
let me = Person {
    name: "BARRAL",
    birth_date: chrono::NaiveDate::from_ymd(1984,09,29),
    gender: Gender::Male
};
```

Methods can be associated to a structure : 
```rust
impl Person {
    pub fn whats_your_name(&self){
        println!("My name is : {}", self.name);
    }
}
```

This allow to call this methods on the "me" object instanciated previously : 

```rust
let me = Person {
    name: "BARRAL",
    birth_date: chrono::NaiveDate::from_ymd(1984,09,29),
    gender: Gender::Male
};

me.whats_your_name();
```

In C++ there is a notion of **constructor** method, called when we instanciate an object from the stack or the heap with the `new` keywords.
There is no equivalent in rust. If you have nothing particular to do when creating an object, just do like in the previous exemple. Else, you can create a function in the implementation of this object that return `Self`. By convention this function will be called `from` or `new`.

```rust
struct Rectangle{
    width: f32,
    height: f32,
    area: f32
}

impl Rectangle {
    pub fn from(width: f32, height: f32) -> Self {
        Rectangle{
            width: width,
            height: height,
            area: width*height
        }
    }
}
```

You can use this struct as it : 

```rust
let r = Rectangle::from(10.0, 15.0);
println!("Area is : {}", r.area);
```

> **Exercice**
>
> From scratch, create a new project named `is_it_soon_weekend` that will import the crate **chrono**.
> By reading the chrono documentation make your program display "YES" if we are a Friday. It will display "We are in weekend 🎉" if we are Sunday or Saturday, and it will
> display "NO" for other days of the week. 
> 
> *This exercice is inspired by the website [https://estcequecestbientotleweekend.fr/](https://estcequecestbientotleweekend.fr/)*.

<details>
  <summary>Tips</summary>

  In the chrono crate it exists an enumerate called `Weekday` that can be really helpfull for that !

</details>

# Control Flow

## Loop
An infinite loop. We must use `break` or `return` to getting out of this loop.

```rust
loop {
    println!("What is the best programming language ever : ");
    let line: String = text_io::read!("{}\n");
    if line == String::from("rust") {
        break;
    } else {
        println!("Try again");
    }
}
println!("Good boy/girl !")
```

## while
Exactly like in C/C++ and other language. The content of the loop is executed while a condition is true.

There is no equivalent of `do / while` pattern of c++. If we want to run the loop code once before deciding use a boolean variable. Look at the previous exemple rewritten with a `while` loop :

```rust
let mut should_stop = false;
while should_stop == false {
    println!("What is the best programming language ever : ");
    let line: String = text_io::read!("{}\n");
    if line == String::from("rust") {
        should_stop = true;
    } else {
        println!("Try again");
    }
}
println!("Good boy/girl !")
```

## for

Allow to iterate over **iterable collection**.
An array is an iterable collection :

```rust
let array = [10,20,30,40];
let mut sum = 0;

for elem in array {
    sum = sum + elem;
}
assert_eq!(sum, 10+20+30+40);
```
`Range` is the list of number between two values and it is written `(val_min..val_max)`. This is also iterable.

```rust
for i in (0..10) {
    println!("{}", i);
}
```
will display `0 1 2 3 4 5 6 7 8 9`

A lot of things are iterable, we will see later what the sentence "every things that implements the `Iterator` traits" means.
