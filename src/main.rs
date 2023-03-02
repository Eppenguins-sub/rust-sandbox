use std::{
    io,
    str::FromStr,
    cmp,
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

// Let x_1 + x_2 + ... + x_i = PS[i]
// DP[i] = max_{j <= i} (DP[j-1] + a(PS[i]-PS[j-1])^2 + b(PS[i]-PS[j-1]) + c)
//       = max_{j <= i} ((-2aPS[i])*PS[j-1] + (DP[j-1]+aPS^2[j-1]-bPS[j-1]) + (aPS^2[i]+bPS[i]) + c)
//       = max_{j <= i} (c1*x + c0) + d
// Since PS[j-1] monotonically increases, we can use the convex hull trick.

struct DataEnv {
    dp: Vec<i64>,  // 1-based indexing
    ps: Vec<i64>,  // 1-based indexing
    a: i64,
    b: i64,
    c: i64,
}

impl DataEnv {
    fn fetch() -> DataEnv {
        // Input handling
        let [a, b, c]: [i64] = next_line().split_parse()[..] else { panic!() };
        let xs: Vec<i64> = {
            let mut xs_temp = Vec::from([0]);
            let mut xs_temp2 = next_line().split_parse();
            xs_temp.append(&mut xs_temp2);
            xs_temp
        };
        let ps = {
            let mut ps_temp = Vec::from([0]);
            for x in xs.iter().skip(1) {
                ps_temp.push(ps_temp.last().unwrap() + x);
            }
            ps_temp
        };
        let dp: Vec<i64> = Vec::from([0]);

        DataEnv { dp, ps, a, b, c }
    }

    fn get_c1(&self, j: usize) -> i64 {
        self.ps[j-1]
    }

    fn get_c0(&self, j: usize) -> i64 {
        self.dp[j-1] + self.a * self.ps[j-1].pow(2) - self.b * self.ps[j-1]
    }

    fn get_d(&self, i: usize) -> i64 {
        self.a * self.ps[i].pow(2) + self.b * self.ps[i] + self.c
    }

    fn get_x(&self, i: usize) -> i64 {
        -2 * self.a * self.ps[i]
    }
}

// For Finite case, always self.denom > 0.
#[derive(Clone, Copy)]
enum Fraction {
    NegInfinity,
    Finite { nom: i64, denom: i64 },
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NegInfinity, Self::NegInfinity) => true,
            (Self::Finite { nom: p1, denom: q1 }, Self::Finite { nom: p2, denom: q2 }) => {
                let lhs: i128 = *p1 as i128 * *q2 as i128;
                let rhs: i128 = *p2 as i128 * *q1 as i128;
                lhs == rhs
            }
            (_, _) => false,
        }
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (Self::NegInfinity, Self::NegInfinity) => Some(cmp::Ordering::Equal),
            (Self::NegInfinity, _) => Some(cmp::Ordering::Less),
            (_, Self::NegInfinity) => Some(cmp::Ordering::Greater),
            (Self::Finite { nom: p1, denom: q1 }, Self::Finite { nom: p2, denom: q2 }) => {
                let lhs: i128 = *p1 as i128 * *q2 as i128;
                let rhs: i128 = *p2 as i128 * *q1 as i128;
                lhs.partial_cmp(&rhs)
            },
        }
    }
}

impl PartialEq<i64> for Fraction {
    fn eq(&self, other: &i64) -> bool {
        self.eq(&Fraction::Finite { nom: *other, denom: 1 })
    }
}

impl PartialOrd<i64> for Fraction {
    fn partial_cmp(&self, other: &i64) -> Option<cmp::Ordering> {
        self.partial_cmp(&Fraction::Finite { nom: *other, denom: 1 })
    }
}

impl Fraction {
    fn new(mut nom: i64, mut denom: i64) -> Fraction {
        if denom == 0 { panic!() }
        if denom < 0 {
            nom *= -1;
            denom *= -1;
        }
        Fraction::Finite { nom, denom }
    }
}

struct CHTData {
    max_idx: Vec<usize>,  // 1-based indexing
    max_bdry: Vec<Fraction>,
}

impl CHTData {
    fn new() -> CHTData {
        CHTData { max_idx: Vec::from([1]), max_bdry: Vec::from([Fraction::NegInfinity]) }
    }

    fn last(&self) -> Option<(usize, Fraction)> {
        match (self.max_idx.last(), self.max_bdry.last()) {
            (Some(idx), Some(bdry)) => Some((*idx, *bdry)),
            _ => None,
        }
    }

    fn pop(&mut self) -> Option<(usize, Fraction)> {
        self.max_idx.pop().and_then(|x| Some((x, self.max_bdry.pop().unwrap())))
    }

    fn push(&mut self, (idx, bdry): (usize, Fraction)) {
        self.max_idx.push(idx);
        self.max_bdry.push(bdry);
    }

    // Find maximum i s.t. max_bdry[i] <= x
    // and return max_idx[i].
    fn max_j(&self, x: i64) -> usize {
        let ceil = |x, y| x/y + cmp::min(1, x%y);
        let mut start = 0;
        let mut end = self.max_idx.len() - 1;
        while start < end {
            let mid = ceil(end-start, 2) + start;
            if self.max_bdry[mid] <= x {
                start = mid;
            } else {
                end = mid-1;
            }
        }
        self.max_idx[start]
    }
}

fn main() {
    let n: usize = next_line().parse().unwrap();
    let mut data_pack = DataEnv::fetch();
    let mut cht_data = CHTData::new();

    for i in 1..(n+1) {
        // Update dp
        let x = data_pack.get_x(i);
        let d = data_pack.get_d(i);

        let j = cht_data.max_j(x);
        let c1 = data_pack.get_c1(j);
        let c0 = data_pack.get_c0(j);
        data_pack.dp.push(c1*x + c0 + d);

        // Update cht_data
        loop {
            match cht_data.last() {
                Some((cur_j, cur_bdry)) => {
                    let new_c1 = data_pack.get_c1(i+1);
                    let new_c0 = data_pack.get_c0(i+1);
                    let cur_c1 = data_pack.get_c1(cur_j);
                    let cur_c0 = data_pack.get_c0(cur_j);

                    let intsct_x = Fraction::new(cur_c0 - new_c0, new_c1 - cur_c1);
                    if intsct_x < cur_bdry {
                        cht_data.pop();
                    } else {
                        cht_data.push((i+1, intsct_x));
                        break
                    }
                },
                None => break,
            }
        }
    }

    println!("{}", data_pack.dp.last().unwrap());
}