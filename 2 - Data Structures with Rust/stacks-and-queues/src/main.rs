use std::collections::VecDeque;

fn main() {

    //stacks are going to be implemented using vectors

    // my guess is vector with pop and push methods

    // you could make a struct called a Stack if you wanted to

    let mut stack = Stack { items: vec![]};

    for i in 0..9 {
        stack.push(i);
    }

    println!("{:?}", stack);
    
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    
    println!("{:?}", stack);


    let mut queue = Queue { items: VecDeque::new()};

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("{:?}", queue);

    println!("{:?}", queue.dequeue());
    println!("{:?}", queue);
}

#[derive(Debug)]
struct Stack {
    items: Vec<i32>,
}
impl Stack {
    fn push(&mut self, item: i32) {
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }
}


// so everything above here was my take on it before seeing the video's way of doing it (except for the initial setup of a struct that holds items: Vec<i32>)

/*
    in the instructor's version he also implements a peek method

    fn peek(&self) -> Option<&i32> {
        self.items.last()
    }

    
*/



/*

    So all of this could be achieved in fewer lines of code by just using a vector like a stack,

    however making the stack struct does help communicate the intention of the data structure and can be helpful in using it in other parts of the code versus just seeing that it's a vector

    this can help enforce that the vector IS used like a stack if that's what kind of data it is meant to store and way that it's supposed to work like

*/



/*

    Stacks and Queues - Part 2

    now we're going to implement a queue and I expect it will be basically the same thing except FIFO instead of LIFO

    I WAS SURPRISED TO FIND OUT THAT THERE IS A DIFFERENT KIND OF VECTOR HERE FOR THIS

    rust has a 'Vector Dequeue' type that is more efficient for dequeueing than a regular vector's remove would be

    a vector is efficient pushing and popping from the back end, but not the front,

    but a vecdeque (vector Double Ended Queue) is efficient pushing and popping from both the front and back ends (but is a little slower at random access than a vector is)

*/

#[derive(Debug)]
struct Queue {
    items: VecDeque<i32>,
}
impl Queue{
    fn enqueue(&mut self, item: i32) {
        self.items.push_back(item);
    }
    fn dequeue(&mut self) -> Option<i32> {
        self.items.pop_front()
    }

    ///something that can view the next item to dequeue without moving it
    fn peek(&mut self) -> Option<&i32> {
        self.items.front()
    }
}


/*

GitHub Copilot
Certainly! Vec and VecDeque are both collections in Rust, but they have different use cases and performance characteristics.

Vec

Vec is a growable array type, similar to arrays in other languages but with dynamic resizing capabilities.

Characteristics:

Contiguous Memory: Elements are stored in a contiguous block of memory, which provides excellent cache performance.

Fast Random Access: Accessing elements by index is very fast (O(1) time complexity).

Efficient Push/Pop at End: Adding or removing elements at the end of the vector is very efficient (amortized O(1) time complexity).

Limitations:

Slow Insert/Remove at Beginning: Inserting or removing elements at the beginning or middle of the vector requires shifting all subsequent elements, which can be slow (O(n) time complexity).

VecDeque

VecDeque (short for "vector double-ended queue") is a double-ended queue implemented with a growable ring buffer.

Characteristics:

Efficient Push/Pop at Both Ends: Adding or removing elements at both the front and the back of the deque is very efficient (O(1) time complexity).

Contiguous Memory: While not strictly contiguous like Vec, VecDeque uses a ring buffer to provide good cache performance.

Limitations:

Slower Random Access: Accessing elements by index is slightly slower compared to Vec due to the ring buffer implementation (still O(1) but with a bit more overhead).

More Complex Implementation: The internal structure is more complex, which can lead to slightly higher overhead for some operations.


When to Use Vec

Random Access: When you need fast random access to elements.

Stack-like Behavior: When you primarily push and pop elements from the end of the collection.

Simple Use Cases: When you don't need to frequently insert or remove elements from the beginning or middle of the collection.

When to Use VecDeque

Queue-like Behavior: When you need efficient push and pop operations at both ends of the collection.

Double-ended Queue: When you need a double-ended queue for algorithms that require efficient access to both ends.

Frequent Insertions/Removals: When you frequently insert or remove elements from the beginning of the collection.

*/