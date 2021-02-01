use md5::{Md5, Digest};
use rand::Rng;
use std::cmp;
use threadpool::ThreadPool;
use std::process;

fn brute() {
    let mut hasher = Md5::new();
    let mut rng = rand::thread_rng();

    let seed = rng.gen::<u128>();

    println!("seed: {:032x}",seed);
    hasher.update(format!("{:032x}", seed));

    let mut max: String = "fffff800000000000000000000000000".to_string();
    let mut min: String = "000008ffffffffffffffffffffffffff".to_string();

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
        if format!("{:032x}", result) > max{
            println!("max x:      {:032x}", previous);
            println!("max md5(x): {:032x}", result)
        }
        max = cmp::max(max,format!("{:032x}", result));

        // check if new min hash
        if format!("{:032x}", result) < min{
            println!("min x:      {:032x}", previous);
            println!("min md5(x): {:032x}", result)
        }
        min = cmp::min(min,format!("{:032x}", result));

        // check Preffix
        checkPreffix(format!("{:032x}", previous),format!("{:032x}", result),0,false);

        // check suffix
        checkPreffix(format!("{:032x}", previous),format!("{:032x}", result),0,true);
    }
}

fn checkPreffix(mut first: String, mut second: String, mut count: u16,reversed: bool) -> u16 {
    if reversed{
        first = first.chars().rev().collect();
        second = second.chars().rev().collect();
    }
    if first.chars().count() == 0{
        return 0;
    }
    else if first.chars().nth(0).unwrap() == second.chars().nth(0).unwrap(){
        checkPreffix(without_first(first.as_str()).to_string(), without_first(second.as_str()).to_string(),count,reversed);
        count += 1;
    }
    if count > 1{
        if reversed{
            first = first.chars().rev().collect();
            second = second.chars().rev().collect();
        }
        println!("*FIX x:      {}", first);
        println!("*FIX md5(x): {}", second)
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

    let n_workers = 8;
    let n_jobs = 8;
    let pool = ThreadPool::new(n_workers);

    for _ in 0..n_jobs{
        pool.execute(|| brute());
    }
    pool.join();
}