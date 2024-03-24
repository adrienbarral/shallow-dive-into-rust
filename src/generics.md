# Generics

This feature of the language allow to define behaviour on abstract types that are known implementing a certain trait.
For exemple, a method able to return the min value between two `f32` can be :

```rust
fn min(n: f32, m: f32) -> f32 {
    if n < m {
        n
    } 
    m
}
```

A old C++ developer will write something like that : 

```rust
fn min<T>(n: T, m: T) -> T 
{
    if n < m {
        return n;
    }
    return m;
}

fn main(){
    let m = min(3, 2);
}
```

But this will not compile with the following error message : 
```
error[E0369]: binary operation `<` cannot be applied to type `T`
 --> src/lib.rs:3:10
  |
3 |     if n < m {
  |        - ^ - T
  |        |
  |        T
  |
help: consider restricting type parameter `T`
  |
1 | fn min<T: std::cmp::PartialOrd>(n: T, m: T) -> T 
  |  
  ```
Because rust will compile the generic method, where C++ will wait to have a concrete type to generate a method and compile it. This is a very different approach.

If we need to write this method for all types that are "comparable", that means on which the `<` operator apply, we can write a generic version of this methods like this : 

```rust
fn min<T>(n: T, m: T) -> T 
where T: PartialOrd 
{
    if n < m {
        return n;
    }
    return m;
}
```
This is a method that use the **generic** feature of rust.

Exercice : 
