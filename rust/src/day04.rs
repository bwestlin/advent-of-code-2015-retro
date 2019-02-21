#[macro_use] extern crate lazy_static;
extern crate utils;
//extern crate md5;
extern crate rayon;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use rayon::prelude::*;
use utils::*;

type Input = String;

fn is_advent_coin(input: &Input, leading_zeroes: usize, i: usize) -> bool {
    //let digest = md5::compute(format!("{}{}", input, i));
    let digest = md5::digest(format!("{}{}", input, i));
    for j in 0..leading_zeroes {
        if ((0x0F << ((j + 1) % 2) * 4) & digest[j >> 1]) > 0 {
            break;
        }
        if j == leading_zeroes - 1 {
            return true;
        }
    }
    false
}

fn find_advent_coin(input: &Input, leading_zeroes: usize) -> usize {
    let n_per_thread = 10000;
    let n_threads = 8;

    for i in (1..).step_by(n_per_thread * n_threads) {
        let ranges: Vec<_> = (0..n_threads)
            .map(|ti| (i + (ti * n_per_thread))..(i + (ti * n_per_thread) + n_per_thread))
            .collect();
        let maybe_coins: Vec<usize> = ranges.into_par_iter()
            .map(|r| {
                for j in r {
                    if is_advent_coin(input, leading_zeroes, j) {
                        return Some(j);
                    }
                }
                None
            })
            .flatten()
            .collect();

        if maybe_coins.len() > 0 {
            return *maybe_coins.iter().min().unwrap();
        }
    }

    // for i in 1.. {
    //     if is_advent_coin(input, leading_zeroes, i) {
    //         return i;
    //     }
    // }
    0
}

fn part1(input: &Input) -> usize {
    find_advent_coin(input, 5)
}

fn part2(input: &Input) -> usize {
    find_advent_coin(input, 6)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().map(|l| l.unwrap()).collect())
}

mod md5 {
    type Digest = [u8; 16];

