use std::{io, cmp};
fn string2nums(s: String) -> Vec<i64> {
   s.trim()
    .split_whitespace()
    .map(|c| c.parse::<i64>().unwrap())
    .collect::<Vec<i64>>()
}

#[derive(Debug)]
struct Pair {
    factor2: i64,
    factor5: i64,
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
        ps.push(Pair{ factor2: f2, factor5: f5})
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![-1; 30*201]; k+1];
    dp[0][0] = 0;
    for i in 0..n {
        let mut tmp = dp.clone();
        for j in 0..k {
            for l in 0..30*200 {
                if dp[j as usize][l as usize] >= 0 {
                   tmp[j+1 as usize][l+ps[i as usize].factor5 as usize]
                       = cmp::max(tmp[j+1 as usize][l+ps[i as usize].factor5 as usize],
                                  dp[j as usize][l as usize] + ps[i as usize].factor2);
                }
            }
        }
        dp = tmp;
    }
    let mut ans = 0;
    for j in 0..k+1 {
        for l in 0..30*200 {
            ans = cmp::max(ans, cmp::min(l, dp[j as usize][l as usize]))
        }
    }
    println!("{}", ans);
}
