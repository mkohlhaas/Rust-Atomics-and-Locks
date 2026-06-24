// spawn-join
// https://mara.nl/atomics/memory-ordering.html#spawning-and-joining

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn f() {
  let x = X.load(Relaxed);
  print!("{x}");
  assert!(x == 1 || x == 2); // cannot fail; x can be still 1 or already 2
}

fn main() {
  X.store(1, Relaxed);

  let t = thread::spawn(f); // NOTE: spawning a thread creates a happens-before relationship

  X.store(2, Relaxed);

  t.join().unwrap(); // NOTE: joining a thread creates a happens-before relationship 

  X.store(3, Relaxed);
}
