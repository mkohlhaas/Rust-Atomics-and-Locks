// progress-reporting-multiple-threads
// Fetch-and-Modify Operations
// https://mara.nl/atomics/atomics.html#example-progress-reporting-from-multiple-threads

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn process_item(n: usize) {
  println!("Processing item {n}");
  thread::sleep(Duration::from_millis(200));
}

fn main() {
  // AtomicUsize is not COPY. We use a reference.
  // Shared references (&T) are Copy.
  // https://doc.rust-lang.org/std/marker/trait.Copy.html
  let num_done = &AtomicUsize::new(0);

  thread::scope(|s| {
    // Four background threads to process all 100 items, 25 each.
    for t in 0..4 {
      s.spawn(move || {
        for i in 0..25 {
          process_item(t * 25 + i); // assuming this takes some time
          // ⚠️ this won't work as values are overwritten by the different threads!
          // {
          //   let a = num_done.load(Relaxed);
          //   // here could happen updates from other threads (non-atomic)
          //   num_done.store(a + 1, Relaxed);
          // }

          // fetch_… is atomic!
          num_done.fetch_add(1, Relaxed);
        }
      });
    }

    // The main thread shows status updates.
    loop {
      let n = num_done.load(Relaxed);
      if n == 100 {
        break;
      }
      println!("Working.. {n}/100 done");
      thread::sleep(Duration::from_millis(250));
    }
  });

  println!("Done!");
}
