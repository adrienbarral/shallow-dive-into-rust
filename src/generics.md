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

If we need to write this method for all types that are "comparable", that means on which the `<` operator apply, we can write a generic version of this methods like this : 

```rust
fn min(n: T, m: T) -> T 
where T: PartialOrd 
{
    if n < m {
        n
    }
    m
}
```
This is a method that use the **generic** feature of rust.

If your are familiar with C++, you may notice that the syntax looks like
Exercice : 
