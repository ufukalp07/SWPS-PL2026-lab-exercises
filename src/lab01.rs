//! Lab 1 — First Steps in Rust — exercises.

#![allow(dead_code, unused_variables, unused_mut)]

fn task1_formatting() {
    let name = "Ada";
    let age = 36;
    println!("Name: {name}, age: {age}");
    println!("Pi to 3 places: {:.3}", std::f64::consts::PI);
    println!("|{:>8}|{:<8}|{:^8}|", "right", "left", "center");
    println!("{:?}", (1, "two", 3.0));
    println!("0x {:x}, 0b {:b}", 255, 10);
}

fn task2_variables() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner: {x}");
    }
    println!("outer: {x}");

    let mut count = 0;
    count += 1;
    println!("{count}");
}

fn factorial(n: i32) -> Option<u64> {
    if n < 0 {
        return None;
    }
    let mut acc: u64 = 1;
    for i in 2..=n as u64 {
        acc = match acc.checked_mul(i) {
            Some(val) => val,
            None => return None,
        };
    }
    Some(acc)
}

fn task3_scalars() {
    println!("{}", i8::MIN);
    println!("{}", i8::MAX);
    println!("{}", u8::MAX);
    println!("{}", i32::MAX);
    println!("{}", u64::MAX);
    println!("{}", f64::EPSILON);

    let val: u8 = 250;
    println!("{:?}", val.checked_add(10));
    println!("{}", val.wrapping_add(10));
    println!("{}", val.saturating_add(10));
    println!("{:?}", val.overflowing_add(10));

    println!("{:?}", factorial(20));
    println!("{:?}", factorial(21));

    println!("{}", 300_i32 as u8);
    println!("{}", -1_i32 as u32);
    println!("{}", 3.99_f64 as i32);
    println!("{}", -3.99_f64 as u32);
    println!("{:?}", u8::try_from(300_i32));
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn sign(x: i32) -> String {
    if x > 0 {
        "positive".to_string()
    } else if x == 0 {
        "zero".to_string()
    } else {
        "negative".to_string()
    }
}

fn square(x: i32) -> i32 {
    x * x
}

fn task4_expressions() {
    println!("{}", celsius_to_fahrenheit(100.0));
    println!("{}", fahrenheit_to_celsius(212.0));
    println!("{}", sign(-3));
    println!("{}", square(7));

    let y = {
        let a = 3;
        a * a + 1
    };
    println!("{y}");
}

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

fn collatz_steps(mut n: u64) -> u32 {
    let mut steps = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    steps
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn grade(points: u32) -> char {
    match points {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

fn task5_control_flow() {
    for i in 1..=20 {
        println!("{}", fizzbuzz(i));
    }

    println!("{}", collatz_steps(27));
    println!("{}", gcd(48, 18));
    println!("{}", is_prime(97));
    println!("{}", fib(50));
    println!("{}", grade(95));
}

fn first_power_of_two_above(n: u64) -> u64 {
    let mut pow = 1;
    loop {
        if pow > n {
            break pow;
        }
        pow *= 2;
    }
}

fn task6_labelled(limit: u32) {
    'outer: for i in 1..=9 {
        for j in 1..=9 {
            if i * j > limit {
                println!();
                continue 'outer;
            }
            print!("{:2} ", i * j);
        }
        println!();
    }
}

fn min_max_avg(values: Vec<i32>) -> (i32, i32, f64) {
    let mut min = values[0];
    let mut max = values[0];
    let mut sum = 0;

    for &v in &values {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
        sum += v;
    }

    let avg = sum as f64 / values.len() as f64;
    (min, max, avg)
}

fn swap_pair(p: (i32, String)) -> (String, i32) {
    (p.1, p.0)
}

fn task7_compound() {
    let (lo, hi, avg) = min_max_avg(vec![3, -1, 7, 2]);
    println!("{lo}, {hi}, {avg}");

    let mut grid = [[0u8; 3]; 3];
    for i in 0..3 {
        grid[i][i] = 1;
    }
    println!("{:?}", grid);

    let swapped = swap_pair((42, String::from("hello")));
    println!("{:?}", swapped);
}

pub fn run() {
    task1_formatting();
    task2_variables();
    task3_scalars();
    task4_expressions();
    task5_control_flow();
    println!(
        "first power of two above 5: {}",
        first_power_of_two_above(5)
    );
    task6_labelled(12);
    task7_compound();
}