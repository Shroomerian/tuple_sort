/*!
Double sort works by separating the elements into pairs of two called nodes, this allows
to order the two elements within the node extremely fast since it's a 1:1 memory swap.

Grouping elements by nodes allows a simple loop statement to order the whole vector in n/2 - 1 reads maximum by utilizing BinaryHeaps

# Usage

```toml
[dependencies]
double_sort = "1.0.0"
```
*/

#[cfg(debug_assertions)]
use std::time::Instant;
use core::fmt::Debug;

#[derive(PartialEq,PartialOrd,Eq,Debug,Clone, Copy,Ord)]
struct Node<T>(T,Option<T>); 

impl<T: PartialOrd + Copy> Node<T> {
    //Rearranges the node in the correct order
    fn order(&mut self) {
        if let Some(_number) = self.1 {
            switch(&mut self.0,self.1.as_mut().unwrap());
        }
    }

    //Informs if there is None present in the structure
    fn none_present(&self) -> bool {
        if self.1 == None {
            true
        } else {
            false
        }
    }

    fn contains(&self,element:T) -> bool {
        if element == self.0 {
            true
        } else {
            false
        }
    }

    //Returns the node in form of a vector slice
    fn slices(&self) -> Vec<T> {
        let mut vector = Vec::new();
        vector.push(self.0);
    
        if let Some(number) = self.1 {
            vector.push(number);
        }
    
        vector
    }
}

//Checks if the left element is the smallest and if not then swaps with the right element so they are ordered from left to right.
fn switch<T: PartialOrd>(left: &mut T,right: &mut T) -> bool {
    if left > right {
        std::mem::swap(left,right);
        return true;
    } else {
    false
    }
}

///Sorts a vector of elements by pairing them and ordering them by the lowest number, then exchanging elements with neighbours until sorted.
/// # Example 

///```rust
///use double_sort::double_sort;

///fn main() {
///    let mut vector = vec![48,23,78,67,89,22,33,44];
///  
///    double_sort(&mut vector);
/// 
///    assert_eq!(vector,[22,23,33,44,48,67,78,89]);
///}
///```
pub fn double_sort<T: Copy + Ord>(list: &mut Vec<T>) {

    #[cfg(debug_assertions)] 
    let total = Instant::now();

    if list.len() <= 2 {

        if list.len() == 1 {
            return;
        }

        let mut node = Node(list[0],Some(list[1]));
        node.order();

        list.clear();
        list.append(&mut node.slices());

        #[cfg(debug_assertions)]
        println!("Elapsed time: {:.2?}",total.elapsed());
        
        return;
    }

    //Mutable values used to control the while loop
    let mut counter = 0; //Amount of times the loop ran for
    let mut nothing = 0; //Amount of times nothing was done on a read

    #[cfg(debug_assertions)]
    let chunks = Instant::now();

    let mut vector = Vec::new();

    let iter = list.chunks_exact(2);
    let mut node: Node<T>;
    let temp_node: Option<Node<T>>;

    if !iter.remainder().is_empty() {
        temp_node = Some(Node(iter.remainder()[0],None));

        temp_node.unwrap().order();
    } else {
        temp_node = None;
    }

    for chunk in iter {
        node = Node(chunk[0],Some(chunk[1]));

        node.order();

        vector.push(node);
    }

    if let Some(_element) = temp_node {
        vector.push(temp_node.unwrap());
    }
    

    let mut reference_vec = Vec::new();
    let mut temp_vec = Vec::new();

    for node in &vector {
        temp_vec.push(node.0);
    }

    double_sort(&mut temp_vec);

    for reference in temp_vec {
        let left_node = *vector.iter().find(|x| x.contains(reference) == true).unwrap();

        reference_vec.push(left_node);
    }

    vector = reference_vec;

    
    #[cfg(debug_assertions)]
    println!("Time creating nodes: {:.2?}",chunks.elapsed());

    //Clears the list so it can be given the sorted slices
    list.clear();


    #[cfg(debug_assertions)]
    let loops = Instant::now();

    //Final sort of the values by comparing left and right values of neighbouring nodes
    loop {

        let mut left = vector[counter];

        if counter == vector.len() - 1 {
            list.append(&mut left.slices());
            break;
        }

        let mut right = vector[counter+1];

        let switched: bool; //Checks whether anything was changed

        if let Some(_number) = left.1 {
            switched = switch(left.1.as_mut().unwrap(),&mut right.0);
        } else {
            switched = switch(&mut left.0,&mut right.0);
        }

        if !switched {
            vector[counter] = left;
            list.append(&mut left.slices());

            //Increment the times where read did nothing
            nothing += 1;
            counter += 1;

            if right.none_present() {
                vector[counter] = right;
                list.append(&mut right.slices());

                if counter == vector.len() - 1 {
                    break;
                }

                if counter == vector.len() - 2 {
                    let left = vector[counter+1];

                    list.append(&mut left.slices());

                    //Info dump
                    #[cfg(debug_assertions)]
                    {
                        println!("Total reads done: {}",counter);
                        println!("Total number of memory switches: {}", counter - nothing);
                    }

                    break;
                }
                
            }
            continue;
        }

        left.order();
        right.order();

        vector[counter] = left;
        vector[counter+1] = right;
        //Increment counter
        counter += 1;

        if counter == vector.len() - 1 {
            list.append(&mut left.slices());
            list.append(&mut right.slices());
            break;
        }

        //Everything is pushed back into the heap so nothing is lost.
        list.append(&mut left.slices());

        let mut temp_vec = Vec::new();
        let mut reference_vec = Vec::new();

        for node in &vector {
            temp_vec.push(node.0);
        }
    
        double_sort(&mut temp_vec);
    
        for reference in temp_vec {
            let left_node = *vector.iter().find(|x| x.contains(reference) == true).unwrap();
    
            reference_vec.push(left_node);
        }
    
        vector = reference_vec;

    }

    #[cfg(debug_assertions)]
    println!("Time looping: {:.2?}",loops.elapsed());

    #[cfg(debug_assertions)]
    println!("Total function time: {:.2?}",total.elapsed());

    //Info dump
    #[cfg(debug_assertions)]
    {
        println!("Total reads done: {}",counter);
        println!("Total number of memory switches: {}", counter - nothing);
    }

}

