# Variables type

All variables are strictly typed, most of the time type is infered.


```
let <variable name>[: <variable type>] = <variable value>
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
fn is_prime
```
## `return` keyoword is not mendatory


> **Exercice**
>
> Create a function called `addition` taking into arguments two `u32` and returning sum of this two numbers as a `u32`.
> This must make passing the unit test called `ex1_addition`.

# Structures and basic objects

## enumerates

### Level 0 : As in C/C++ :
```rust
enum Gender {
    Male,
    Female
}

let gender: Gender = Gender::Male;
```

### Level 1 : We can attach data to enumerate values

For exemple, imagine we want to define a function to control a robot speed, unit of the desired speed can be stored in an enum :

```rust
enum Velocity {
    MilesPerHours(f32),
    KiloMetersPerHours(f32),
    Knot(f32)
}
```

> **Exercice**
>
> Make the test of ex2 passing by creating a function called `convert_to_meter_per_seconds` taking into argument a variable with type `Velocity` and returning a `f32` in meter per seconds.
> Constant allowing to convert from the velocity unit to meter per seconds are given at the top of the source file.

> **Exercice for next week**
> * Why does I use the macro `assert_abs_diff_eq` rather than `assert_eq` to compare results in test `ex2_can_convert` ?
> * If you try to use this macro in the `main` function, program will not compile. Why ? And why this is a good practice ? (tips, have a look to `Cargo.toml` file).

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

This allow to write call this methods on the "me" object instanciated previously : 

```rust
let me = Person {
    name: "BARRAL",
    birth_date: chrono::NaiveDate::from_ymd(1984,09,29),
    gender: Gender::Male
};

me.whats_your_name();
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

## for

## while

