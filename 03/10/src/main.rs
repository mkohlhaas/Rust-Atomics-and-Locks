// seqcst
// https://mara.nl/atomics/memory-ordering.html#seqcst

// https://youtu.be/C5xY96i0Aes?t=1204

// Sequentially Consistent Ordering includes all the guarantees of acquire ordering (for loads) and release ordering (for stores),
// and also guarantees a GLOBALLY CONSISTENT ORDER of operations.

// While it might seem like the easiest memory ordering to reason about, SeqCst ordering is almost
// never necessary in practice. In nearly all cases, regular acquire and release ordering suffice.

// It is advisable to see SeqCst as a WARNING SIGN. Seeing it in the wild often means that either
// something complicated is going on, or simply that the author did not take the time to analyze
// their memory ordering related assumptions, both of which are reasons for extra scrutiny.

#![allow(static_mut_refs, unused_imports)]

use std::sync::atomic::AtomicBool;
use std::time::Duration;
use std::{
  sync::atomic::Ordering::{Acquire, Release, SeqCst},
  thread,
};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
  // Release store with acquire load wouldn't work here!

  // ⚠️ Code is wrong!
  // Every now and then you get an unchanged S (empty String).
  // And no print of "It's me A/B".
  // But A == B == true.
  let a = thread::spawn(|| {
    A.store(true, SeqCst);
    if !B.load(SeqCst) {
      println!("It's me A!");
      unsafe { S.push('!') };
    }
  });

  let b = thread::spawn(|| {
    B.store(true, SeqCst);
    if !A.load(SeqCst) {
      println!("It's me B!");
      unsafe { S.push('!') };
    }
  });

  // ⚠️ Same here! (As expected according to the book.)
  // let a = thread::spawn(|| {
  //   // we want to access S
  //   A.store(true, Release);
  //   // checking if other thread also wants to access S
  //   // There is nothing before B on store (see in the other thread).
  //   // So it doesn't see the current B necessarily.
  //   if !B.load(Acquire) {
  //     println!("It's me A!");
  //     unsafe { S.push('!') };
  //   }
  // });

  // let b = thread::spawn(|| {
  //   // we want to access S
  //   B.store(true, Release);
  //   // checking if other thread also wants to access S
  //   // There is nothing before B on store (see in the other thread).
  //   // So it doesn't see the current A necessarily.
  //   if !A.load(Acquire) {
  //     println!("It's me B!");
  //     unsafe { S.push('!') };
  //   }
  // });

  a.join().unwrap();
  b.join().unwrap();

  println!("{:?}", A);
  println!("{:?}", B);

  println!("Sleeping for a while!");
  thread::sleep(Duration::from_millis(1000));

  unsafe {
    println!("{:?}", S);
  }
}
