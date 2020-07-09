use std::thread;
use std::time::Duration;

fn main() {
    let threads = std::env::var("THREADS")
        .unwrap_or_else(|_| "100".into())
        .parse::<usize>().unwrap();

    let crash = std::env::var("CRASH")
        .unwrap_or_else(|_| "0".into())
        .parse::<usize>().unwrap();

    println!("Running with {} threads crashing in {} loops", threads, crash);
    let mut handlers = vec![];
    for i in 0..threads{
        let handler = thread::spawn(move || for x in 0.. {
            println!("Thread {} Running", i);
            thread::sleep(Duration::from_secs(1));
            if crash > 0 && crash < x {
                panic!("at the disco {}", x);
            }
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
}
