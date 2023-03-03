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
struct Crises {
    data: Vec<usize>
}

impl Crises {
    fn fetch() -> Crises {
        Crises { data: next_line().split_parse() }
    }

    // Inclusive start, exclusive end
    fn crisis_safe(&self, start: usize, end: usize) -> bool {
        self.data.partition_point(|&x| x < start) == self.data.partition_point(|&x| x < end)
    }

    fn next_crisis(&self, t: usize) -> usize {
        self.data.partition_point(|&x| x < t)
    }

    fn get(&self, i: usize) -> usize {
        self.data[i]
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn last_crisis_chunk(&self, i: usize, level: usize) -> usize {
        self.data.partition_point(|&x| x < self.data[i]+level) - 1
    }
}

#[derive(Debug, Clone)]
struct TimeData {
    t: usize,
    ind: usize,
}

struct VecCache<T: Copy + Ord> {
    cache: Vec<Option<T>>,
    cached_indexes: Vec<usize>,
}

impl<T: Copy + Ord> VecCache<T> {
    fn with_capacity(n: usize) -> VecCache<T> {
        let cache = vec![None; n];
        let cached_indexes = Vec::new();
        VecCache { cache, cached_indexes }
    }

    fn update(&mut self, val: T, ind: usize) {
        match self.cache[ind] {
            Some(val_origin) => {
                self.cache[ind] = Some(cmp::min(val, val_origin))
            },
            None => {
                self.cached_indexes.push(ind);
                self.cache[ind] = Some(val);
            },
        };
    }

    fn iter(&self) -> impl Iterator<Item = (T, usize)> + '_ {
        self.cached_indexes.iter().map(|&i| (self.cache[i].unwrap(), i))
    }
}

fn run_dp(crises: Crises) -> bool {
    // Initialization
    let mut level = 1;
    let mut cur_times = Vec::from([TimeData{ t: 0, ind: 0 }]);

    loop {
        if cur_times.len() == 0 { break; }
        let mut next_times_cache = VecCache::with_capacity(200000);
        for &TimeData{ t, ind } in &cur_times {
            // Train
            if crises.crisis_safe(t, t + 2*level) {
                next_times_cache.update(t + 2*level, ind);
            }
            // Fight
            let abil_start = {
                let optimal_start = (crises.get(crises.last_crisis_chunk(ind, level))+1).saturating_sub(level);
                cmp::max(t, optimal_start)
            };
            if crises.crisis_safe(abil_start + level, abil_start + 2*level) {
                let new_t = abil_start + 2*level;
                let new_ind = crises.next_crisis(new_t);
                if new_ind >= crises.len() { return true; }
                next_times_cache.update(new_t, new_ind);
            }
        }

        cur_times = next_times_cache.iter()
                                    .map(|(t, ind)| TimeData { t, ind })
                                    .collect(); 
        level += 1;
    }
    return false;
}

fn main() {
    let _n: usize = next_line().parse().unwrap();
    
    if run_dp(Crises::fetch()) {
        println!("YES");
    } else {
        println!("NO");
    }
}
