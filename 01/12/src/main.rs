// condvar
// https://mara.nl/atomics/basics.html#condvar

use std::collections::VecDeque;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// Condvars are used with Mutexes.

fn main() {
  let queue = Mutex::new(VecDeque::new());
  let not_empty = Condvar::new();

  thread::scope(|s| {
    s.spawn(|| {
      loop {
        let mut mutex_guard = queue.lock().unwrap();
        let item = loop {
          if let Some(item) = mutex_guard.pop_front() {
            break item;
          } else {
            // queue is empty
            // `wait` takes a mutex guard and then unlocks the Mutex, waits for notification, and then relocks the Mutex
            // returns the locked mutex guard
            mutex_guard = not_empty.wait(mutex_guard).unwrap(); // blocks the current thread until this condition variable receives a notification
          }
        };
        drop(mutex_guard);
        dbg!(item);
      }
    });

    for i in 0.. {
      queue.lock().unwrap().push_back(i);
      if i % 5 == 0 {
        // Wakes up one blocked thread on this condvar.
        // Producer notifies when queue is not empty.
        not_empty.notify_one();
      }
      thread::sleep(Duration::from_millis(200));
    }
  });
}
