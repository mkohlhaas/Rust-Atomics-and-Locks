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

#![allow(static_mut_refs)]

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
  // Release store with acquire load wouldn't work here!

  let a = thread::spawn(|| {
    A.store(true, SeqCst);
    if !B.load(SeqCst) {
      unsafe { S.push('!') };
    }
  });

  let b = thread::spawn(|| {
    B.store(true, SeqCst);
    if !A.load(SeqCst) {
      unsafe { S.push('!') };
    }
  });

  a.join().unwrap();
  b.join().unwrap();

  println!("{:?}", A);
  println!("{:?}", B);
}
