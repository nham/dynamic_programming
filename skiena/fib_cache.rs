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
    //println!("{}", non_cache_fib(n.unwrap()));
    println!("{}", dpfib(n.unwrap()));
    println!("the ultimate: {}", dpfib_ultra(n.unwrap()));

}

fn fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    println!("we're manually computing {}", n); 
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


fn non_cache_fib(n: u64) -> u64 {
    println!("we're manually computing {}", n); 
    if n == 0 || n == 1 {
        n
    } else {
        non_cache_fib(n-1) + non_cache_fib(n-2)
    }

}


fn dpfib(n: u64) -> u64 {
    let mut cache: ~[u64] = ~[0u64, 1u64];

    for i in range(2, n+1) {
        cache.push(cache[i-1] + cache[i-2]);
    }

    cache[n]
}

fn dpfib_ultra(n: u64) -> u64 {
    let mut prev = 0u64;
    let mut this = 1u64;

    let mut tmp;
    for i in range(2, n+1) {
        tmp = this;
        this = this + prev;
        prev = tmp;
    }

    this
}
