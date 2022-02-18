use std::io;

fn function02() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    match buffer.trim().parse::<usize>() {
        Ok(number) => (0..number).for_each(|_| println!("paiza")),
        Err(e) => println!("{}", e),
    }
    Ok(())
}

fn function03() {
    let mut buffer = String::new();
    if let Ok(_) = io::stdin().read_line(&mut buffer) {
        if let Ok(number) = buffer.trim().parse::<usize>() {
            let vector = (0..number)
                .filter_map(|_| {
                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();
                    buffer.trim().parse::<usize>().ok()
                })
                .collect::<Vec<_>>();
            vector.iter().for_each(|number| println!("{}", number));
        }
    }
}

fn main() -> io::Result<()> {
    // function02().unwrap();
    function03();
    Ok(())
}
