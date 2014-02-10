use std::hashmap::HashMap;
use std::os;

fn main() {
    let args = os::args();
    let n: Option<u64> = from_str(args[1]);
    if n.is_none() {
        println!("that ain't a number");
        return;
    }


    let mut fibcache = HashMap::<u64, u64>::new();

    println!("{}", fib(n.unwrap(), &mut fibcache));

}

fn fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 || n == 1 {
        n
    } else {
        let minus1: u64;
        let minus2: u64;

        if cache.contains_key(&(n-1)) {
            minus1 = *(cache.get(&(n-1)))
        } else {
            minus1 = fib(n-1, cache);
            cache.insert(n-1, minus1);
        }

        if cache.contains_key(&(n-2)) {
            minus2 = *(cache.get(&(n-2)))
        } else {
            minus2 = fib(n-2, cache);
            cache.insert(n-2, minus2);
        }

        minus1 + minus2
    }

}

