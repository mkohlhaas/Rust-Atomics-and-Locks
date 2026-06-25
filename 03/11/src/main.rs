// fence
// https://mara.nl/atomics/memory-ordering.html#fences

#![allow(unused_imports)]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::atomic::fence;
use std::thread;
use std::time::Duration;

// NOTE: Ordering Summary
// https://youtu.be/C5xY96i0Aes?t=1561
// Release-Acquire        -> happens-before
// Sequentially consisten -> happens-before & global order
// Relaxed                -> any relative order (no order guarantees)

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn some_calculation(i: usize) -> u64 {
  thread::sleep(Duration::from_millis(400 + i as u64 % 3 * 100));
  123
}

fn main() {
  for i in 0..10 {
    thread::spawn(move || {
      let data = some_calculation(i);
      unsafe { DATA[i] = data };
      READY[i].store(true, Release);
    });
  }

  thread::sleep(Duration::from_millis(500));

  // These Relaxed loads could turn into Acquire's bc of the fence that follows later.
  let ready: [bool; 10] = std::array::from_fn(|idx| READY[idx].load(Relaxed));

  if ready.contains(&true) {
    // A fence is not tied to any single atomic variable.
    // Every Relaxed load before this fence turns into an Acquire.
    // (In our case there is only one such location.)
    fence(Acquire);

    for i in 0..10 {
      if ready[i] {
        // will always set data{i} to 123
        println!("data{i} = {}", unsafe { DATA[i] });
      }
    }
  }

  // ⚠️
  // NOTE:
  // IF   thread A stores an (atomic) value with Ordering::Release
  // AND  thread B reads this value with Ordering::Acquire
  // THEN A synchronizes with thread B.

  // NOTE:
  // That doesn't mean that thread B will necessarily see the update from thread A!
  // All it means is, if thread B sees the updated value then thread A and B synchronize.

  // NOTE: on "synchronize"
  // All memory writes that happened before the atomic store in thread A become
  // visible side-effects in thread B.

  // Make sure all threads have finished (except main thread).
  // thread::sleep(Duration::from_millis(1000));

  // NOTE: Whenever you have true, you must have a value of 123 in DATA.
  println!("{:?}", &READY); // ⚠️
  println!("{:?}", unsafe { DATA }); // ⚠️

  println!("Done!")
}
