use std::env::var_os;

fn main() {
    let mut numero: u8 = 0;
    loop {
        print!("{}", numero);
        numero += 1;
        if numero == 10 {
            break;
        }
    }
}
