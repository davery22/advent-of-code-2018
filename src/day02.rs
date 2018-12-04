use std::collections::HashMap;

pub fn run() {
    let box_ids = include_str!("../input/day02.in").lines().collect();
    println!("{}", calculate_checksum(&box_ids));
    println!("{}", get_letters_in_common(&box_ids));
}

pub fn calculate_checksum(box_ids: &Vec<&str>) -> i32 {
    let mut num_ids_with_2_of_any_letter = 0;
    let mut num_ids_with_3_of_any_letter = 0;

    for box_id in box_ids.iter() {
        let encoding = encode_char_frequency(box_id);

        if encoding.values().any(|freq| *freq == 2u8) {
            num_ids_with_2_of_any_letter += 1;
        }

        if encoding.values().any(|freq| *freq == 3u8) {
            num_ids_with_3_of_any_letter += 1;
        }
    }

    num_ids_with_2_of_any_letter * num_ids_with_3_of_any_letter
}

pub fn encode_char_frequency(input: &str) -> HashMap<char, u8> {
    // Several ways to do this - pick your poison

    input.chars()
        .fold(HashMap::new(), |mut encoding, next_char| {
            *encoding.entry(next_char).or_insert(0) += 1;
            encoding
        })

//    let mut encoding = HashMap::new();
//
//    for next_char in input.chars() {
//        *encoding.entry(next_char).or_insert(0) += 1;
//    }
//
//    encoding

//    use std::str::Chars;
//
//    fn helper(mut encoding: HashMap<char, u8>, mut chars: Chars) -> HashMap<char, u8> {
//        match chars.next() {
//            Some(next_char) => helper({ *encoding.entry(next_char).or_insert(0) += 1; encoding }, chars),
//            None => encoding
//        }
//    }
//
//    helper(HashMap::new(), input.chars())
}

pub fn get_letters_in_common(box_ids: &Vec<&str>) -> String {
    for (i, this_box_id) in box_ids.iter().enumerate() {
        for that_box_id in box_ids.iter().skip(i) {
            let matching_chars: String = this_box_id.chars()
                .zip(that_box_id.chars())
                .filter(|(this_char, that_char)| this_char == that_char)
                .map(|(this_char, _)| this_char)
                .collect();

            if matching_chars.len() == this_box_id.len() - 1 {
                return matching_chars;
            }
        }
    }

    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encodes_empty_string() {
        assert_eq!(HashMap::new(), encode_char_frequency(""));
    }

    #[test]
    fn test_encodes_string() {
        let mut encoding = HashMap::new();
        encoding.insert('a', 1);
        encoding.insert('b', 2);
        encoding.insert('c', 2);
        encoding.insert('d', 3);

        assert_eq!(encoding, encode_char_frequency("abcbdcdd"));
    }
}
