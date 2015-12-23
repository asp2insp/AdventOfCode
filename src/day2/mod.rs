use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,is_digit};

#[derive(Debug)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

fn min_three(a: u32, b: u32, c: u32) -> u32 {
    if a < b && a < c {
        a
    } else if b < c {
        b
    } else {
        c
    }
}

impl Present {
    fn new(l: &[u8], w: &[u8], h: &[u8]) -> Present {
         let l = str::from_utf8(l).unwrap().parse::<u32>().unwrap();
         let w = str::from_utf8(w).unwrap().parse::<u32>().unwrap();
         let h = str::from_utf8(h).unwrap().parse::<u32>().unwrap();

         Present {
             l: l,
             w: w,
             h: h,
         }
    }

    fn sq_ft_needed(&self) -> u32 {
        let a = self.l*self.w;
        let b = self.w*self.h;
        let c = self.l*self.h;
        2*a + 2*b + 2*c + min_three(a, b, c)
    }

    fn ribbon_needed(&self) -> u32 {
        let side = self.smallest_side();
        return 2*(side.0 + side.1) + self.l*self.w*self.h;
    }

    fn smallest_side(&self) -> (u32, u32) {
        let a = self.l*self.w;
        let b = self.w*self.h;
        let c = self.l*self.h;
        let min = min_three(a, b, c);
        match min {
            _ if min == a => (self.l, self.w),
            _ if min == b => (self.w, self.h),
            _ if min == c => (self.l, self.h),
            _ => (0, 0),
        }
    }
}

fn present(i: Input<u8>) -> U8Result<Present> {
    parse!{i;
        let l = take_while(is_digit);
                token(b'x');
        let w = take_while(is_digit);
                token(b'x');
        let h = take_while(is_digit);
                skip_whitespace();
        ret Present::new(l, w, h)
    }
}

fn all_presents(i: Input<u8>) -> U8Result<Vec<Present>> {
    parse!{i;
        let r = many1(present);
        ret r
    }
}

pub fn part1(input: String) -> String {
    let result = parse_only(all_presents, input.as_bytes());
    let total_sq_ft = result.unwrap().iter().fold(0, |sum, p| sum + p.sq_ft_needed());
	format!("{}", total_sq_ft)
}


pub fn part2(input: String) -> String {
    let result = parse_only(all_presents, input.as_bytes());
    let total_ribbon = result.unwrap().iter().fold(0, |sum, p| sum + p.ribbon_needed());
	format!("{}", total_ribbon)}
