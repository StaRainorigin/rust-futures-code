
fn main() {
}


use std::{thread, time::Duration};
#[test]
fn test1() {
    println!("So we start the program here!");
    let t1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        println!("We create tasks which gets run when they're finished!");
    });

    let t2 = thread::spawn(move ||{
        thread::sleep(Duration::from_millis(100));
        println!("We can even chain callbacks...");
        let t3 = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            println!(",..like this!");
        });
        t3.join().unwrap();
    });
    println!("While our tasks are executing we can do other stuff here.");

    t1.join().unwrap();
    t2.join().unwrap();
}


use std::ptr;
use std::thread::Thread;

const DEFAULT_STACK_SIZE: usize = 1024*1024*2;
const MAX_THREADS: usize = 4;
static mut RUNTIME: usize = 0;



pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}