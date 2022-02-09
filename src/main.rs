use futures::StreamExt;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use futures::executor::block_on;
// use wasm_timer::Interval;
mod interval;
use interval::Interval;

fn main() {
    block_on(disrupted_interval())
}

async fn disrupted_interval() {
    let mut interval_durations = VecDeque::new();

    let mut last_tick = Instant::now();
    let mut interval = Interval::new(Duration::from_millis(100));

    let mut ticks = 0;

    while let Some(_) = interval.next().await {
        let elapsed_ms = last_tick.elapsed().as_millis() as u64;
        interval_durations.push_front(elapsed_ms);

        last_tick = Instant::now();
        ticks += 1;
        if ticks % 5 == 0 {
            std::thread::sleep(Duration::from_millis(200)); // miss the next tick
        }
        if ticks == 1_000 {
            return;
        }

        let recent_avg = {
            let recent_total: u64 = interval_durations.iter().take(5).sum();
            recent_total / (interval_durations.len().min(5) as u64)
        };
        println!(
            "rolling avg: (past 5 ticks): {:?}",
            Duration::from_millis(recent_avg)
        );
    }
}
