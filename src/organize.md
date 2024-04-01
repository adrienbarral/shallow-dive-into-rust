# Organize your project source code

You may notice that Cargo.toml file doesn't contains the list of source file. This is because the only file that cargo needs is the entry point of your crate. 
By default, the entry point for an applicatio is ```src/main.rs```, and for a library ```src/lib.rs```.

It's a common practice to avoid having file bigger than 250-500 line of code. So you will quickly need to split your project into different files.

In rust, you can either : 
* create a file in src folder (called `foo.rs`), then add `mod foo;` in your `main.rs`
* or create a folder called `foo` containing a file called `mod.rs` in your src folder and add `mod foo;` in your `main.rs`.

That means that both tree are equivalents : 
```
.
├── foo.rs
└── main.rs
```

```
.
├── foo
│   └── mod.rs
└── main.rs
```

This is recursive. You can have a file ```bar.rs``` into the foo folder, and `mod bar` into `foo/mod.rs` etc ....

Here is the [rustlang book section](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) covering this topic.