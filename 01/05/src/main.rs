// rc
// https://mara.nl/atomics/basics.html#arc

#![allow(unused_imports)]

use std::{rc::Rc, sync::Arc, thread};

fn f(a: &i32, b: &mut i32) {
  let before = *a;
  *b += 1;

  let after = *a;

  if before != after {
    println!("not the same"); // never happens
  } else {
    println!("the same");
  }
}

fn main() {
  // Rc (single threads)
  {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    println!("{a:?}");
    println!("{b:?}");

    println!();

    println!("{:p}", a);
    println!("{:p}", b);

    // ⚠️ `Send` trait is not implemented for Rc's
    // thread::spawn(move || dbg!(a));

    assert_eq!(a.as_ptr(), b.as_ptr()); // same allocation!
  }

  println!();

  // Arc (thread-safe)
  {
    let a = Arc::new([4, 5, 6]);
    let b = a.clone();

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
  }

  println!();

  // https://mara.nl/atomics/basics.html#naming-clones

  // The clone of the Arc lives in the same scope.
  // Each thread gets its own clone with a different name.
  {
    let a = Arc::new([7, 8, 9]);
    let b = a.clone();

    let j = thread::spawn(move || {
      dbg!(b);
    });

    j.join().unwrap();

    dbg!(a);
  }

  println!();

  // Better:
  // The clone of the Arc lives in a different scope.
  // We can use the same name in each thread.
  {
    let a = Arc::new([9, 10, 11]);

    let j = thread::spawn({
      let a = a.clone();
      move || {
        dbg!(a);
      }
    });

    j.join().unwrap();

    dbg!(a);
  }

  // Arc's and Rc's are read-only
  {
    // let a: Arc<[i32]> = Arc::new([1, 2, 3]);
    // a.sort(); // ⚠️
  }

  // Borrowing and Data Races
  // https://mara.nl/atomics/basics.html#borrowing-and-races
  {
    let a = 42;
    let mut b = 42;

    f(&a, &mut b);
  }

  // {
  //   let mut a = 42;
  //
  //   f(&a, &mut a); // ⚠️ cannot borrow `a` as mutable because it is also borrowed as immutable
  // }

  // Undefined Behavior
  // https://mara.nl/atomics/basics.html#undefined-behavior
  {
    let a = [123, 456, 789];
    println!("{}", a.len());

    let b = unsafe { a.get_unchecked(5) }; // ⚠️ panics: out of bounds
    println!("{b}");
  }
}
