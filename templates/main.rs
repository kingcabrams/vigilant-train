#![allow(dead_code, non_snake_case)]
use std::{io::{Read, Write, BufWriter}, collections::*, str::SplitAsciiWhitespace};

type Map<K, V> = BTreeMap<K, V>;
type Set<V> = BTreeSet<V>;
type Queue<V> = VecDeque<V>;

fn solve<W: Write>(stdin: &mut SplitAsciiWhitespace, stdout: &mut BufWriter<W>) {
}

fn main() { let mut stdin = String::new(); let _ = std::io::stdin().read_to_string(&mut stdin); let mut stdin = stdin.split_ascii_whitespace(); let stdout = std::io::stdout(); let mut stdout = std::io::BufWriter::new(stdout.lock()); solve(&mut stdin, &mut stdout); }

trait ChangeMinMax { 
    fn chmin(&mut self, x: Self) -> bool; 
    fn chmax(&mut self, x: Self) -> bool; 
} 

impl<T: PartialOrd> ChangeMinMax for T { 
    fn chmin(&mut self, x: Self) -> bool { 
        *self > x && { *self = x; true } 
    } 
    fn chmax(&mut self, x: Self) -> bool { 
        *self < x && { *self = x; true } 
    } 
}

trait BinarySearch<T: PartialOrd> {
    fn lower_bound(&self, value: &T) -> usize;
    fn upper_bound(&self, value: &T) -> usize;
}

impl<T: PartialOrd> BinarySearch<T> for [T] {
    fn lower_bound(&self, value: &T) -> usize {
        let mut l = 0;
        let mut r = self.len();

        while l < r {
            let m = l + (r-l) / 2;
            if self[m] < *value {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }

    fn upper_bound(&self, value: &T) -> usize {
        let mut l = 0;
        let mut r = self.len();
        while l < r {
            let m = l + (r-l) / 2;
            if self[m] > *value {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

use::std::ops::Rem;
trait Gcd {
    fn gcd(&self, b: Self) -> Self;
}

impl<T: Rem<Output = T> + Default + PartialEq + Copy> Gcd for T {
    fn gcd(&self, mut other: Self) -> Self {
        let mut a = *self;

        while other != T::default() {
            let temp = other;
            other = a % other;
            a = temp;
        }
        a
    }
}

// reference: https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
#[macro_export]
macro_rules! input {
    (source = $iter:expr, $($r:tt)*) => {
        input_inner!{$iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_ascii_whitespace();
        input_inner!{iter, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, bytes) => {
        read_value!($iter, String).bytes().collect::<Vec<u8>>()
    };
    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

trait Print {
    fn print<W: Write>(&self, out: &mut BufWriter<W>);
}

macro_rules! impl_print_collection {
    ($($t:ident),*) => {
        $(
            impl<T: std::fmt::Display> Print for $t<T> {
                fn print<W: Write>(&self, out: &mut BufWriter<W>) {
                    for item in self.iter() {
                        write!(out, "{} ", item).unwrap();
                    }
                    writeln!(out).unwrap();
                }
            }
        )*
    };
}

impl_print_collection!(Vec, Queue, Set);

impl Print for bool {
    fn print<W: Write>(&self, out: &mut BufWriter<W>) {
        writeln!(out, "{}", if *self { "YES" } else { "NO" }).unwrap();
    }
}

impl<T: std::fmt::Display> Print for Option<T> {
    fn print<W: Write>(&self, out: &mut BufWriter<W>) {
        if let Some(x) = self {
            writeln!(out, "{}", x).unwrap();
        } else {
            writeln!(out, "-1").unwrap();
        }
    }
}

macro_rules! impl_print {
    ($($t:ty),*) => {
        $(
            impl Print for $t {
                fn print<W: Write>(&self, out: &mut BufWriter<W>) {
                    writeln!(out, "{}", self).unwrap();
                }
            }
        )*
    };
}

impl_print!(isize, i32, i64, i128, usize, u32, u64, u128, f32, f64, String, &str);
