use std::io;
use std::io::Read;
use std::io::BufRead;

fn quicksort(numbers: &mut Vec<isize>) {

}

fn main() {
    let mut line = String::with_capacity(500_000); // FIX ME!
    io::stdin().lock().read_line(&mut line);
    
    let mut values: Vec<isize> = line // PRE ALLOCATE!
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<isize>().unwrap())
        .collect();
    
    let length = values.len() as isize; // O(1) OPERATION

    // Sort
    quicksort(&mut values);

    // Output
    let mut output = String::with_capacity(500_000);
    for value in &values {
        output.push_str(&value.to_string());
        output.push_str(" ");
    }

    println!("{}", output);
}
