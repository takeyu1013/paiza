pub fn c_rank_std_in_out_step1() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    print!("{}", buffer);
}

const PAIZA: &str = "paiza";

pub fn c_rank_std_in_out_step2() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    let Ok(number) = buffer.trim().parse::<usize>() else {
        return
    };
    (0..number - 1).for_each(|_| print!("{} ", PAIZA));
    println!("{}", PAIZA);
}

pub fn c_rank_std_in_out_step3() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    let Ok(number) = buffer.trim().parse::<usize>() else {
        return
    };
    (0..number)
        .filter_map(|_| {
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .ok()
                .and_then(|_| buffer.trim().parse::<usize>().ok())
        })
        .collect::<Vec<_>>()
        .iter()
        .for_each(|number| println!("{}", number));
}

pub fn c_rank_std_in_out_step4() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    let Ok(number) = buffer.trim().parse::<usize>() else {
        return
    };
    let vector = (0..number)
        .filter_map(|_| {
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .ok()
                .and_then(|_| buffer.trim().parse::<usize>().ok())
        })
        .collect::<Vec<_>>();
    let Some(max) = vector.iter().max() else {
        return
    };
    println!("{}", max);
}

pub fn c_rank_std_in_out_step5() {
    use std::io::{stdin, BufRead};
    let Some(next) = stdin().lock().lines().next() else {
        return
    };
    let Ok(line) = next else {
        return
    };
    println!("{}", line.split_whitespace().count());
}

pub fn c_rank_std_in_out_step6() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    let Ok(number) = buffer.trim().parse::<usize>() else {
        return
    };
    buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    buffer
        .split_whitespace()
        .take(number)
        .for_each(|word| println!("{}", word));
}

pub fn c_rank_std_in_out_boss() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    let Ok(number) = buffer.trim().parse::<usize>() else {
        return
    };
    (0..number)
        .map(|_| {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).ok();
            buffer
        })
        .collect::<Vec<_>>()
        .iter()
        .for_each(|line| {
            let vector = line.split_whitespace().take(2).collect::<Vec<_>>();
            vector.get(0).map(|name| print!("{}", name));
            vector.get(1).map(|string| {
                string
                    .trim()
                    .parse::<usize>()
                    .ok()
                    .map(|number| print!(" {}", number + 1));
            });
            println!();
        })
}

pub fn c_rank_string_step1() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    buffer.trim().parse::<usize>().ok().map(|number| {
        (0..number)
            .map(|_| {
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).ok();
                buffer
            })
            .collect::<Vec<_>>()
            .iter()
            .for_each(|line| {
                println!("{}", line.trim().chars().count());
            });
    });
}

pub fn c_rank_string_step2() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    buffer.chars().next().map(|character| {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        println!(
            "{}",
            if buffer.contains(character) {
                "YES"
            } else {
                "NO"
            }
        );
    });
}

pub fn c_rank_string_step3() {
    use std::io::BufRead;
    std::io::stdin().lock().lines().next().map(|next| {
        next.ok().map(|line| {
            let numbers = line
                .chars()
                .filter_map(|character| character.to_digit(10))
                .collect::<Vec<_>>();
            numbers.get(0).map(|first| {
                numbers.get(3).map(|second| print!("{}", first + second));
            });
            numbers.get(1).map(|first| {
                numbers.get(2).map(|second| print!("{}", first + second));
            });
            println!();
        });
    });
}

pub fn c_rank_string_step4() {
    use std::io::BufRead;
    std::io::stdin().lock().lines().next().map(|next| {
        next.ok().map(|line| {
            line.trim().parse::<usize>().ok().map(|number| {
                println!("{:03}", number);
            });
        });
    });
}

pub fn c_rank_string_step5() {
    use std::io::BufRead;
    std::io::stdin().lock().lines().next().map(|next| {
        next.ok().map(|line| {
            line.split(":").take(2).for_each(|string| {
                string
                    .parse::<usize>()
                    .ok()
                    .map(|number| println!("{}", number));
            });
        });
    });
}

pub fn c_rank_string_step6() {
    use std::io::BufRead;
    std::io::stdin().lock().lines().next().map(|next| {
        next.ok().map(|line| {
            let numbers = line
                .split(":")
                .take(2)
                .flat_map(|string| string.parse::<usize>().ok())
                .collect::<Vec<_>>();
            numbers.get(1).map(|minutes| {
                let new_minutes = minutes + 30;
                if &new_minutes > &59 {
                    numbers
                        .get(0)
                        .map(|hour| println!("{:02}:{:02}", hour + 1, new_minutes - 60));
                } else {
                    numbers
                        .get(0)
                        .map(|hour| println!("{:02}:{:02}", hour, new_minutes));
                }
            });
        });
    });
}

