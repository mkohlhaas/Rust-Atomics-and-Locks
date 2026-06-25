### Overview

| Struct     | Send | Sync | Documentation                                             |
| ------     | ---- | ---- | -------------                                             |
| Rc         |  -   |   -  | https://doc.rust-lang.org/std/rc/struct.Rc.html           |
| Cell       |  ✓   |   -  | https://doc.rust-lang.org/std/cell/struct.Cell.html       |
| RefCell    |  ✓   |   -  | https://doc.rust-lang.org/std/cell/struct.RefCell.html    |
| UnsafeCell |  ✓   |   -  | https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html |
| Mutex      |  ✓   |   ✓  | https://doc.rust-lang.org/std/sync/struct.Mutex.html      |
| RwLock     |  ✓   |   ✓  | https://doc.rust-lang.org/std/sync/struct.RwLock.html     |
| Arc        |  ✓   |   ✓  | https://doc.rust-lang.org/std/sync/struct.Arc.html        |
| Atomic     |  ✓   |   ✓  | https://doc.rust-lang.org/std/sync/atomic/                |
| CondVar    |  ✓   |   ✓  | https://doc.rust-lang.org/std/sync/struct.Condvar.html    |

Often you just pack everything up in an Arc to get [Send and Sync semantics](https://doc.rust-lang.org/std/sync/struct.Condvar.html#examples).


## Send, Sync

```
  - Send -> can send  T
  - Sync -> can send &T
```

```
  - Data that is Send can be sent   to other threads.
  - Data that is Sync can be shared between  threads.
```

```
  - &T     is a  shared    reference.
  - &mut T is an exclusive reference.
```


| Struct     | Description                                                      | Mutability                                                                                      |
| ------     | -----------                                                      | ----------                                                                                      |
| Rc         | A single-threaded reference-counting pointer.                    | Shared references in Rust disallow mutation by default. Use interior mutability.                |
| Cell       | A mutable memory location.                                       | get, set, take, replace, update, swap, …                                                        |
| RefCell    | A mutable memory location with dynamically checked borrow rules. | borrow, borrow_mut, replace, replace_with, take, swap, …                                        |
| UnsafeCell | The core primitive for interior mutability in Rust.i             | get, get_mut, replace, …
| Mutex      | A mutual exclusion primitive useful for protecting shared data.  | lock, get_mut, …                                                                                |
| RwLock     | A reader-writer lock.                                            | read, write, …                                                                                  |
| Arc        | A thread-safe reference-counting pointer.                        | Shared references in Rust disallow mutation by default. Use interior mutability.                |
| Atomic     | Provide primitive shared-memory communication between threads.   | atomic load and store operations, fetch-and-modify operations, compare-and-exchange operations  |
| CondVar    | A Condition Variable.                                            | Not Applicable                                                                                  |
