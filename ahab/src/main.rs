use std::thread;
use std::time::Duration;

fn main() {
    let threads = std::env::var("THREADS")
        .unwrap_or_else(|_| "100".into())
        .parse::<usize>().unwrap();
    println!("Running with {} threads", threads);
    let mut handlers = vec![];
    for i in 0..threads{
        let handler = thread::spawn(move || loop {
            println!("Thread {} Running", i);
            thread::sleep(Duration::from_secs(1));
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.join().unwrap();
    }
}
