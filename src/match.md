# The `match` keyword

This is a very powerfool keyword introduced in rust. At a first glance, it looks like a `switch / case` in C++.

```rust
enum Country {
    FRANCE, 
    ITALY, 
    USA
}

fn get_capital(country: Country) -> String {
    match country {
        Country::FRANCE => String::from("Paris"),
        Country::ITALY => String::from("Roma"),
        Country::USA => String::from("Washington D.C"),
    }
}
```

*Note the gain in readability to have the `return` keyword optional.*

This `match` block can be read : *If country::France match the content of variable country, execute what is after the arrow, and so on... for other countries*

```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

`PATTERN` can be very complex and must have the same type as `VALUE`. All `EXPRESSION`s must have the same type. That will be the type of the `match` arm. In the exemple above, `PATTERN` is simply an enum.


```rust
enum Honours {
    Repeat,
    Pass,
    Good,
    VeryGood,
    Cheater
}
fn print_honors(grade: u8) -> Honours {
    match grade {
        _grade @ 0..=9 => Honours::Repeat,
        _grade @ 10..=13 => Honours::Pass,
        _grade @ 14..=15 => Honours::Good,
        _grade @ 16..=20 => Honours::VeryGood,
        _ => Honours::Cheater
    }
}
```

> **Exercice**
>
> Write the main function of the `ex4` to display Heads or Tail according to the result value of the `flip_coin` method. Do this by using the `match` keyword.

To illustrate potential complexity of the "pattern" matched by the `match` keyword here is an example from the rustlang official book : 

```rust
fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
``` 
