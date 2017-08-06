use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nums: Vec<u32> = buf.split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    let width = nums.pop().unwrap();
    let height = nums.pop().unwrap();

    let mut buf = String::new();
    let mut flag: Vec<Vec<char>> = Vec::new();
    for _ in 0..height {
        io::stdin().read_line(&mut buf).unwrap();
        flag.push(buf.trim().chars().collect::<Vec<char>>());
        buf = String::new();
    }

    let mut colors: HashSet<char> = ['R', 'G', 'B'].iter().cloned().collect();
    let mut sizes: HashMap<char, u32> = [('R', 0), ('G', 0), ('B', 0)].iter().cloned().collect();;
    let mut line_is_same = true;
    let mut color_is_onece = true;
    let mut curr_color = '_';
    for i in 0..height {
        let i = i as usize;
        let color = flag[i][0];
        if let Some(c) = sizes.get_mut(&color) {
            *c += 1;
        };

        // check line has same colors
        for j in 1..width {
            let j = j as usize;
            if color != flag[i][j] {
                line_is_same = false;
                break;
            }
        }

        // check each color be used in only onece
        if curr_color != color {
            if !colors.remove(&color) {
                color_is_onece = false;
                break;
            }
            curr_color = color;
        }

        if line_is_same == false || color_is_onece == false { break; }
    }

    let sizes: HashSet<u32> = sizes.values().cloned().collect();

    if line_is_same && color_is_onece && sizes.len() == 1 {
        println!("YES");
        return;
    }

    let mut colors: HashSet<char> = ['R', 'G', 'B'].iter().cloned().collect();
    let mut sizes: HashMap<char, u32> = [('R', 0), ('G', 0), ('B', 0)].iter().cloned().collect();;
    let mut line_is_same = true;
    let mut color_is_onece = true;
    let mut curr_color = '_';
    for i in 0..width {
        let i = i as usize;
        let color = flag[0][i];
        if let Some(c) = sizes.get_mut(&color) {
            *c += 1;
        };

        // check line has same colors
        for j in 1..height {
            let j = j as usize;
            if color != flag[j][i] {
                line_is_same = false;
                break;
            }
        }

        // check each color be used in only onece
        if curr_color != color {
            if !colors.remove(&color) {
                color_is_onece = false;
                break;
            }
            curr_color = color;
        }

        if line_is_same == false || color_is_onece == false { break; }
    }

    let sizes: HashSet<u32> = sizes.values().cloned().collect();

    if line_is_same && color_is_onece && sizes.len() == 1 {
        println!("YES");
        return;
    } else {
        println!("NO");
    }

}
