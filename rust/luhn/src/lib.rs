/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.to_string();
    let count = code.len();
    if count < 2 {
        return false;
    }
    println!("{code}");
    let mut result: u32 = 0;
    let mut tmp: u32;
    for (index, character) in code.chars().rev().enumerate() {
        if character.is_whitespace() {
            continue;
        }
        println!("{index} {character}");
        if index % 2 == 0 {
            tmp = character.to_digit(10).unwrap() * 2;
            if tmp > 9 {
                tmp -= 9;
            }
            println!("tmp = {tmp}");
            result += tmp;
        } else {
            tmp = character.to_digit(10).unwrap();
            println!("tmp = {tmp}");
            result += tmp;
        }
        println!("result = {result}");
    }
    result % 10 == 0
}
