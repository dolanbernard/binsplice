use clap::Parser;

mod args;

fn main() {
    let config = args::Args::parse();
    let data = std::fs::read(&config.input_filename).unwrap();
    let line_len = config.columns * config.group_len;
    data.into_iter().enumerate().for_each(|b| {
        if b.0 % line_len == 0 {
            println!();
        } else {
            if b.0 % config.group_len == 0 {
                print!("    ");
            } else {
                print!(" ");
            }
        }
        print!("{}", to_str_radix(b.1, 16, 3));
    });
    println!();
}

fn to_str_radix(b: u8, radix: u8, padding: u8) -> String {
    let mut value = b;
    let mut result = String::new();
    if value == 0 {
        result.push('0');
    }
    while value > 0 {
        let r = value % radix;
        let c = char::from_digit(r as u32, radix as u32).unwrap();
        result.push(c.to_ascii_uppercase());
        value /= radix;
    }
    if padding > result.len() as u8 {
        let padding_len = padding - result.len() as u8;
        for _ in 0..padding_len {
            result.insert(0, '0');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_str_radix() {
        assert_eq!("0", to_str_radix(0, 10, 0));
        assert_eq!("0", to_str_radix(0, 10, 1));
        assert_eq!("00", to_str_radix(0, 10, 2));
        assert_eq!("FF", to_str_radix(255, 16, 0));
        assert_eq!("FF", to_str_radix(255, 16, 1));
        assert_eq!("FF", to_str_radix(255, 16, 2));
        assert_eq!("0", to_str_radix(0, 3, 0));
        assert_eq!("011", to_str_radix(4, 3, 3));
    }

}
