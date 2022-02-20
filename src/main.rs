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

fn main() {
    function04()
}
