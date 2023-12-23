# Option : Managing optionnal values

Imagine you have to write a method that must load a dictionnary (containing words of language, and their definition ) from a file, and give the definition of the word passed in argument.

What does this methods will look like in term of signature in rust ?

```rust
fn find_definition(word: &String) -> Option<String> {
    if word == "apple" {
        return Some(String::from("a round fruit with firm, white flesh and a green, red, or yellow skin"));
    } else if word == "tomato" {
        return Some(String::from("a round, red fruit with a lot of seeds, eaten cooked or uncooked as a vegetable, for example in salads or sauces"));
    } /*... all other words of the language */
    else {
        // If word doesn't exists in the language, return none :
        return None;
    }
}
```

**If the word can't be found, the method will return `None`**

We will use this method like this : 

```rust
let word = String::from("apple");
match find_definition(&word) {
    Some(def) => println!("Definition of word {} is : {}", word, def),
    None => println!("Can't find definition of word {}", word);
}
```

> **Exercice**
>
> Write code to make passing the unit test `ex5_can_find_first_odd_number`. Write a function that will take as argument a reference to a vector of i32 `&Vec<i32>` and
> returning None if there is no odd number in the vector, and the first odd number otherwise.

## Handling default value
In the previous example, you may want to create a function returning the found definition or a default value. This can be done with : 

```rust
fn find_definition_or_default(word: &String) -> String {
    find_definition(word).unwrap_or(String::from("Can't find definition of this word"))
}
```

# Result : When something goes wrong.
When a function can not do what they have to do, this function return an error. In C++ you can either use exception, returning boolean value, or an integer. There is no standard way to return an error. In rust, the standard way to return an error is to return a `Result`.

As an example, assume you are writing the interface of an USV like we do at exail. There is a method to set commands. This method can be written as : 

```rust
enum CommandError {
    RudderAngleError,
    EngineError
}
enum Direction{
    Forward,
    Backward
}
fn set_rudder_command(rudder_angle_deg: f32) -> Result<(), CommandError> {
    // Our rudder have a course of +/- 30° :
    if(rudder_angle_deg.abs() > 30) {
        return Err(CommandError::RudderAngleError);
    }
    // Sending command to low level controller after doing bound checks.
    interface_to_plc.send(rudder_angle_deg);
    Ok(())
}
fn set_engine_commnad(, engine_rpm: f32, direction: Direction) -> Result<(), CommandError>{
    if(engine_rpm < 0 || engine_rpm > 4000) {
        return Err(CommandError::EngineError);
    }
    match direction{
        Forward => interface_to_plc.send('FORWARD'),
        Backward => interface_to_plc.send('BACKWARD'),
    }
    interface_to_plc.send(engine_rpm);
    Ok(())
}
```

## The ellison (?) operator


```rust
fn set_commands(rudder_angle_deg: f32, engine_rpm: f32, direction: Direction) -> Result<(), CommandError>{
    if set_rudder_command(rudder_angle_deg).is_err() {
        return Err(CommandError::RudderAngleError);
    }
    if set_engine_command(engine_rpm, direction).is_err() {
        return Err(CommandError::EngineError);
    }
    Ok(())
}
```

Ellision operator will allow to forward an error to the calling method, without having to write these anoying `if....`

```rust
fn set_commands(rudder_angle_deg: f32, engine_rpm: f32, direction: Direction) -> Result<(), CommandError>{
    set_rudder_command(rudder_angle_deg)?;
    set_engine_command(engine_rpm, direction)?;
    Ok(())
}
```

