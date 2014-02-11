fn main() {
    println!("{}", dp(7, 3));

}


fn naive_rec(n: u64, k: u64) -> u64 {
    if k == 0 || n == k {
        1
    } else {
        naive_rec(n-1, k-1) + naive_rec(n-1, k)
    }
}

fn dp(n: u64, k: u64) -> u64 {
    // initialize an n+1 by k+1 block of u64s
    let mut c: ~[~[u64]] = ~[];
    for i in range(0, n+1) {
        c.push(~[]);
        for j in range(0, k+1) {
            if j == 0 || i == j {
                c[i].push(1);
            } else {
                c[i].push(0);
            }
        }
    }


    for i in range(1, n+1) {
        let mut j = 1;
        while j < i && j <= k {
            c[i][j] = c[i-1][j] + c[i-1][j-1];
            j += 1;
        }

    }

    println!("{:?}", c);

    c[n][k]

}
