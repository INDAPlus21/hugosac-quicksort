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

fn partition(numbers: &mut Vec<isize>, start: usize, end: usize) -> isize {
    let pivot = numbers[end];
    // println!("Pivot: {}", pivot);
    // let mut i: isize = start as isize - 1;
    // for j in start..end {
    //     if numbers[j] < pivot {
    //         i += 1;
    //         numbers.swap(i as usize, j);
    //     }
    // }
    // numbers.swap((i + 1) as usize, end);
    // return i + 1;

    let mut left = start;
    let mut right = end;


    loop {
        while left < right && numbers[left] <= pivot {
            left += 1;
        }
        while right > left && numbers[right] >= pivot {
            right -= 1;
        }

        if left < right {
            numbers.swap(left, right);
        }

        if left >= right {
            break;
        }
    }

    numbers.swap(left, end);

    return left as isize;
}

fn quicksort(numbers: &mut Vec<isize>, start: isize, end: isize) {
    if start >= end { return; }

    if end - start < 10 {
        insertionsort(numbers, start as usize, end as usize);
        return;
    }
    
    // Get pivot index and rearrange
    let pivot_index = partition(numbers, start as usize, end as usize);
    // println!("Pivot index: {}", pivot_index);
    // println!("{:?}\n", numbers);

    // Left of pivot
    quicksort(numbers, start, pivot_index - 1);

    // Right of pivot
    quicksort(numbers, pivot_index + 1, end);
}

fn main() {
    // Input
    let mut line = String::with_capacity(2_000_000);
    io::stdin().lock().read_line(&mut line);
    
    let mut values: Vec<isize> = line
        .split_whitespace()
        .skip(1) // Skip length param
        .map(|_value| _value.parse::<isize>().unwrap())
        .collect();
    
    let length = values.len() as isize;

    // let mut values: Vec<isize> = vec![8, 7, 2, 1, 0, 9, 6];
    // let length = values.len() as isize; 
    // println!("Start: {:?}\n", values);

    // Sort
    quicksort(&mut values, 0, length - 1);

    // Output
    let mut output = String::with_capacity(line.len());
    for value in values {
        output.push_str(&value.to_string());
        output.push_str(" ");
    }
    println!("{}", output);
}
