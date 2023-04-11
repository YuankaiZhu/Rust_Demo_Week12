use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
    left_fork: usize,
    right_fork: usize,
}

impl Philosopher {
    fn new(name: &str, left_fork: usize, right_fork: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    fn eat(&self, table: &Table) {
        let _left_fork = table.forks[self.left_fork].lock().unwrap();
        println!("{} picked up left fork", self.name);

        thread::sleep(Duration::from_millis(100));

        let _right_fork = table.forks[self.right_fork].lock().unwrap();
        println!("{} picked up right fork", self.name);

        println!("{} is eating", self.name);
        thread::sleep(Duration::from_millis(1000));

        println!("{} finished eating", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Aristotle", 0, 1),
        Philosopher::new("Plato", 1, 2),
        Philosopher::new("Socrates", 2, 3),
        Philosopher::new("Descartes", 3, 4),
        Philosopher::new("Spinoza", 0, 4),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = Arc::clone(&table);
            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
