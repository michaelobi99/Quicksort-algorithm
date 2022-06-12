use std::fs::File;
use std::io::{BufRead, Write};
use std::time::{Instant, Duration};
struct Timer{
    start: Instant,
}
impl Timer{
    fn elapsed_time(&self)-> Duration {
        self.start.elapsed()
    }
}

fn swap(vec: &mut Vec<u32>, elem1: usize, elem2: usize)->(){
    let a = vec[elem1];
    vec[elem1] = vec[elem2];
    vec[elem2] = a;
}

fn partition(vec: &mut Vec<u32>, begin: usize, end: usize)->usize{
    let mut pivot: usize;
    let mut small_index: usize;
    swap(vec, begin.clone(), (begin + end)/2);
    pivot = vec[begin] as usize;
    small_index = begin.clone();
    for index in (begin+1)..=end{
        if vec[index] < (pivot as u32){
            small_index += 1;
            swap(vec, small_index.clone(), index.clone());
        }
    }
    swap(vec, begin.clone(), small_index.clone());
    small_index
}

fn quick_sort(vec: &mut Vec<u32>, begin: usize, end: usize)->(){
    let mut pivot;
    if begin < end{
        pivot = partition(vec, begin, end);
        quick_sort(vec, begin.clone(), pivot);
        quick_sort(vec, pivot+1, end.clone());
    }
}

fn print_numbers(vec: &Vec<u32>)->(){
    let mut file = File::create(r"C:\Users\HP\PycharmProjects\filerust.txt").unwrap();
    for elem in vec{
        let str = format!("{:<8}", elem.to_string());
        file.write(str.as_bytes());
    }
}

fn main()->Result<(), std::io::Error>{
    let file = File::open(r"C:\Users\HP\PycharmProjects\file.txt")?;
    let file = std::io::BufReader::new(file);
    let mut numbers: Vec<u32> = Vec::with_capacity(10000);
    for line in file.lines(){
        numbers.push(line?.parse::<u32>().unwrap());
        if numbers.capacity() == numbers.len(){
            numbers.reserve(10000);
        }
    }
    //let mut numbers= vec![2,5,6,8,3,9,1,7,4,12,15,16,18,13,19,10,20,17, 11, 14];
    numbers.shrink_to_fit();
    let length = numbers.len() - 1;
    let timer = Timer{start: Instant::now()};
    quick_sort( &mut numbers, 0 as usize, length);
    println!("The elapsed time = {:?}", timer.elapsed_time());
    print_numbers(&numbers);
    Ok(())
}