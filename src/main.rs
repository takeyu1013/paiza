pub fn function01() {
    let mut buffer = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        print!("{}", buffer);
    }
}

pub fn function02() {
    let mut buffer = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if let Ok(number) = buffer.trim().parse::<usize>() {
            (0..number).for_each(|_| println!("paiza"));
        }
    }
}

pub fn function03() {
    let mut buffer = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if let Ok(number) = buffer.trim().parse::<usize>() {
            let vector = (0..number)
                .filter_map(|_| {
                    let mut buffer = String::new();
                    std::io::stdin()
                        .read_line(&mut buffer)
                        .ok()
                        .and_then(|_| buffer.trim().parse::<usize>().ok())
                })
                .collect::<Vec<_>>();
            vector.iter().for_each(|number| println!("{}", number));
        }
    }
}

pub fn function04() {
    let mut buffer = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if let Ok(number) = buffer.trim().parse::<usize>() {
            let vector = (0..number)
                .filter_map(|_| {
                    let mut buffer = String::new();
                    std::io::stdin()
                        .read_line(&mut buffer)
                        .ok()
                        .and_then(|_| buffer.trim().parse::<usize>().ok())
                })
                .collect::<Vec<_>>();
            if let Some(max) = vector.iter().max() {
                println!("{}", max);
            }
        }
    }
}

pub fn function05() {
    use std::io::{stdin, BufRead};
    if let Some(next) = stdin().lock().lines().next() {
        if let Ok(line) = next {
            let count = line.split_whitespace().count();
            println!("{}", count);
        }
    }
}

pub fn function06() {
    let mut buffer = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if let Ok(number) = buffer.trim().parse::<usize>() {
            let mut buffer = String::new();
            if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
                buffer
                    .split_whitespace()
                    .take(number)
                    .for_each(|word| println!("{}", word))
            }
        }
    }
}

pub fn c_rank_std_in_out_boss() {
    let mut buffer = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if let Ok(number) = buffer.trim().parse::<usize>() {
            let lines = (0..number)
                .filter_map(|_| {
                    let mut buffer = String::new();
                    std::io::stdin()
                        .read_line(&mut buffer)
                        .ok()
                        .and_then(|_| Some(String::from(buffer.trim())))
                })
                .collect::<Vec<_>>();
            let tuples = lines
                .iter()
                .filter_map(|line| {
                    Some((
                        line.split_whitespace().next().unwrap(),
                        line.split_whitespace().nth(1).unwrap(),
                    ))
                })
                .collect::<Vec<_>>();
            tuples
                .iter()
                .for_each(|tuple| println!("{}, {}", tuple.0, tuple.1));
        }
    }
}

pub fn c_rank_string_step1() {
    let mut buffer = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if let Ok(number) = buffer.trim().parse::<usize>() {
            let lines = (0..number)
                .filter_map(|_| {
                    let mut buffer = String::new();
                    std::io::stdin()
                        .read_line(&mut buffer)
                        .ok()
                        .and_then(|_| Some(String::from(buffer.trim())))
                })
                .collect::<Vec<_>>();
            lines.iter().for_each(|line| {
                println!("{}", line);
            });
        }
    }
}

fn main() {
    c_rank_string_step1();
}
