
# enumerates

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


> **Exercice if you have time**
> * Why does I use the macro `assert_abs_diff_eq` rather than `assert_eq` to compare results in test `ex2_can_convert` ?
> * If you try to use this macro in the `main` function, program will not compile. Why ? And why this is a good practice ? (tips, have a look to `Cargo.toml` file).