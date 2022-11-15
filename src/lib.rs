use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

type Fork = Arc<Mutex<usize>>;

const TIMEOUT: u64 = 60;

pub enum TakeFirst {
    Left,
    Right,
}

struct Philosopher {
    id: usize,
    first: Arc<Mutex<usize>>,  // первая вилка
    second: Arc<Mutex<usize>>, // вторая вилка
    count: u64,                // счетчик сколько раз философ поел
    start: Instant,
}

impl Philosopher {
    pub fn eat(&mut self) {
        loop {
            let first = self.first.lock().unwrap();
            let second = self.second.lock().unwrap();
            self.count += 1;
            drop(second);
            drop(first);
            thread::sleep(Duration::from_nanos(1));
            if self.start.elapsed().as_secs() > TIMEOUT {
                break;
            }
        }
    }

    pub fn new(id: usize, left: &Fork, right: &Fork, t: TakeFirst) -> Self {
        let (mut left, mut right) = (left.clone(), right.clone());
        if let TakeFirst::Right = t {
            std::mem::swap(&mut left, &mut right)
        }
        Philosopher {
            id,
            first: left.clone(),
            second: right.clone(),
            count: 0,
            start: Instant::now(),
        }
    }
}

pub fn dining<const N: usize>(f: impl Fn(usize) -> TakeFirst) {
    let forks: Vec<Fork> = (0..N).map(|i| Arc::new(Mutex::new(i))).collect();
    let phs: Vec<_> = (0..N)
        .map(|id| Philosopher::new(id, &forks[id], &forks[(id + 1) % N], f(id)))
        .map(|mut ph| {
            thread::spawn(move || {
                ph.eat();
                ph
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|th| th.join().unwrap())
        .collect();
    let sum: u64 = phs.iter().map(|ph| ph.count).sum();
    println!("Общее количество операций: {sum}");
    phs.iter()
        .for_each(|ph| println!("{}: {:.4}%", ph.id, (ph.count as f64 / sum as f64) * 100.0))
}
