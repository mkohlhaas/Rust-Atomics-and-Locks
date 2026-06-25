// scoped-threads
// https://mara.nl/atomics/basics.html#scoped-threads

use std::thread;

fn main() {
  // Thread Builder
  {
    let builder = thread::Builder::new()
      .name("my_thread".into())
      .stack_size(32 * 1024);

    let handler = builder.spawn(|| 42).unwrap();

    let res = handler.join().unwrap();
    println!("{res}");
  }

  println!();

  // Scoped Threads
  {
    let numbers = vec![1, 2, 3];

    // Scoped threads allow us to spawn threads which have a known lifetime,
    // so can safely borrow local variables.
    // `numbers` lives longer than the scope s. Therefore we can borrow a
    // reference to it.
    thread::scope(|s| {
      s.spawn(|| {
        println!("length: {}", numbers.len());
      });

      s.spawn(|| {
        // we can borrow reference to numbers
        // for n in numbers { // this would not be possible
        for n in &numbers {
          println!("{n}");
        }
      });
    }); // All spawned threads are joined!
  }

  // ⚠️ not allowed: readers and writers at the same time
  // {
  //   let mut numbers = vec![1, 2, 3];
  //
  //   thread::scope(|s| {
  //     s.spawn(|| {
  //       numbers.push(1);
  //     });
  //
  //     s.spawn(|| {
  //       numbers.push(2); // Error!
  //     });
  //   });
  // }

  // ⚠️ thread out-lives numbers
  // {
  //   let mut numbers: Box<[i32]> = Vec::from_iter(0..=1000).into_boxed_slice();
  //   let numbers: &mut [i32] = &mut numbers[..];
  //
  //   let t = thread::spawn(|| {
  //     let len = numbers.len() as i32;
  //     let sum = numbers.iter().sum::<i32>();
  //     sum / len
  //   });
  //   let _average = t.join().unwrap();
  // }

  println!();

  {
    // static data isn't necessarily data that lives from the start of the program
    // but to its end.
    // `forget` is similar to `leak`.
    // https://doc.rust-lang.org/std/mem/fn.forget.html
    let numbers: Box<[i32]> = Vec::from_iter(0..=1000).into_boxed_slice();
    let numbers: &mut [i32] = Box::leak(numbers);

    let t = thread::spawn(|| {
      let len = numbers.len() as i32;
      let sum = numbers.iter().sum::<i32>();
      sum / len
    });

    let average = t.join().unwrap();
    println!("Average: {average}");
  }

  println!();

  {
    // You can use Box::leak(…) to make data static.

    // static lifetime: lives as long as the program
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    // let x: &mut [i32; 3] = Box::leak(Box::new([1, 2, 3])); // ⚠️

    let t1 = thread::spawn(move || dbg!(x));
    let t2 = thread::spawn(move || dbg!(x));

    t1.join().unwrap();
    t2.join().unwrap();
  }
}
