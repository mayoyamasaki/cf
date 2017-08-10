use std::{io, cmp};
fn string2nums(s: String) -> Vec<i64> {
   s.trim()
    .split_whitespace()
    .map(|c| c.parse::<i64>().unwrap())
    .collect::<Vec<i64>>()
}

struct Pair {
    factor2: usize,
    factor5: usize,
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nums = string2nums(buf);
    let k = nums.pop().unwrap() as usize;
    let n = nums.pop().unwrap() as usize;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let nums = string2nums(buf);

    let mut ps: Vec<Pair> = Vec::new();
    for n in nums {
        let mut n = n;
        let mut f2 = 0;
        let mut f5 = 0;
        while n%2 == 0 {
            f2 += 1;
            n /= 2;
        }
        while n%5 == 0 {
            f5 += 1;
            n /= 5;
        }
        ps.push(Pair{ factor2: f2 as usize, factor5: f5 as usize})
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![-1; 30*201]; k+1];
    dp[0][0] = 0;
    for i in 0..n as usize{
        let mut tmp = dp.clone();
        for j in 0..k as usize {
            for l in 0..30*200 as usize {
                if dp[j][l] >= 0 {
                   tmp[j+1][l+ps[i].factor5] = cmp::max(tmp[j+1][l+ps[i].factor5],
                                                        dp[j][l] + ps[i].factor2 as i64);
                }
            }
        }
        dp = tmp;
    }
    let mut ans = 0;
    for j in 0..k+1 as usize {
        for l in 0..30*200 {
            ans = cmp::max(ans, cmp::min(l, dp[j][l as usize]))
        }
    }
    println!("{}", ans);
}
