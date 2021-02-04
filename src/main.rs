use md5::{Md5, Digest};
use rand::Rng;
use std::cmp;
use threadpool::ThreadPool;
use std::process;
use num_cpus;

fn brute() {
    let mut hasher = Md5::new();
    let mut rng = rand::thread_rng();

    let seed = rng.gen::<u128>();

    println!("seed: {:032x}",seed);
    hasher.update(format!("{:032x}", seed));

    let mut max: String = "fffffffff00000000000000000000000".to_string();
    let mut min: String = "000000000fffffffffffffffffffffff".to_string();
    let mut match_threshold: u16 = 10;

    let mut result = hasher.finalize();

    loop {
        let mut hasher = Md5::new();

        let previous = result;

        hasher.update(format!("{:032x}", result));
        result = hasher.finalize();

        // check if fixed point
        if result == previous{
            println!("{:032x}", result);
            process::exit(0);
        }

        // check if new max hash
        if format!("{:032x}", result) > max {
            println!("max x:      {:032x}", previous);
            println!("max md5(x): {:032x}", result);
        }
        max = cmp::max(max,format!("{:032x}", result));

        // check if new min hash
        if format!("{:032x}", result) < min {
            println!("min x:      {:032x}", previous);
            println!("min md5(x): {:032x}", result);
        }
        min = cmp::min(min,format!("{:032x}", result));

        // check prefix
        let pref_len: u16 = check_prefix(format!("{:032x}", previous),format!("{:032x}", result),0,false);
        if  pref_len >= match_threshold {
            println!("prefix x:      {:032x}", previous);
            println!("prefix md5(x): {:032x}", result);
        }

        // check suffix
        let suff_len: u16 = check_prefix(format!("{:032x}", previous),format!("{:032x}", result),0,true);
        if suff_len >= match_threshold {
            println!("suffix x:      {:032x}", previous);
            println!("suffix md5(x): {:032x}", result);
        }

        match_threshold = cmp::max(cmp::max(pref_len, suff_len), match_threshold);
    }
}

fn check_prefix(mut first: String, mut second: String, mut count: u16,reversed: bool) -> u16 {
    if reversed{
        first = first.chars().rev().collect();
        second = second.chars().rev().collect();
    }
    if first.chars().count() == 0{
        return count;
    }
    else if first.chars().nth(0).unwrap() == second.chars().nth(0).unwrap(){
        count += 1;
        return check_prefix(without_first(first.as_str()).to_string(), without_first(second.as_str()).to_string(),count,false);
    }
    return count;
}

fn without_first(string: &str) -> &str {
    string
        .char_indices()
        .next()
        .and_then(|(i, _)| string.get(i + 1..))
        .unwrap_or("")
}

fn main() {
    let n_workers = num_cpus::get();
    let n_jobs = num_cpus::get();
    let pool = ThreadPool::new(n_workers);

    for _ in 0..n_jobs{
        pool.execute(|| brute());
    }
    pool.join();
}