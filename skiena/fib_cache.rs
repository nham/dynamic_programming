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
        let foo1 = |x: &u64| fib(*x, cache);
        let foo2 = |x: &u64| fib(*x, cache);

        let minus1: u64 = *(cache.find_or_insert_with(n-1, foo1));
        let minus2: u64 = *(cache.find_or_insert_with(n-2, foo2));

        minus1 + minus2
    }

}

