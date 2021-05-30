/*
    ランレングス圧縮/復元 O(N)
    文字列の可逆圧縮の一種。
*/

pub fn run_length_encode(s: Vec<char>) -> Vec<(char, usize)> {
    let n = s.len();
    let mut ret = Vec::new();
    let mut l = 0;
    while l < n {
        let mut r = l + 1;
        while r < n && s[l] == s[r] {
            r += 1;
        }
        ret.push((s[l], r-l));
        l = r;
    }
    ret
}

fn run_length_decode(code: Vec<(char, usize)>) -> String {
    let mut ret = String::new();
    for (c, c_size) in code {
        for _ in 0..c_size {
            ret.push(c);
        }
    }
    ret
}

#[test] 
fn run_length_test() {
    let s = "aabbbaad".chars().collect::<Vec<char>>();
    let encode = run_length_encode(s);
    let mut res = String::new();
    for (c, c_size) in encode.iter() {
        res.push(*c);
        res.push(std::char::from_digit(*c_size as u32, 10).unwrap());
    }
    assert_eq!("a2b3a2d1", res);
    let res = run_length_decode(encode);
    assert_eq!("aabbbaad", res);
}