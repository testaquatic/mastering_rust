fn convert_first_char_to_hex(input: Option<String>) -> Result<String, String> {
    let Some(s) = input else {
        return Err("No input provided".to_string());
    };

    if let Some(first_char) = s.chars().next() {
        if first_char.is_ascii_hexdigit() {
            Ok(format!("0x{:x}", first_char.to_digit(16).unwrap()))
        } else {
            Err("First character is not a valid hexadecimal digit".to_string())
        }
    } else {
        Err("No first character".to_string())
    }
}

fn main() {
    let input = Some("a123".to_string());
    match convert_first_char_to_hex(input) {
        Ok(hex) => println!("First character in hex: {}", hex),
        Err(err) => println!("Error: {}", err),
    }

    let empty_input = Some("B".to_string());
    match convert_first_char_to_hex(empty_input) {
        Ok(hex) => println!("First character in hex: {}", hex),
        Err(err) => println!("Error: {}", err),
    }

    let no_inut = None;
    match convert_first_char_to_hex(no_inut) {
        Ok(hex) => println!("First character in hex: {}", hex),
        Err(err) => println!("Error: {}", err),
    }
}
