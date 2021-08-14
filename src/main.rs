use std::{num::Wrapping, sync::mpsc::channel, time::Instant};
struct Work {
    how_much: i32,
    data: Vec<Wrapping<u32>>
}

#[inline(never)]
fn do_work(mut work: Work) -> Work {
    for _ in 0..work.how_much {
        for i in 0..work.data.len() {
            let sum = work.data.iter().sum::<Wrapping<u32>>();
            work.data[i] += sum;
        }
    }
    work
}

fn main() {
    let w = Work{ how_much: 1, data: vec![Wrapping(1); 1000] };
    let mut w = do_work(w);
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let j = std::thread::spawn(move || {
        loop { tx2.send(do_work(rx1.recv().unwrap())); }
    });

    let cs = 100000;
    let start = Instant::now();
    for _ in 0..cs {
        tx1.send(w);
        w = do_work(rx2.recv().unwrap());
    }

    let end = Instant::now();
    println!("Took {}", (end - start).as_secs_f32());

    let start = Instant::now();
    for _ in 0..(cs*2) {
        w = do_work(w);
    }

    let end = Instant::now();
    println!("Single Took {}", (end - start).as_secs_f32());

    println!("Hello, world! {}", w.data[0]);
}
