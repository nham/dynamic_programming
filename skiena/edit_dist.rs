fn main() {
    let in1 = ~"excellent do";
    let in2 = ~"exceleent eo";
    //let in1 = ~"abog";
    //let in2 = ~"booga";
    let vec1: ~[char] = in1.chars().collect();
    let vec2: ~[char] = in2.chars().collect();
    let vec12: &[char] = vec1.as_slice();
    let vec22: &[char] = vec2.as_slice();
    println!("{}", edit_dist(vec1, vec2, vec1.len(), vec2.len()) );
    println!("DP: {}", edit_dist_DP(vec1, vec2) );

}

fn cost_match(c: char, d: char) -> uint {
    if c == d {
        0u
    } else {
        1u
    }
}

// i, j means s has i chars, t has j chars
fn edit_dist(s: &[char], t: &[char], i: uint, j: uint) -> uint {
    //println!("ed: ({}, {})", i, j);
    if i == 0 {
        return j;
    } else if j == 0 {
        return i;
    }

    let both = edit_dist(s, t, i-1, j-1) + cost_match(s[i-1], t[j-1]);
    let dels = edit_dist(s, t, i-1, j) + 1;
    let delt = edit_dist(s, t, i, j-1) + 1;

    let mut min = both;
    if dels < min { min = dels; }
    if delt < min { min = delt; }
    min

}


fn edit_dist_DP(s: &[char], t: &[char]) -> uint {
    // initialize
    let mut c = ~[];
    c.push(~[]);
    for v in range(0, t.len() + 1) {
        c[0].push(v);
    }

    for u in range(0, s.len() + 1) {
        c.push(~[]);
        c[u].push(u);
        for v in range(1, t.len() + 1) {
            c[u].push(0u);
        }
    }

    for u in range(1, s.len() + 1) {
        for v in range(1, t.len() + 1) {
            //println!("edDP: ({}, {})", u, v);
            let both = c[u-1][v-1] + cost_match(s[u-1], t[v-1]);
            let dels = c[u-1][v] + 1;
            let delt = c[u][v-1] + 1;

            let mut min = both;
            if dels < min { min = dels; }
            if delt < min { min = delt; }
            c[u][v] = min
        }
    }

    c[s.len()][t.len()]

}
