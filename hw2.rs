use std::collections::HashMap;
use std::io::{self, Write};

/// Function that counts the frequency of characters in the given string
/// 
/// # Arguments
/// * `input` - A string slice containing the input string
/// 
/// # Returns
/// * A tuple where the first value is the character that occurs the most and
///   the second value is the number of occurrences
fn most_frequent_char(input: &str) -> Option<(char, usize)> {
    let mut char_count = HashMap::new();

    // Count occurrences of each character
    for c in input.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // Find the character with the highest count
    char_count.into_iter().max_by_key(|&(_, count)| count)
}

fn main() {
    // Ask the user for input
    print!("Enter a string: ");
    io::stdout().flush().unwrap(); // Flush to ensure print statement is shown before input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim(); // Remove any extra whitespace

    // Find and print the most frequent character
    if let Some((char, count)) = most_frequent_char(&input) {
        println!("The character '{}' appears the most with {} occurrences.", char, count);
    } else {
        println!("No characters found.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_frequent_char_single() {
        let input = "hello";
        let result = most_frequent_char(input);
        assert_eq!(result, Some(('l', 2)));
    }

    #[test]
    fn test_most_frequent_char_tied() {
        let input = "average";
        let result = most_frequent_char(input);
        assert_eq!(result, Some(('e', 2))); // 'e' comes is repeated twice
    }

    #[test]
    fn test_most_frequent_char_empty() {
        let input = "";
        let result = most_frequent_char(input);
        assert_eq!(result, None);
    }
}
