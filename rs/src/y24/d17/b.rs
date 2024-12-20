use super::*;

#[aoc_proc::aoc_run(24-17b)]
pub fn run(_: impl AsRef<str>) -> Result<u64, BoxError> {
    let a = 164516454365000;
    let cor = "2411751503445530";
    let mut best_count = 0;
    let mut best = 0;
    for i in 0..999 {
        let new_a = a + i;
        let ans = exe(new_a);
        let count = shared(&ans, &cor);
        if count > best_count {
            best = new_a;
            best_count = count;
        }
    }
    Ok(best)
}

fn exe(mut a: u64) -> String {
    let mut v = Vec::new();
    while a != 0 {
        v.push(((a % 8) ^ 4 ^ (a / 2u64.pow((a % 8) as u32 ^ 1))) % 8);
        a /= 8;
    }
    v.into_iter().join("")
}

fn shared(s1: &String, s2: &str) -> usize {
    match s1
        .chars()
        .rev()
        .zip(s2.chars().rev())
        .try_fold(String::new(), |mut s, (c1, c2)| {
            if c1 == c2 {
                s.push(c1);
                Ok(s)
            } else {
                Err(s)
            }
        }) {
        Ok(s) => s.len(),
        Err(s) => s.len(),
    }
}
