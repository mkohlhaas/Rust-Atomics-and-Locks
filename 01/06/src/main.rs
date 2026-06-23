// cell
// https://mara.nl/atomics/basics.html#cell

use std::cell::Cell;

fn f(v: &Cell<Vec<i32>>) {
  // typical Cell code:
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
