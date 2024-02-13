#[derive(Debug)]
enum TerminalColor {
    Red,
    Green,
    Blue,
}

fn print_color(terminal_color: TerminalColor) {
    match terminal_color {
        TerminalColor::Red => println!("Terminal Color: {:?}", terminal_color),
        TerminalColor::Green => println!("Terminal Color: {:?}", terminal_color),
        TerminalColor::Blue => println!("Terminal Color: {:?}", terminal_color),
    }
}

fn main() {
    print_color(TerminalColor::Red);
}