///Works in the same way that [double_sort] does but utilizes [BinaryHeaps](std::collections::BinaryHeap). This provides a faster but less logarithmic sort since
/// it only shaves half [heap.pop()](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.pop) runs.
/// # Note
/// This was the previous [double_sort] in 1.0.0
/// 
///# Example 
/// 
///```rust
///use double_sort::double_heap_sort;
/// 
///fn main() {
///    let mut vector = vec![48,23,78,67,89,22,33,44];
///  
///    double_heap_sort(&mut vector);
/// 
///    assert_eq!(vector,[22,23,33,44,48,67,78,89]);
///}
///```
/// 
/// # Debug Assertion Output
/// 
/// This output will only be shown if you have debug enabled
/// 
///```text
///Time creating nodes: 4.09µs
///Time looping: 4.33µs
///Total function time: 14.37µs
///Total reads done: 3
///Total number of memory switches: 2
///
pub fn double_heap_sort<T: Copy + Ord>(list: &mut Vec<T>) {

    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    #[cfg(debug_assertions)] 
    let total = Instant::now();
    
    if list.len() <= 2 {
        let mut node = Node(list[0],list.get(1).cloned());
        node.order();

        list.clear();
        list.append(&mut node.slices());

        #[cfg(debug_assertions)]
        println!("Elapsed time: {:.2?}",total.elapsed());
        
        return;
    }

    //BinaryHeap is used due to it's efficient ordering capabilities.
    let mut heap = BinaryHeap::new();

    //Mutable values used to control the while loop
    let mut counter = 0; //Amount of times the loop ran for
    let mut nothing = 0; //Amount of times nothing was done on a read

    #[cfg(debug_assertions)]
    let chunks = Instant::now();

    let iter = list.chunks_exact(2);
    let mut node: Node<T>;

    if !iter.remainder().is_empty() {
        node = Node(iter.remainder()[0],None);

        node.order();
        heap.push(Reverse(node));
    }

    for chunk in iter {
        node = Node(chunk[0],Some(chunk[1]));

        node.order();
        heap.push(Reverse(node));
    }

    #[cfg(debug_assertions)]
    println!("Time creating nodes: {:.2?}",chunks.elapsed());

    //Clears the list so it can be given the sorted slices
    list.clear();


    #[cfg(debug_assertions)]
    let loops = Instant::now();

    //Final sort of the values by comparing left and right values of neighbouring nodes
    loop {
        let mut left = heap.pop().unwrap().0;

        if heap.is_empty() {
            list.append(&mut left.slices());
            break;
        }

        let mut right = heap.pop().unwrap().0;

        let switched: bool; //Checks whether anything was changed

        if let Some(_number) = left.1 {
            switched = switch(left.1.as_mut().unwrap(),&mut right.0);
        } else {
            switched = switch(&mut left.0,&mut right.0);
        }

        if !switched {
            list.append(&mut left.slices());

            //Increment the times where read did nothing
            nothing += 1;
            counter += 1;

            if right.none_present() {
                list.append(&mut right.slices());

                if heap.len() == 1 {
                    list.append(&mut heap.pop().unwrap().0.slices());

                    //Info dump
                    #[cfg(debug_assertions)]
                    {
                        println!("Total reads done: {}",counter);
                        println!("Total number of memory switches: {}", counter - nothing);
                    }

                    break;
                }
                
            } else {
                heap.push(Reverse(right));
            }

            continue;
        }

        left.order();
        right.order();

        //Increment counter
        counter += 1;

        if heap.is_empty() {
            list.append(&mut left.slices());
            list.append(&mut right.slices());
            break;
        }

        //Everything is pushed back into the heap so nothing is lost.
        list.append(&mut left.slices());
        heap.push(Reverse(right));

    }

    #[cfg(debug_assertions)]
    println!("Time looping: {:.2?}",loops.elapsed());

    #[cfg(debug_assertions)]
    println!("Total function time: {:.2?}",total.elapsed());

    //Info dump
    #[cfg(debug_assertions)]
    {
        println!("Total reads done: {}",counter);
        println!("Total number of memory switches: {}", counter - nothing);
    }
}