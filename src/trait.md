# Trait

In all Object Oriented Programing (OOP) language, there is a feature allowing to share similar behaviour of object. In C++, we can use virtual methods and class inheritance. In Java, we define interfaces and subclasses... In rust we use 

Here is an exemple in C++ that we will rewrite in rust : 

```c++
class Identity{
public :
    Identity(const std::string& name): m_name(name){}
    std::string WhatsYourName() {
        return m_name;
    }
private :
    std::string m_name;
}

class Teacher : public Perspon {
    Teacher(const std::string& name, int salary):
    Identity(name),
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
    println!("Hello again {}", adrien.whats_your_name());

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

This is how we share behaviour in Rust. We create a trait, then we create function that know how to process this trait. Finally we implement this trait for our structures.

## The `dyn` keyowrd :

I decide to pass as argument of my trait processing method the following type : `&dyn Named`. That can be translated by *a reference to an object that implements the trait Named that will be discover dynamically*.
I would also be able to write the following signature :  `fn does_its_name_is_nice(named: &impl Named) -> bool`. Here we use **static dispatch**. To understand the difference between static and dynamic distpatch, you can read [this nice article](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/static-and-dynamic-dispatch.html). But this is out of scope of this course.

Static dispatching generate more code, but is more straight forward at runtime (it consume less CPU). Dynamic dispatching have a very slight CPU overload. In C++, we don't have choice, only dynamic dispatching is implemented (or maybe you can decide to do static dispatching if you master meta programming tricks.).

**If you write code for something else than a Micro Controller, I strongly suggest to use dynamic dispatching.**. Because dynamic dispatching allow polymorphism like this : 

```rust
fn main() {

    let everybody = Vec::<Box<dyn Named>>::new();
    everybody.push(Box<dyn Named>::new(Teacher{...}));
    everybody.push(Box<dyn Named>::new(Identity{...}));

    for named in everybody {
        println!("Hello : {}", named.whats_your_name());
    }
}
```

## The `derive` macro is just code generation :

You may notice than in C++ 
## Exemple of realworld trait :

