fn main() {
    let in1 = ~"excellent do";
    let in2 = ~"exceleent eo";
    let vec1: ~[char] = in1.chars().collect();
    let vec2: ~[char] = in2.chars().collect();
    let vec12: &[char] = vec1.as_slice();
    let vec22: &[char] = vec2.as_slice();
    println!("{}", edit_dist(vec1, vec2, vec1.len() - 1, vec2.len() - 1) );

}

// T = "I'm agog"
// P = "iago"
fn edit_dist(s: &[char], t: &[char], i: uint, j: uint) -> uint {
    if i == 0 {
        return j;
    } else if j == 0 {
        return i;
    }

    fn cost_match(c: char, d: char) -> uint {
        if c == d {
            0u
        } else {
            1u
        }
    }

    let both = edit_dist(s, t, i-1, j-1) + cost_match(s[i], t[j]);
    let dels = edit_dist(s, t, i-1, j) + 1;
    let delt = edit_dist(s, t, i, j-1) + 1;

    let mut min = both;
    if dels < min { min = dels; }
    if delt < min { min = delt; }
    min

}
