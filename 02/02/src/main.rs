// progress-reporting
// Atomic Load and Store Operations
// https://mara.nl/atomics/atomics.html#example-progress-reporting

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn process_item(n: usize) {
  println!("Working on {n}");
  thread::sleep(Duration::from_millis(100));
}

fn main() {
  let num_done = AtomicUsize::new(0);

  thread::scope(|s| {
    // A background thread to process all 100 items.
    s.spawn(|| {
      for i in 0..100 {
        process_item(i); // assuming this takes some time
        num_done.store(i + 1, Relaxed);
      }
    });

    // The main thread shows status updates, every second.
    loop {
      let n = num_done.load(Relaxed);
      if n == 100 {
        break;
      }
      println!("Working.. {n}/100 done");
      thread::sleep(Duration::from_secs(1));
    }
  });

  println!("Done!");
}
