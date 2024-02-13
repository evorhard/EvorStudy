fn main() {
    let mut i: i8 = 1;
    loop {
        println!("{:?}", i);
        i = i + 1;
        if i > 4{
            break;
        }
    }
}
