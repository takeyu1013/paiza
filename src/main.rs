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

fn function03() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut vector = Vec::new();
    match buffer.trim().parse::<usize>() {
        Ok(number) => (0..number).for_each(|_| {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            vector.push(buffer.trim().parse::<usize>().unwrap());
        }),
        Err(e) => println!("{}", e),
    }
    vector.iter().for_each(|number| println!("{}", number));
    Ok(())
}

fn main() -> io::Result<()> {
    // function02().unwrap();
    function03().unwrap();
    Ok(())
}
