// refcell

use std::cell::RefCell;

fn f(v: &RefCell<Vec<i32>>) {
  v.borrow_mut().push(1); // we can modify the `Vec` directly.
}

fn main() {
  let v = RefCell::new(vec![1, 2, 3]);
  f(&v);

  assert_eq!(v.into_inner(), vec![1, 2, 3, 1]);
}
