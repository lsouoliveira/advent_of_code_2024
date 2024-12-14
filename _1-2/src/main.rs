use std::io;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let lines = input.split('\n');

    for line in lines {
        let numbers: Vec<&str> = line.split(' ').filter(|&x| !x.is_empty()).collect();

        first_list.push(numbers[0].parse().unwrap());
        second_list.push(numbers[1].parse().unwrap());
    }

    let mut second_list_ocurrences = HashMap::new();

    for number in &second_list {
        if !(second_list_ocurrences.contains_key(number)) {
            second_list_ocurrences.insert(number, 1);
        } else {
            match second_list_ocurrences.get(number) {
                Some(n) => { second_list_ocurrences.insert(number, n + 1); },
                _ => ()
            }
        }
    }

    let mut sum = 0;

    for number in &first_list {
        match second_list_ocurrences.get(number) {
            Some(n) => { sum += number * n; },
            _ => ()
        }
    }

    println!("Sum: {}", sum)
}

