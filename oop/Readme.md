# Chapter 17: Object-Oriented Programming Features of Rust

Object-oriented programming (OOP) is a way of modeling programs. 
Objects as a programmatic concept were introduced in the programming language Simula in the 1960s. 

The book Design Patterns: Elements of Reusable Object-Oriented Software by Erich Gamma, Richard Helm, Ralph Johnson, 
and John Vlissides (Addison-Wesley Professional, 1994), colloquially referred to as The Gang of Four book, 
is a catalog of object-oriented design patterns. It defines OOP this way:


Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. 
The procedures are typically called methods or operations.


Using this definition, Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs and enums. 
Even though structs and enums with methods aren’t called objects, they provide the same functionality, 
according to the Gang of Four’s definition of objects.

## Encapsulation that Hides Implementation Details

Another aspect commonly associated with OOP is the idea of encapsulation, 
which means that the implementation details of an object aren’t accessible to code using that object. 
Therefore, the only way to interact with an object is through its public API; 
code using the object shouldn’t be able to reach into the object’s internals and change data or behavior directly. 
This enables the programmer to change and refactor an object’s internals without needing to change the code that uses the object.

## Object-Oriented Programming in Rust
https://youtu.be/UGDa0P2PXrY

## Book 
https://doc.rust-lang.org/stable/book/ch17-00-oop.html