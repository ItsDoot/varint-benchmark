#![feature(test)]
extern crate test;

use std::io::{self, Write};

mod blended;
mod bungee;
mod lucky5;
mod velocity;

fn main() {
    println!("Run with 'cargo bench'");
}

trait VarIntWrite {
    fn write<W: Write>(writer: W, value: i32) -> io::Result<()>;
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::blended::BlendedVarIntWrite;
    use crate::bungee::BungeeVarIntWrite;
    use crate::lucky5::Lucky5VarIntWrite;
    use crate::VarIntWrite;
    use crate::velocity::VelocityVarIntWrite;

    fn numbers() -> impl Iterator<Item=i32> {
        (0..i32::MAX / 2048).map(|v| v * 2047)
    }

    #[bench]
    fn blended(b: &mut Bencher) {
        let mut bytes = test::black_box(Vec::<u8>::new());

        b.iter(|| {
            for n in numbers() {
                BlendedVarIntWrite::write(&mut bytes, n);
                bytes.clear();
            }
        })
    }

    #[bench]
    fn bungee(b: &mut Bencher) {
        let mut bytes = test::black_box(Vec::<u8>::new());

        b.iter(|| {
            for n in numbers() {
                BungeeVarIntWrite::write(&mut bytes, n);
                bytes.clear();
            }
        })
    }

    #[bench]
    fn lucky5(b: &mut Bencher) {
        let mut bytes = test::black_box(Vec::<u8>::new());

        b.iter(|| {
            for n in numbers() {
                Lucky5VarIntWrite::write(&mut bytes, n);
                bytes.clear();
            }
        })
    }

    #[bench]
    fn velocity(b: &mut Bencher) {
        let mut bytes = test::black_box(Vec::<u8>::new());

        b.iter(|| {
            for n in numbers() {
                VelocityVarIntWrite::write(&mut bytes, n);
                bytes.clear();
            }
        })
    }
}