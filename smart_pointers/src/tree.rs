use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

pub fn test_weak_ref_tree(){

    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    // After leaf is created, its Rc<Node> has a strong count of 1 and a weak count of 0.
    println!();
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );


    { // custom scope

        // In the inner scope, we create branch and associate it with leaf,
        // at which point when we print the counts, the Rc<Node> in branch
        // will have a strong count of 1 and a weak count of 1
        // (for leaf.parent pointing to branch with a Weak<Node>).
        // When we print the counts in leaf, we’ll see it will have a strong count of 2,
        // because branch now has a clone of the Rc<Node> of leaf stored in branch.children,
        // but will still have a weak count of 0.

        let branch = Rc::new(Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!();
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!();
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }


    // When the inner scope ends, branch goes out of scope and the strong count
    // of the Rc<Node> decreases to 0, so its Node is dropped.
    // The weak count of 1 from leaf.parent has no bearing on whether or not Node is dropped,
    // so we don’t get any memory leaks!

    println!();
    println!("After scope ");
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // By specifying that the relationship from a child to its parent should be a Weak<T> reference
    // in the definition of Node, you’re able to have parent nodes point to child nodes
    // and vice versa without creating a reference cycle and memory leaks.

}