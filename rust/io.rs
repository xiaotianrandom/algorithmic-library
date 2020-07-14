use std::io::{self, Write};
use std::str;

// Begin of backporting of SplitAsciiWhitespace (I don't know what I'm writting here) for CodeJam
// CodeJam 2020 uses rustc 1.24.1
type SplitAsciiWhitespace = std::iter::Map<std::iter::Filter<std::slice::Split<'static, u8, for<'r> fn(&'r u8) -> bool>, for<'a, 'b> fn(&'a &'b [u8]) -> bool>, for<'a> fn(&'a [u8]) -> &'a str>;
fn is_ascii_whitespace(byte: &u8) -> bool {
    byte.is_ascii_whitespace()
}
fn bytes_is_not_empty<'a, 'b>(s: &'a &'b [u8]) -> bool {
    !s.is_empty()
}
fn unsafe_bytes_to_str<'a>(bytes: &'a [u8]) -> &'a str {
    // SAFETY: not safe
    unsafe { std::str::from_utf8_unchecked(bytes) }
}
#[inline]
pub fn split_ascii_whitespace(s: &str) -> SplitAsciiWhitespace {
    let inner = s.as_bytes().split(is_ascii_whitespace as for<'r> fn(&'r u8) -> bool).filter(bytes_is_not_empty as for<'a, 'b> fn(&'a &'b [u8]) -> bool).map(unsafe_bytes_to_str as for<'a> fn(&'a [u8]) -> &'a str);
    unsafe {
        std::mem::transmute(inner)
    }
}
// End of backporting of SplitAsciiWhitespace

struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: SplitAsciiWhitespace,
}

#[allow(dead_code)]
impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self { reader, buf_str: Vec::new(), buf_iter: split_ascii_whitespace("") }
    }
    fn next<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                split_ascii_whitespace(slice)
            }
        }
    }

    fn next_line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).expect("Failed to read line");
        line
    }

    fn next_vec<T: str::FromStr>(&mut self) -> Vec<T> {
        let mut res: Vec<T> = Vec::new();
        loop {
            loop {
                match self.buf_iter.next() {
                    Some(token) => res.push(token.parse().ok().expect("Failed parse")),
                    None => break,
                }
            }
            if res.len() > 0 {
                return res;
            }
            self.buf_str.clear();
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                split_ascii_whitespace(slice)
            }
        }
    }

    fn next_vec_n<T: str::FromStr>(&mut self, n: u32) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut cin = Scanner::new(stdin.lock());
    let mut cout = io::BufWriter::new(stdout.lock());

    let n = cin.next::<u32>();
    let v = cin.next_vec_n::<i32>(n);

    writeln!(cout, "{}: {:?}", n, v).ok();
}
