#![allow(unused)]
//sequences types
//-----------------
// Vec -> a growable sequence of elements. dynamically resizable array or vector
// VecDeque -> double ended queue (not always contiguous in memory)implemented as a growable array
// LinkedList -> doubly-linked list containing a series of nodes

//maps
//-------
// hashmaps -> key - value store
// Btreemap -> map optimized for search.  A map implemented as a balanced binary tree.

//sets
//--------
// Hashset -> a hashmap but with no duplicates - A hash set for storing a collection of unique values
// BtreeSet -> betreemap for sets. A hash set for storing a collection of unique value

//misc
//-------
// binary heap -- priority queue implementation

// Primitives types
// Arrays -> fixed size, contiguos in memory, objects of the same type are 0 base

// Tuples -> finite (contains different types sequensce of elementes) objs of different types

// Slices -> views into a block of memory, mutable o shared, dynamically sized. A dynamically sized view into contiguous memory, which can be a part of a Vec or an array.

//Range: A sequence of numbers between two endpoints, often used in loops.
//BitSet: A collection of bits, implemented as an array of bits.

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
};

fn main() {
    let mut my_vec = vec![1, 2, 3];

    let my_array = [1, 2, 6, 3];

    // Sort characters
    my_vec.sort();

    // Remove duplicates
    my_vec.dedup();

    my_vec.push(9);

    let str_sequence = String::from("Hello");

    let mut to_be_cleared = String::from("this gone");

    // Delete values in a string if mutable
    to_be_cleared.clear();

    let idx: usize = 0; // we can not use i32
    println!("{}", my_vec[idx]);

    match my_vec.get(6) {
        Some(x) => println!("{}", x),
        None => println!("out of index"),
    }

    // for dat in my_vec {
    //     println!("{}", dat);
    // }

    //println!("{:?}", my_vec);

    // use iter() to avoid the borrowed values from vector
    // for dat in my_vec.iter() {
    //     println!("{}", dat + 1);
    // }

    // println!("{:?}", my_vec);

    // for dat in my_vec.iter_mut() {
    //     *dat += 10;  // here use * to dereference the variable and update the value where the dat is pointing to
    //     println!("{}", dat);
    // }
    // or
    for dat in &mut my_vec {
        *dat += 10; // here use * to dereference the variable and update the value where the dat is pointing to
        println!("{}", dat);
    }

    let mut coffe = HashMap::new();
    coffe.insert("latte", 10);
    coffe.insert("Mocka", 17);
    coffe.insert("simple", 10);

    // this will not update the existing value for that existing key
    coffe.entry("latte").or_insert(44);

    for (name, value) in &coffe {
        println!("name ->{name}, vlaue->{value}");
    }
    // Iterate over hashmap
    for (k, v) in coffe.iter() {
        println!("{} = {}", k, v);
    }

    // Check for key in hashmap
    if coffe.contains_key(&"latte") {
        println!("we have latte");
    }

    //pay attetion to this example!!!
    // we want to count the frecuency of the values in a vec
    // Use the .entry() method to insert a value if nothing is found.
    let vect = vec![2, 1, 5, 4, 2, 1, 4, 2, 4];
    let mut frec_vec = HashMap::new();
    for i in &vect {
        let mut frecu = frec_vec.entry(*i).or_insert(0);
        *frecu += 1; // this is a mutable reference to the value obtained in the previos sentence
    }
    println!("fecuency of values -> {:?}", frec_vec);

    let tup = (1, 2, "something");

    
    let my_array: [u8; 4] = [1, 2, 6, 3];

    for val in my_array {
        println!("{}", val);
    }

    println!("value {:?}", my_array[2]);

    let arr = [1, 2, 3];

    let arr_with_som_val = [33u16; 15]; // it will create anarray of 15 position with 33 value

    //let matrix [[i32;2];3]

    println!("{:?}", arr); // use the :? to print the all arrr, with the debug mark

    let mys_slice = &my_array[1..3]; // 2,6,3

    let sentence = "The fox jumps over the dog";

    println!("THE SENTENCE --> {}", sentence);

    let index = sentence.find("fox");

    println!("Index: {:#?}", index);

    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..1];

    println!("{:#?}", slice);

    // let words_after_fox = &sentence[index..]; // Error: Can't index str with Option<usize>
    if let Some(ind) = index {
        // assign to  ind the value of index That could be Some or None
        let words_after_fox = &sentence[ind..];
        println!("this is after the index --> {}", words_after_fox);
    }
}

fn vec_deque() {
    // Create a new Array
    let arr = [1, 2, 3];
    // Create a new Tuple
    let tup = (1, 2, 3);
    // Create a new Slice
    let slce = &arr[1..2];
    // Create a new Range
    let rang = 1..20;

    // Create a new Vec
    let mut my_vec = vec![1, 200, 150];
    // Create a new VecDeque
    let mut deque: VecDeque<i32> = VecDeque::new();
    // Create a new LinkedList
    let mut list: LinkedList<i32> = LinkedList::new();

    my_vec.push(600);
    // Create a new HashMap
    let mut hashmap: HashMap<String, i32> = HashMap::new();
    // Create a new BTreeMap
    let mut btreemap: BTreeMap<String, i32> = BTreeMap::new();

    // Create a new HashSet
    let mut hash_set: HashSet<String> = HashSet::new();
    // Create a new BTreeSet
    let mut btree_set: BTreeSet<String> = BTreeSet::new();

    // Create a new BinaryHeap
    let mut binary_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
}
