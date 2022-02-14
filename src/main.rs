use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    match buffer.trim().parse::<usize>() {
        Ok(number) => (0..number).for_each(|_| println!("paiza")),
        Err(e) => println!("{}", e),
    }
    Ok(())
}
