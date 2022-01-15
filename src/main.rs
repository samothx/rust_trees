use colored::Colorize;

fn main() {
    let red = "Red".red().to_string();
    let blue = "Blue".blue().to_string();
    let green = "Green".green().to_string();
    let mut buffer = red;
    buffer.push_str(",");
    buffer.push_str(&blue);
    buffer.push_str(",");
    buffer.push_str(&green);
    println!("Hello World! \n{}", buffer);
}
