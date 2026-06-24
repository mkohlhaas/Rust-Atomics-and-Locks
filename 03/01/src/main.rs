// relaxed
// https://mara.nl/atomics/memory-ordering.html#happens-before

// NOTE: Memory ordering only matters when there are multiple shared variables with a dependeny!
// https://youtu.be/C5xY96i0Aes?t=1360

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
  X.store(10, Relaxed);
  Y.store(20, Relaxed);
}

fn b() {
  let y = Y.load(Relaxed);
  let x = X.load(Relaxed);
  println!("{x} {y}");
}

fn main() {
  thread::scope(|s| {
    s.spawn(a);
    s.spawn(b);
  });
}
