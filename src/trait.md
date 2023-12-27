# Trait

In all Object Oriented Programing (OOP) language, there is a feature allowing to share similar behaviour of object. In C++, we can use virtual methods and class inheritance. In Java, we define interfaces and subclasses... In rust we use 

Here is an exemple in C++ that we will rewrite in rust : 

```c++
class Person{
public :
    Person(const std::string& name): m_name(name){}
    std::string WhatsYourName() {
        return m_name;
    }
private :
    std::string m_name;
}

class Teacher : public Perspon {
    Teacher(const std::string& name, int salary):
    Person(name),
    m_salary(salary) {}
}

void main() {
    Teacher adrien("Adrien BARRAL", 40000);
    std::cout << adrien.WhatsYourName() << std::endl;
}
```

```rust
trait Named {
    fn whats_your_name(&self) -> String;
}

struct Identity {
    name: String,
}

struct Teacher {
    identity: Identity,
    salary: u32
}

struct Student {
    identity: Identity,
    class: String
}

impl Named for Identity {
    fn whats_your_name(&self) -> String {
        self.name.clone()
    }
}

impl Named for Teacher {
    fn whats_your_name(&self) -> String {
        self.identity.name.clone()
    }
}

fn main() {
    let adrien = Teacher{
        identity: Identity{name: String::from("Adrien BARRAL")},
        salary: 40_000
    };
    println!("Hello {}", adrien.identity.whats_your_name());
}
```

Here, we can have the feeling that we don't share a lot of behaviour un rust (compared to C++).

But now, let create a method that guess if a name is nice or not : 

```rust
fn does_its_name_is_nice(named: &dyn Named) -> bool {
    if named.whats_your_name().contains("Adrien") {
        return true;
    }
    return false;
}
```

We can use this method : 

```rust
fn main() {
    let adrien = Teacher{
        identity: Identity{name: String::from("Adrien BARRAL")},
        salary: 40_000
    };
    
    let john = Student {
        identity: Identity(name: String::from("John SMITH")),
        class: String::from("MIR Master")
    };
    assert_eq!(does_its_name_is_nice(&adrien), true);
    assert_eq!(does_its_name_is_nice(&john.identity), false);
}
```

This is how we share behaviour in Rust. We create a trait, then we create function that know how to process this trait. Finally we implement this trait with structures.

## The `derive` macro is just code generation :

## Exemple of realworld trait :

