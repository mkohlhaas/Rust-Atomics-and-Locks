// progress-reporting-unpark
// Atomic Load and Store Operations
// https://mara.nl/atomics/atomics.html#synchronization

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn process_item(n: usize) {
  println!("Working on {n}");
  thread::sleep(Duration::from_millis(50));
}

fn main() {
  let num_done = AtomicUsize::new(0);

  let main_thread = thread::current();

  thread::scope(|s| {
    // A background thread to process all 100 items.
    s.spawn(|| {
      for i in 0..100 {
        process_item(i); // assuming this takes some time
        num_done.store(i + 1, Relaxed);
        main_thread.unpark(); // wake up the main thread
      }
    });

    // The main thread shows status updates.
    loop {
      let n = num_done.load(Relaxed);
      if n == 100 {
        break;
      }
      println!("Working.. {n}/100 done");
      thread::park_timeout(Duration::from_secs(1));
    }
  });

  println!("Done!");
}
