use std::sync::Arc;

const NUM_LOOP: usize = 100000;

mod fairlock;

fn main() {
    let lock = Arc::new(fairlock::FairLock::new(0));
    let mut v = Vec::new();

    for i in 0..fairlock::NUM_LOCK {
        let lock0 = lock.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let mut data = lock0.lock(i);
                *data += 1;
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})",
        *lock.lock(0),
        NUM_LOOP * fairlock::NUM_LOCK
    );
}
