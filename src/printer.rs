


pub fn print_data(data: &Vec<u8>,
    columns: usize,
    column_len: usize,
    start_index: Option<usize>,
    end_index: Option<usize>,
    no_decode: bool,
    hide_ranges: bool
) -> Vec<String> {
    let line_len = columns * column_len;
    let start_index = start_index.unwrap_or_default();
    let end_index = usize::min(end_index.unwrap_or(usize::MAX), data.len());
    data[start_index..end_index]
        .chunks(line_len)
        .enumerate()
        .map(|chunk| print_line(chunk.1, chunk.0, (start_index..end_index).len(), columns, column_len, no_decode, hide_ranges))
        .collect()
}

fn print_line(line: &[u8], line_num: usize, block_len: usize, columns: usize, column_len: usize, no_decode: bool, hide_ranges: bool) -> String {
    let mut printed_line = String::new();
    if !hide_ranges {
        let line_num_len = (block_len.checked_ilog(16).unwrap_or(0) + 1) as usize;
        let line_start = line_num * line.len();
        let line_end = std::cmp::min(line_start + line.len(), block_len);
        printed_line.push_str(&format!("<{}-{}>   ", to_str_radix(line_start, 16, line_num_len), to_str_radix(line_end, 16, line_num_len)));
    }
    line.chunks(column_len).for_each(|column| {
        column.iter().for_each(|b| {
            printed_line.push_str(&to_str_radix(*b as usize, 16, 2));
            printed_line.push(' ');
        });
        printed_line.push_str("  ");
    });
    if !no_decode {
        let missing_byte_count = (columns * column_len) - line.len();
        let missing_col_count = missing_byte_count / column_len;
        if missing_byte_count != 0 {
            let mut padding_string = String::new();
            for _ in 0..missing_byte_count {
                padding_string.push_str("   ");
            }
            for _ in 0..missing_col_count {
                padding_string.push_str("  ");
            }
            printed_line.push_str(&padding_string);
        }
        line.iter().for_each(|b| {
            let c = *b as char;
            if c.is_alphanumeric() {
                printed_line.push(c);
            } else {
                printed_line.push('.');
            }
        });
    }
    printed_line
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
