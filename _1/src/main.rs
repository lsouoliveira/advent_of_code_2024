use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    let mut sum = 0;

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let lines = input.split('\n');

    for line in lines {
        let numbers: Vec<&str> = line.split(' ').filter(|&x| !x.is_empty()).collect();

        first_list.push(numbers[0].parse().unwrap());
        second_list.push(numbers[1].parse().unwrap());
    }

    first_list.sort();
    second_list.sort();

    for i in 0..first_list.len() {
        sum += (second_list[i] - first_list[i]).abs();
    }

    println!("{}", sum);
}
