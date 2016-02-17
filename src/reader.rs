use std::collections::HashMap;
use std::io::{Read, Result};

pub trait Atbash<R> {
    fn atbash(self) -> AtbashReader<R>;
}

impl<R: Read> Atbash<R> for R {
    fn atbash(self) -> AtbashReader<R> {
        AtbashReader {
            inner: self,
            bytemap: build_mapping(
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                "abcdefghijklmnopqrstuvwxyz",
            )
        }
    }
}

pub struct AtbashReader<R> {
    inner: R,
    bytemap: HashMap<u8, u8>
}

impl<R: Read> Read for AtbashReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let ret = self.inner.read(buf);
        for address in buf.iter_mut() {
            *address = *self.bytemap.get(&address).unwrap_or(address)
        }
        ret
    }
}

fn build_mapping(upper: &str, lower: &str) -> HashMap<u8, u8> {
    let upper: Vec<_> = upper.chars().map(|c| c as u8).collect();
    let lower: Vec<_> = lower.chars().map(|c| c as u8).collect();

    let upper = upper.iter().zip(upper.iter().rev()).map(|(&a, &b)| (a, b));
    let lower = lower.iter().zip(lower.iter().rev()).map(|(&a, &b)| (a, b));

    upper.chain(lower).collect()
}
