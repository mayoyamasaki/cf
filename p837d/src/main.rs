use std::io;
fn string2nums(s: String) -> Vec<u64> {
   s.trim()
    .split_whitespace()
    .map(|c| c.parse::<u64>().unwrap())
    .collect::<Vec<u64>>()
}

fn roundness(n: u64) -> usize {
    n.to_string().chars().rev().take_while(|&c| c == '0').count()
}

fn besti(ref nums: &Vec<u64>, p: u64) -> usize {
    let mut max_i = 0;
    let mut max_r = 0;
    for (i, n) in nums.iter().enumerate() {
        let r = roundness(n * p);
        if r > max_r {
            max_r = r;
            max_i = i;
        }
    }
    max_i
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nums = string2nums(buf);
    let k = nums.pop().unwrap() as usize;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nums = string2nums(buf);

    if k < 2 {
        let i = besti(&nums, 1);
        println!("{}", roundness(nums[i]));
        return;
    }

    let mut max_r = 0;
    let mut max_p = 0;
    let mut max_i = 0;
    let mut max_j = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j { continue; }
            let i = i as usize;
            let j = j as usize;
            let p = nums[i] * nums[j];
            let r = roundness(p);
            if r >= max_r {
                max_r = r;
                max_p = p;
                max_i = i;
                max_j = j;
            }
        }
    }

    if max_i > max_j {
        nums.remove(max_i); nums.remove(max_j);
    } else {
        nums.remove(max_j); nums.remove(max_i);
    }

    for _ in 0..k-2 {
        let i = besti(&nums, max_p);
        max_p *= nums[i];
        max_r = roundness(max_p);
        nums.remove(i);
    }
    println!("{}", max_r);
}
