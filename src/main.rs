use std::io::BufRead;

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
    if let Some(next) = std::io::stdin().lock().lines().next() {
        if let Ok(line) = next {
            let count = line.split_whitespace().count();
            println!("{}", count);
        }
    }
}

pub fn function06() {
    if let Some(next) = std::io::stdin().lock().lines().next() {
        if let Ok(line) = next {
            if let Ok(number) = line.trim().parse::<usize>() {
                println!("{}, {}", number, line);
            }
        }
    }
}

fn main() {
    function06()
}