    pub fn digest<T: AsRef<[u8]>>(data: T) -> Digest {
        lazy_static! {
            // s specifies the per-round shift amounts
            static ref s: [u32; 64] = [7_u32, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
                    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
                    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
                    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21];

            static ref K: [u32; 64] = {
                let mut x = [0_u32; 64];
                for i in 0..64 {
                    x[i] = (2_f64.powi(32) * ((i as f64 + 1.0).sin()).abs()).floor() as u32;
                }
                println!("Running lazy static");
                x
            };
        }


        let mut data = data.as_ref().to_vec();
        data.reserve(64);

        // for i in (0..64).step_by(4) {
        //     println!("K[{:2}..{:2}] := {{ 0x{:08x}, 0x{:08x}, 0x{:08x}, 0x{:08x} }}", i, i + 3, K[i], K[i + 1], K[i + 2], K[i + 3]);
        // }

        // Padding
        let dlen: u64 = (data.len() * 8) as u64;
        // println!("dlen={}", dlen);
        let mut n_fill: i32 = 55 - ((data.len() % 64) as i32);
        if n_fill < 0 {
            n_fill += 64;
        }
        // println!("n_fill={}", n_fill);
        data.push(0x1 << 7);
        for i in 0..(n_fill - 0) {
            data.push(0);
        }
        for i in 0..8 {
            //data.push(0);
            data.push(((dlen >> (i * 8)) & 0xff) as u8);
            //data.push(((dlen >> ((7 - i) * 8)) & 0xff) as u8);
        }
        //data[63] = 1;

        // for i in 0..data.len() {
        //     println!("[{:02}] = {:02x} {:08b} {}", i, data[i], data[i], data[i]);
        // }

        // let foo = 123_u32;
        // println!("foo={:032b} !foo={:032b}", foo, !foo);

        let mut a0: u32 = 0x67452301;
        let mut b0: u32 = 0xefcdab89;
        let mut c0: u32 = 0x98badcfe;
        let mut d0: u32 = 0x10325476;

        //let leftrotate = |x: u32, c: u32| (x << c) | (x >> (32 - c));

        for i in (0..data.len()).step_by(64) {
            // break chunk into sixteen 32-bit words M[j], 0 ≤ j ≤ 15
            let mut M = [0_u32; 16];
            for j in 0..16 {
                let di = i + j * 4;
                // OK enligt ""
                M[j] = ((data[di] as u32) << 0) | ((data[di + 1] as u32) << 8) | ((data[di + 2] as u32) << 16) | ((data[di + 3] as u32) << 24);
                // println!("M[{:2}] = 0x{:08x}", j, M[j]);
            }

            //Initialize hash value for this chunk:
            let mut A = a0;
            let mut B = b0;
            let mut C = c0;
            let mut D = d0;
            //Main loop:
            for j in 0..64 {
                let mut F: u32;
                let mut g: u32;
                if j < 16 {
                    F = (B & C) | ((! B) & D);
                    g = j;
                } else if j < 32 {
                    F = (D & B) | ((! D) & C);
                    g = (5 * j + 1) % 16;
                } else if j < 48 {
                    F = B ^ C ^ D;
                    g = (3 * j + 5) % 16;
                } else {
                    F = C ^ (B | (! D));
                    g = (7 * j) % 16;
                }
                //Be wary of the below definitions of a,b,c,d
                F = F + A + K[j as usize] + M[g as usize];
                A = D;
                D = C;
                C = B;
                //B = B + leftrotate(F, s[j as usize]);
                B = B + ((F << s[j as usize]) | F >> (32 - s[j as usize]));
            }
            //Add this chunk's hash to result so far:
            a0 = a0 + A;
            b0 = b0 + B;
            c0 = c0 + C;
            d0 = d0 + D;
        }

        [
            (a0 >>  0) as u8,
            (a0 >>  8) as u8,
            (a0 >> 16) as u8,
            (a0 >> 24) as u8,

            (b0 >>  0) as u8,
            (b0 >>  8) as u8,
            (b0 >> 16) as u8,
            (b0 >> 24) as u8,

            (c0 >>  0) as u8,
            (c0 >>  8) as u8,
            (c0 >> 16) as u8,
            (c0 >> 24) as u8,

            (d0 >>  0) as u8,
            (d0 >>  8) as u8,
            (d0 >> 16) as u8,
            (d0 >> 24) as u8
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_digest(in_hex: &str) -> [u8; 16] {
        let mut digest = [0_u8; 16];
        for i in (0..in_hex.len()).step_by(2) {
            digest[i / 2] = u8::from_str_radix(&in_hex[i..=(i+1)], 16).unwrap();
        }
        digest
    }

    #[test]
    fn test_as_digest() {
        assert_eq!(as_digest("d41d8cd98f00b204e9800998ecf8427e"), [0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e]);
    }

    #[test]
    fn test_md5_digest() {
        assert_eq!(md5::digest(""), as_digest("d41d8cd98f00b204e9800998ecf8427e"));
        assert_eq!(md5::digest("abcdefghij"), as_digest("a925576942e94b2ef57a066101b48876"));
        assert_eq!(md5::digest("01234567890123456789012345678901234567890123456789012345"), as_digest("8af270b2847610e742b0791b53648c09"));
        assert_eq!(md5::digest("0123456789012345678901234567890123456789012345678901234567890123"), as_digest("7f7bfd348709deeaace19e3f535f8c54"));
        assert_eq!(md5::digest("The quick brown fox jumps over the lazy dog"), as_digest("9e107d9d372bb6826bd81d3542a419d6"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&"abcdef".into()), 609043);
        assert_eq!(part1(&"pqrstuv".into()), 1048970);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&"abcdef".into()), 6742839);
        assert_eq!(part2(&"pqrstuv".into()), 5714438);
    }
}