use md5::{Digest, Md5};
use num_cpus;
use rand::Rng;
use std::cmp;
use std::convert::TryInto;
use std::process;
use threadpool::ThreadPool;

fn brute() {
    let mut hasher = Md5::new();
    let mut rng = rand::thread_rng();

    let seed = rng.gen::<u128>();

    println!("seed: {:032x}", seed);
    hasher.update(format!("{:032x}", seed));

    let mut max: [u8; 16] = [127; 16];
    let mut min: [u8; 16] = [1; 16];
    let mut match_threshold: u8 = 10;

    let mut result = hasher.finalize();

    loop {
        let mut hasher = Md5::new();

        let previous = result;

        hasher.update(format!("{:032x}", result));
        result = hasher.finalize();

        let result_as_array: [u8; 16] = result.as_slice().try_into().expect("incorrect length");
        let previous_as_array: [u8; 16] = previous.as_slice().try_into().expect("incorrect length");

        // check if fixed point
        if result == previous {
            println!("{:032x}", result);
            process::exit(0);
        }
        // check if new max
        if result_as_array > max {
            println!("{}", hex::encode(max));
        }
        max = cmp::max(max, result_as_array);

        // check if new min hash
        if result_as_array < min {
            println!("{}", hex::encode(min));
        }
        min = cmp::min(min, result_as_array);

        // check prefix}
        let pref_len: u8 = check_prefix(result_as_array, previous_as_array, 0);
        if pref_len >= match_threshold {
            println!("prefix x:      {:032x}", previous);
            println!("prefix md5(x): {:032x}", result);
        }

        // check suffix
        let suff_len: u8 = 16 - check_suffix(result_as_array, previous_as_array, 15);
        if suff_len >= match_threshold {
            println!("suffix x:      {:032x}", previous);
            println!("suffix md5(x): {:032x}", result);
        }

        // set new threshold
        match_threshold = cmp::max(cmp::max(pref_len, suff_len), match_threshold);
    }
}

fn check_prefix(first: [u8; 16], second: [u8; 16], mut i: u8) -> u8 {
    if first[i as usize] == second[i as usize] {
        i += 1;
        return check_prefix(first, second, i);
    }
    return i;
}

fn check_suffix(first: [u8; 16], second: [u8; 16], mut i: u8) -> u8 {
    if first[i as usize] == second[i as usize] {
        i -= 1;
        return check_suffix(first, second, i);
    }
    return i;
}

fn main() {
    let n_workers = num_cpus::get();
    let n_jobs = num_cpus::get();
    let pool = ThreadPool::new(n_workers);
    // let n_jobs = 1;

    for _ in 0..n_jobs {
        pool.execute(|| brute());
    }
    pool.join();
}
