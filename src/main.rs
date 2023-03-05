use std::{
    io,
    str::FromStr,
};

// ############# Presets for input handling #############
fn next_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let trimmed_len = input_line.as_mut_str().trim_end().len();
    input_line.truncate(trimmed_len);
    input_line
}

trait SplitParse<T: FromStr> {
    fn split_parse(&self) -> Vec<T>;
}

impl<T: FromStr> SplitParse<T> for String {
    fn split_parse(&self) -> Vec<T> {
        self.split_whitespace().flat_map(str::parse::<T>).collect::<Vec<T>>()
    }
}

// ############# Main program starts here #############
fn mul_mod(a: usize, b: usize, divisor: usize) -> usize {
    (a * b) % divisor
}

fn pow(base: usize, exp: usize, divisor: usize) -> usize {
    if exp == 0 { 1 }
    else if exp % 2 == 0 {
        let half_result = pow(base, exp/2, divisor);
        mul_mod(half_result, half_result, divisor)
    } else {
        mul_mod(base, pow(base, exp-1, divisor), divisor)
    }
}

/// f(n+2, m) = {(m-1) * (m)^2 * (m+1)^2 * ... * (m+n-2)^2 * (m+n-1)} / {n! * (n+1)!}
/// for n >= 1. 
fn solve(mut n: usize, m: usize) -> usize {
    if n == 2 { return 2 }
    n -= 2;

    const DIV: usize = 1000000007;

    let nominator = {
        let mut nominator = 1;
        for p in m..(m+n-1) {
            let mul_val = mul_mod(p, p, DIV);
            nominator = mul_mod(nominator, mul_val, DIV);
        }
        let mul_val = mul_mod(m-1, m+n-1, DIV);
        nominator = mul_mod(nominator, mul_val, DIV);
        nominator
    };

    let denominator = {
        let mut denominator = 1;
        for p in 2..(n+1) {
            let mul_val = mul_mod(p, p, DIV);
            denominator = mul_mod(denominator, mul_val, DIV);
        }
        denominator = mul_mod(denominator, n+1, DIV);
        pow(denominator, DIV-2, DIV)
    };

    mul_mod(2*nominator, denominator, DIV)
}

fn main() {
    let [n, m] = {
        let [n, m]: [usize] = next_line().split_parse()[..] else { panic!() };
        if n > m { [m, n] }
        else { [n, m] }
    };

    println!("{}", solve(n, m));
    
}