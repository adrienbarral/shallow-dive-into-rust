# Some very cool crates

## [Clap :](https://docs.rs/clap/latest/clap/) 

To handle CLI (Command Line Arguments) in a very elegant way.

```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
```
## [Serde :](https://serde.rs/)

To deserialize and serialize data into rust structs.

Here is an exemple of serde_json, the json parser for serde (serde support many other formats).

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
```

## [anyhow :](https://docs.rs/anyhow/latest/anyhow/)

To return a result anyhow its type.
Has we show in the section [on Result](./result_option.md), the ellision operator (?) can only be used when the result has the same type than the containing method. Because all errors implements the trait `Error`, if your method return a result with the type : ```rust Result<T, Box<dyn Error>>```, you can use the ellision operator everywhere. The crate anyhow define a Result type for which the second type is a generic error, allowing to write : 

```rust
use anyhow::Result;

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}
```

Here, read_to_string return an `std::io::Error`, and from_str return a `serde::Error`. Both of them implementing the trait ```Error````

## [This error](https://docs.rs/thiserror/latest/thiserror/)

Implementing the trait `std::Error` could be complex. The crate this_error simplify this process.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```

So here is a solution of exercice 6 where I asked you to open a file, get the first line and parse a tuple from this line :

```rust
use std::{fs::File, io::Read, path::Path};

use thiserror::Error;
#[derive(Debug, Error)]
enum ParseError {
    #[error("Not enough element to parse, 2 are required {0} found")]
    NotEnoughElementError(usize),
    #[error("Not enough line in  file")]
    NotEnoughLineInFile,
}

fn parse_line(line: &str) -> anyhow::Result<(f32, f32)> {
    let res: Vec<&str> = line.split(";").collect();

    match (res.get(0), res.get(1)) {
        (Some(elem1), Some(elem2)) => {
            return Ok((elem1.parse::<f32>()?, elem2.parse::<f32>()?));
        }
        _ => return Err(ParseError::NotEnoughElementError(res.len()).into()),
    }
}

fn parse_from_file(file: &Path) -> anyhow::Result<(f32, f32)> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let first_line = contents
        .lines()
        .nth(0)
        .ok_or::<ParseError>(ParseError::NotEnoughLineInFile.into())?;

    parse_line(first_line)
}
```