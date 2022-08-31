# Chapter 7: Packages, Crates, and Modules

Workspaces store interrelated packages in one single place.

When calling cargo new, a new package will be created.
A package stores one or more crates. 
A crate be either a binary or a library. 
A crate consists of modules.
Module organizes code. 
Note, modules do not correspond to files. 
Instead, files must be declared in receiving modules.

## Definitions

* Packages - A Cargo feature that lets you build, test, and share crates.
* Crates - A tree of modules that produces a library or executable.
* Modules and use - Let you control the organization, scope, and privacy of paths.
* Paths - A way of naming an item, such as a struct, function, or module.

Creating a new package with a binary crate

```bash 
$ cargo new my-project
```

Creating a new package with a library crate

```bash 
$ cargo new my-project --lib
```


Defining and using modules
```rust
fn some_function() {}

mod outer_module { // private module
    pub mod inner_module { // public module
        pub fn inner_public_function() {
            super::super::some_function();
    } // inner mod 

    fn inner_private_function() {}
    }
}
```


```rust
fn main() {
    // absolute path
    crate::outer_module::
    inner_module::inner_public_function();

    // relative path path
    outer_module::
    inner_module::inner_public_function();

    // bringing path into scope
    use outer_module::inner_module;
    inner_module::inner_public_function();
}
```

Src: 

Book:
https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

Video:
Rust's Module System Explained!
https://youtu.be/5RPXgDQrjio

Rust Modules - Explained Like I'm 5
https://youtu.be/969j0qnJGi8 