pub fn c_rank_string_step6_chrono() {
    use chrono::prelude::Local;
    use chrono::Duration;
    use chrono::Timelike;
    use std::io::BufRead;
    std::io::stdin().lock().lines().next().map(|next| {
        next.ok().map(|line| {
            let numbers = line
                .split(":")
                .take(2)
                .flat_map(|string| string.parse::<u32>().ok())
                .collect::<Vec<_>>();
            numbers.get(0).map(|hour| {
                numbers.get(1).map(|minutes| {
                    let time = Local::today().and_hms(hour.clone(), minutes.clone(), 0)
                        + Duration::minutes(30);
                    println!("{:02}:{:02}", time.hour(), time.minute());
                });
            });
        });
    });
}

pub fn c_rank_string_boss() {
    use chrono::prelude::Local;
    use chrono::Duration;
    use chrono::Timelike;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    buffer.trim().parse::<usize>().ok().map(|number| {
        let lines = (0..number)
            .map(|_| {
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).ok();
                buffer
            })
            .collect::<Vec<_>>();
        lines.iter().for_each(|line| {
            let strings = line.split_whitespace().take(3).collect::<Vec<_>>();
            strings.get(0).map(|string| {
                let numbers = string
                    .split(":")
                    .take(2)
                    .flat_map(|string| string.parse::<u32>().ok())
                    .collect::<Vec<_>>();
                numbers.get(0).map(|hour| {
                    numbers.get(1).map(|minutes| {
                        strings.get(1).map(|string| {
                            string.parse::<i64>().ok().map(|hour_delta| {
                                strings.get(2).map(|string| {
                                    string.parse::<i64>().ok().map(|minutes_delta| {
                                        let time = Local::today().and_hms(
                                            hour.clone(),
                                            minutes.clone(),
                                            0,
                                        ) + Duration::hours(hour_delta)
                                            + Duration::minutes(minutes_delta);
                                        println!("{:02}:{:02}", time.hour(), time.minute());
                                    });
                                });
                            });
                        });
                    });
                });
            });
        });
    });
}

pub fn c_rank_for_step1() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    buffer.trim().parse::<usize>().ok().map(|number| {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        println!(
            "{}",
            buffer
                .split_whitespace()
                .take(number)
                .filter_map(|string| string.parse::<usize>().ok())
                .filter(|number| number % 3 == 0)
                .count()
        );
    });
}

pub fn c_rank_for_step2() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    buffer.trim().parse::<usize>().ok().map(|number| {
        println!(
            "{}",
            if (0..number)
                .filter_map(|_| {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).ok();
                    buffer.trim().parse::<usize>().ok()
                })
                .collect::<Vec<_>>()
                .iter()
                .any(|&number| number == 7)
            {
                "YES"
            } else {
                "NO"
            }
        );
    });
}

pub fn c_rank_for_step3() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    buffer.trim().parse::<usize>().ok().map(|number| {
        let numbers = (0..number)
            .filter_map(|_| {
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).ok();
                buffer.trim().parse::<usize>().ok()
            })
            .collect::<Vec<_>>();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        buffer.trim().parse::<usize>().ok().map(|price| {
            numbers
                .iter()
                .position(|&number| number == price)
                .map(|position| println!("{}", position + 1))
        });
    });
}

pub fn c_rank_for_step4() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).ok();
    buffer.trim().parse::<usize>().ok().map(|number| {
        let characters = (0..number)
            .filter_map(|_| {
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).ok();
                buffer.chars().next()
            })
            .collect::<Vec<_>>();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        buffer.trim().parse::<usize>().ok().map(|number| {
            (0..number)
                .map(|_| {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).ok();
                    buffer
                })
                .collect::<Vec<_>>()
                .iter()
                .for_each(|string| {
                    characters.iter().for_each(|&character| {
                        println!(
                            "{}",
                            if string.contains(character) {
                                "YES"
                            } else {
                                "NO"
                            }
                        )
                    })
                })
        });
    });
}

pub fn d002() {
    use std::io::BufRead;
    std::io::stdin().lock().lines().next().map(|next| {
        next.ok().map(|line| {
            let numbers = line
                .split_whitespace()
                .take(2)
                .filter_map(|string| string.trim().parse::<usize>().ok())
                .collect::<Vec<_>>();
            if let (Some(a), Some(b)) = (numbers.get(0), numbers.get(1)) {
                println!(
                    "{}",
                    if a > b {
                        a.to_string()
                    } else if a < b {
                        b.to_string()
                    } else {
                        String::from("eq")
                    }
                );
            }
        });
    });
}

fn main() {
    c_rank_std_in_out_boss()
}
