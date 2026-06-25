// cell
// https://mara.nl/atomics/basics.html#cell

// Interior Mutability
// Allows data mutation through immutable references (e.g. Rc, Arc, …)
//
// Think of:
// &T     -> shared    reference (instead of immutable reference)
// &mut T -> exclusive reference (instead of mutable   reference)
// If T is an interior mutable type, then the data under bots of these is somewhat mutable.

// Cell
//  - single-threaded only (not thread-safe)
//  - .get()
//  - .set(…)

// RefCell
//  - single-threaded only (not thread-safe) TODO: ???
//  - .borrow()
//  - .borrow_mut()

// UnsafeCell is the building block for other data types, e.g. Cell, Mutex, … (just look at the
// source code)

use std::cell::Cell;

fn f(v: &Cell<Vec<i32>>) {
  // Typical Cell code (no one-liners):
  // 1. get
  // 2. mutate
  // 3. set

  // 1. get
  let mut v2 = v.take(); // replaces the contents of the Cell with an empty Vec
  println!("{:?}", v2);

  // 2. mutate
  v2.push(1);
  println!("{:?}", v2);

  // 3. set
  v.set(v2); // put the modified Vec back
}

// Cell has interior mutability
fn g(a: &Cell<i32>, b: &Cell<i32>) {
  let before = a.get();

  b.set(b.get() + 1); // ⚠️ a and b could be the same

  let after = a.get();

  if before != after {
    println!("not the same");
  } else {
    println!("the same");
  }
}

fn main() {
  {
    let v = Cell::new(vec![1, 2, 3]);
    f(&v);

    assert_eq!(v.into_inner(), vec![1, 2, 3, 1]);
  }

  {
    let a = Cell::new(42);
    let b = Cell::new(43);

    g(&a, &b);
  }

  {
    let a = Cell::new(42);

    g(&a, &a);
  }
}
