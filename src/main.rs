// September Abbott
// Rust Program 2
// 
// Enviroment used for development: VSCode on MacOS

use rand::Rng;
use std::collections::VecDeque;
use std::io;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;

#[derive(Clone, Eq)]
// Main Struct
struct Process {
    pid: u8,
    priority: u8,
    sleep_time: u16,
    description: String,
}

// Traits
impl Process {
    fn build_process(pid: u8) -> Self {
        Self {
            pid: pid,
            priority: rand::thread_rng().gen_range(1..=100),
            sleep_time: rand::thread_rng().gen_range(100..=2000),
            description: " ".to_string(),
        }
    }
}

// Ordering for heap priority 
impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

// Printing Function
fn print_process_info(process: &Process) {
    println!("PID: {}, Priority: {}, Sleep Time: {}, Description: {}", process.pid, process.priority, process.sleep_time, process.description);
}

fn main() {
    // For user input
    let mut count:u32 = 0;

    println!("Program 2");
    println!("Enter the number of process nodes to generate: ");

    // User input
    loop {
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        if n == "quit\n" {
            println!("enter quit");
            break;
        };

        // User input error handling
        let _n: f64 = match n.trim().parse() {
            Ok(n) => {
                count = n;
                break;
            }
            Err(_) => continue,
        };
    }

    // Data Structures
    let mut vec_deque = VecDeque::new();
    let mut min_heap = BinaryHeap::new();
    
    // Push loop
    let mut process_count: u8 = 1;
    for _i in 0..count{
        // Making a process and copying it so that the queue and heap have the same processes
        let iter_process_q = Process::build_process(process_count);
        let iter_process_h = iter_process_q.clone();

        // Pushing into data structures, Using "Reverse" so that heap is a min heap
        vec_deque.push_back(iter_process_q);
        min_heap.push(Reverse(iter_process_h));
        process_count += 1
    }

    // Pop Loop for Dequeue
    println!("\nFIFO QUEUE\n");
    for _i in 0..count {
        if let Some(process) = vec_deque.pop_front() {
            print_process_info(&process);
        }
    }

    // Pop Loop for Heap
    println!("\nHEAP\n");
    for _i in 0..count {
        if let Some(process) = min_heap.pop() {
            print_process_info(&process.0);
        }
    }

    // Finished
    println!("\nGoodbye!");

}


