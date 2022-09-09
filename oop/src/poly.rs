 // Polymorphism
 // To many people, polymorphism is synonymous with inheritance.
 // But it’s actually a more general concept that refers to code that can work with data of multiple types.
 // For inheritance, those types are generally subclasses.
 //
 // Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints
 // on what those types must provide. This is sometimes called bounded parametric polymorphism.
 //
 // However, sometimes we want our library user to be able to extend the set of types
 // that are valid in a particular situation.
 //
 // We’ve mentioned that, in Rust, we refrain from calling structs and enums “objects”
 // to distinguish them from other languages’ objects.
 // In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated.
 //
 // However, trait objects are more like objects in other languages
 // in the sense that they combine data and behavior.
 // But trait objects differ from traditional objects in that we can’t add data to a trait object.
 // Trait objects aren’t as generally useful as objects in other languages:
 // their specific purpose is to allow abstraction across common behavior.

 pub trait Draw {
     fn draw(&self);
 }

 pub struct Screen {
     // This vector is of type Box<dyn Draw>, which is a trait object;
     // it’s a stand-in for any type inside a Box that implements the Draw trait.
     pub components: Vec<Box<dyn Draw>>,
 }

 impl Screen {
   // This works differently from defining a struct that uses a generic type parameter with trait bounds.
   // A generic type parameter can only be substituted with one concrete type at a time,
   // whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
  pub fn run(&self) {
      for component in self.components.iter() {
          component.draw();
      }
  }
 }

 // Now we’ll add some types that implement the Draw trait.
 // We’ll provide the Button type. Again, actually implementing a GUI library
 // is beyond the scope of this book, so the draw method won’t have any useful implementation in its body

 pub struct Button {
     pub width: u32,
     pub height: u32,
     pub label: String,
 }


 impl Draw for Button {
     fn draw(&self) {
         println!("Draw Button {}", self.label)
     }
 }


 pub struct SelectBox{
     pub width: u32,
     pub height: u32,
     pub options: Vec<String>,
 }

 impl Draw for SelectBox {
     fn draw(&self) {
         println!("Draw SelectBox with options {:?}", self.options)
     }
 }