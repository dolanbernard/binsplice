use clap::Parser;

mod args;

fn main() {
    let config = args::Args::parse();
    let data = std::fs::read(&config.input_filename).unwrap();
    let line_len = config.columns * config.group_len;
    /*data.into_iter().enumerate().for_each(|b| {
        if b.0 % line_len == 0 {
            println!();
        } else {
            if b.0 % config.group_len == 0 {
                print!("    ");
            } else {
                print!(" ");
            }
        }
        print!("{}", to_str_radix(b.1 as usize, 16, 3));
    });
    println!();*/
    data.chunks(line_len).enumerate().for_each(|chunk| {
        if !config.hide_ranges {
            let line_num = chunk.0;
            let line_num_len = (data.len().checked_ilog(16).unwrap_or(0) + 1) as usize;
            let line_start = line_num * line_len;
            let line_end = std::cmp::min(line_start + line_len, data.len());
            print!("<{}-{}>  ", to_str_radix(line_start, 16, line_num_len), to_str_radix(line_end, 16, line_num_len));
        }
        let line = chunk.1;
        line.chunks(config.group_len).for_each(|column| {
            column.iter().for_each(|b| {
                print!("{} ", to_str_radix(*b as usize, 16, 3));
            });
            print!("  ");
        });
        if !config.no_decode {
            line.iter().for_each(|b| {
                let c = *b as char;
                if c.is_alphanumeric() {
                    print!("{c}");
                } else {
                    print!(".");
                }
            })
        }
        println!();
    });
}

fn to_str_radix(b: usize, radix: usize, padding: usize) -> String {
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
    if padding > result.len() {
        let padding_len = padding - result.len();
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
