use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i))
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delims.1
        );
    }
}

impl<Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Progress {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(mut self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            i: self.i,
            iter: self.iter,
            bound,
        }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        self.bound.display(&self);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1))
}

fn main() {
    let brkts = ('<', '>');

    let v = vec![1, 2, 3];

    for n in v.iter().progress().with_bound().with_delims(brkts) {
        expensive_calculation(n)
    }

    // for n in (0..).progress() {
    //     expensive_calculation(&n)
    // }

    // use std::collections::HashSet;
    // let mut h = HashSet::new();
    // h.insert(0);

    // for n in h.iter().progress().with_bound() {
    //     expensive_calculation(n)
    // }
}
