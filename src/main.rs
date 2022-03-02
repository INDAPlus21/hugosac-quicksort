use std::io;
use std::io::BufRead;

fn insertionsort(numbers: &mut Vec<isize>, start: usize, end: usize) {
    for i in start + 1..end + 1 {
        let key = numbers[i];
        let mut j = i;

        while j > start && key < numbers[j - 1] {
            numbers[j] = numbers[j - 1];
            j -= 1;
        }
        numbers[j] = key;
    }
}

fn partition(numbers: &mut Vec<isize>, start: isize, end: isize) -> isize {
    // Hoare's Partition Scheme using middle index as pivot
    let pivot = numbers[((start + end) / 2) as usize];
    let mut i = start - 1;
    let mut j = end + 1;

    loop {
        i += 1;
        while numbers[i as usize] < pivot {
            i += 1;
        }
        j -= 1;
        while numbers[j as usize] > pivot {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        numbers.swap(i as usize, j as usize);
    }
}

fn quicksort(numbers: &mut Vec<isize>, start: isize, end: isize) {
    if start >= end { return; }

    if end - start < 15 {   // Insertion sort on smaller arrays
        insertionsort(numbers, start as usize, end as usize);
        return;
    }
    
    // Get pivot index and rearrange
    let pivot_index = partition(numbers, start, end);

    // Left of pivot
    quicksort(numbers, start, pivot_index as isize);

    // Right of pivot
    quicksort(numbers, (pivot_index + 1) as isize, end);
}

fn main() -> io::Result<()> {
    // Input
    let mut line = String::with_capacity(1_000_000);
    io::stdin().lock().read_line(&mut line)?;
    
    let mut values: Vec<isize> = line
        .split_whitespace()
        .skip(1) // Skip length param
        .map(|_value| _value.parse::<isize>().unwrap())
        .collect();
    
    let length = values.len() as isize;

    quicksort(&mut values, 0, length - 1);

    // Output
    let mut output = String::with_capacity(line.len());
    for value in values {
        output.push_str(&value.to_string());
        output.push_str(" ");
    }
    println!("{}", output);
    Ok(())
}
