use std::io;
use std::cmp;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x: u32,
    y: u32
}

impl Rectangle {
    fn max(&self) -> u32 {
        cmp::max(self.x, self.y)
    }

    fn min(&self) -> u32 {
        cmp::min(self.x, self.y)
    }

    fn area(&self) -> u32 {
        self.x * self.y
    }
}

fn string2nums(s: String) -> Vec<u32> {
   s.trim()
    .split_whitespace()
    .map(|c| c.parse::<u32>().unwrap())
    .collect::<Vec<u32>>()
}

fn is_valid(ref a: &Rectangle, ref b: &Rectangle, ref p: &Rectangle) -> bool {
    if (a.max() <= p.max() && b.max() <= p.max() && a.min()+b.min() <= p.min()) ||
       (a.max() <= p.max() && b.min() <= p.max() && a.min()+b.max() <= p.min()) ||
       (a.min() <= p.max() && b.max() <= p.max() && a.max()+b.min() <= p.min()) ||
       (a.min() <= p.max() && b.min() <= p.max() && a.max()+b.max() <= p.min()) ||
       (a.max() <= p.min() && b.max() <= p.min() && a.min()+b.min() <= p.max()) ||
       (a.max() <= p.min() && b.min() <= p.min() && a.min()+b.max() <= p.max()) ||
       (a.min() <= p.min() && b.max() <= p.min() && a.max()+b.min() <= p.max()) ||
       (a.min() <= p.min() && b.min() <= p.min() && a.max()+b.max() <= p.max()) {
        return true;
    } else {
        return false;
    }
}

fn combinations(v: Vec<Rectangle>) -> Vec<(Rectangle, Rectangle)> {
    let mut c: Vec<(Rectangle, Rectangle)> = Vec::new();
    for (i, v1) in v.iter().cloned().enumerate() {
        for (j, v2) in v.iter().cloned().enumerate() {
            if i != j {
                c.push((v1, v2));
            }
        }
    }
    return c
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut nums = string2nums(buf);
    let paper = Rectangle {
        y: nums.pop().unwrap(),
        x: nums.pop().unwrap(),
    };
    let n = nums.pop().unwrap();

    let mut buf = String::new();
    let mut rectangles: Vec<Rectangle> = Vec::new();
    for _ in 0..n {
        io::stdin().read_line(&mut buf).unwrap();
        let mut nums = string2nums(buf);
        rectangles.push(Rectangle {
            y: nums.pop().unwrap(),
            x: nums.pop().unwrap(),
        });
        buf = String::new();
    }

    let mut best = 0;
    for (a, b) in combinations(rectangles){
        if is_valid(&a, &b, &paper) {
            let sum = a.area() + b.area();
            if sum > best {
                best = sum;
            }
        }
    }
    println!("{}", best);
}
