// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


use std::io::Bytes;

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let bytes = input.as_bytes();
    let mut left_trim: &str = "";
    let mut last_char_pos: usize = 0;
    for (i ,&item) in bytes.iter().enumerate(){
        if item != b' ' && left_trim == "" {left_trim = &input[i..];};
        if item != b' ' { last_char_pos = i+1;}
    }
    left_trim[..left_trim.len() + last_char_pos - input.len()].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut ans: String = "".to_string();
    for i in 0..input.len(){
        if &input[i..i+4] == "cars"{
            ans = (&input[..i]).to_string() + "balloons" +&input[i+4..];
            break
        }
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
