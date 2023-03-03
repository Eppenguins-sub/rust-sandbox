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
        Box::new(self.split_whitespace().flat_map(str::parse::<T>)).collect::<Vec<T>>()
    }
}

// ############# Main program starts here #############

// Merge two vectors which is sorted in decending order
fn merge_sorted<T: Ord + Copy>(vec1: &Vec<T>, vec2: &Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(vec1.len() + vec2.len());
    let mut iter1 = vec1.iter().peekable();
    let mut iter2 = vec2.iter().peekable();
    loop {
        let (val, mut_iter) = {
            match (iter1.peek(), iter2.peek()) {
                (Some(&&val1), Some(&&val2)) if val1 > val2 => (val1, &mut iter1),
                (Some(&&val1), None) => (val1, &mut iter1),
                (_, Some(&&val2)) => (val2, &mut iter2),
                (None, None) => break,
            }
        };
        mut_iter.next();
        if result.len() == 0 || val < *result.last().unwrap() { result.push(val) }
    }
    result
}

// Inclusive start, exclusive end
fn crisis_safe(crises: &Vec<usize>, start: usize, end: usize) -> bool {
    crises.partition_point(|&x| x < start) == crises.partition_point(|&x| x < end)
}

fn run_dp(crises: Vec<usize>) -> bool {
    const EMPTY_VEC: Vec<usize> = Vec::new();
    const DP_LEN: usize = 200001;
    let mut dp = [EMPTY_VEC; DP_LEN];

    // Initialization
    dp[0].push(1);
    if crisis_safe(&crises, 1, 2) { dp[2].push(2) }
    for i in 1..dp.len() {
        // Confirm dp[i]
        if crises.binary_search(&(i-1)).is_ok() {
            continue;
        }
        let new_v = merge_sorted(&dp[i], &dp[i-1]);

        // Propagate
        for level in &new_v {
            if !crisis_safe(&crises, i+level, i+level*2) { continue; }
            if let Some(elem) = dp.get_mut(i+level*2) { 
                elem.push(level+1);
            } else {
                return true;
            }
        }
        dp[i] = new_v;
    }
    return false;
}

fn main() {
    let _n: usize = next_line().parse().unwrap();
    
    if run_dp(next_line().split_parse()) {
        println!("YES");
    } else {
        println!("NO");
    }
}

