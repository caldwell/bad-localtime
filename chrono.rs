use chrono::prelude::*;

fn main() {
    let j = std::thread::spawn(|| {
        for i in 0..910000 {
            std::env::set_var(format!("TEST{}", i), i.to_string());
        }
    });

    for i in 0..910000 {
        println!("{} - {}", i, Local::now());
    }

    j.join().unwrap();
}

