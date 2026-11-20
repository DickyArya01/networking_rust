use std::thread;

fn _try_spawn_thread() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("spawn {}", i);
        }
    });

    for i in 1..5 {
        println!("main {}", i);
    }

    handle.join().unwrap();
}

fn try_ownership_spawn() {
    let numbers = vec![1, 2, 3, 4, 5];

    //not adding move will result in fail, it's because numbers is not moved to spawn closure
    let handle = thread::spawn(|| {
        // let handle = thread::spawn(move || {
        for number in numbers {
            println!("{:?}", number)
        }
    });

    handle.join().unwrap();
}

fn main() {
    try_ownership_spawn();
}
