use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Parse the input puzzle.
    // Format: "WORD1 + WORD2 + ... == RESULT"
    let parts: Vec<&str> = input.split("==").collect();
    if parts.len() != 2 {
        return None;
    }

    let left_side = parts[0];
    let right_side = parts[1].trim();

    let addends: Vec<&str> = left_side.split('+').map(|s| s.trim()).collect();

    // Collect all unique characters and identify leading characters (which cannot be 0).
    let mut letters_set = HashSet::new();
    let mut leading_chars = HashSet::new();

    let mut add_word = |word: &str| {
        let bytes = word.as_bytes();
        if !bytes.is_empty() {
            leading_chars.insert(bytes[0] as char);
            for &b in bytes {
                letters_set.insert(b as char);
            }
        }
    };

    for addend in &addends {
        add_word(addend);
    }
    add_word(right_side);

    let letters: Vec<char> = letters_set.into_iter().collect();
    if letters.len() > 10 {
        return None; // Can't map > 10 unique letters to digits 0-9
    }

    // Precompute coefficients for each letter to make evaluation fast.
    // A word like "SEND" contributes S * 1000 + E * 100 + N * 10 + D * 1.
    // Left side words add positive coefficients, right side words subtract positive coefficients.
    // Total sum of (coefficient * digit) must equal 0.
    let mut coefficients: HashMap<char, i64> = HashMap::new();

    let mut process_word = |word: &str, sign: i64| {
        let bytes = word.as_bytes();
        let len = bytes.len();
        for (i, &b) in bytes.iter().enumerate() {
            let power = (len - 1 - i) as u32;
            let multiplier = 10_i64.pow(power) * sign;
            *coefficients.entry(b as char).or_insert(0) += multiplier;
        }
    };

    for addend in &addends {
        process_word(addend, 1);
    }
    process_word(right_side, -1);

    // To prune search space effectively, order letters by the absolute magnitude of their coefficient (descending).
    let mut sorted_letters: Vec<(char, i64)> = letters
        .into_iter()
        .map(|c| (c, *coefficients.get(&c).unwrap_or(&0)))
        .collect();
    sorted_letters.sort_by(|a, b| b.1.abs().cmp(&a.1.abs()));

    let ordered_chars: Vec<char> = sorted_letters.iter().map(|(c, _)| *c).collect();
    let ordered_coeffs: Vec<i64> = sorted_letters.iter().map(|(_, coeff)| *coeff).collect();

    let mut assignment: HashMap<char, u8> = HashMap::new();
    let mut used_digits: u16 = 0; // bitmask of digits 0-9

    fn backtrack(
        index: usize,
        current_sum: i64,
        ordered_chars: &[char],
        ordered_coeffs: &[i64],
        leading_chars: &HashSet<char>,
        assignment: &mut HashMap<char, u8>,
        used_digits: &mut u16,
    ) -> Option<HashMap<char, u8>> {
        if index == ordered_chars.len() {
            return if current_sum == 0 {
                Some(assignment.clone())
            } else {
                None
            };
        }

        // Optional pruning: check remaining bounds if needed, but backtracking with sorted coeffs is extremely fast for alphametics.
        let ch = ordered_chars[index];
        let coeff = ordered_coeffs[index];
        let is_leading = leading_chars.contains(&ch);

        // Try digits 0-9 (or 1-9 if leading)
        let start_digit = if is_leading { 1 } else { 0 };

        for digit in start_digit..=9 {
            let mask = 1u16 << digit;
            if (*used_digits & mask) == 0 {
                *used_digits |= mask;
                assignment.insert(ch, digit);

                let next_sum = current_sum + coeff * (digit as i64);

                if let Some(res) = backtrack(
                    index + 1,
                    next_sum,
                    ordered_chars,
                    ordered_coeffs,
                    leading_chars,
                    assignment,
                    used_digits,
                ) {
                    return Some(res);
                }

                assignment.remove(&ch);
                *used_digits &= !mask;
            }
        }

        None
    }

    backtrack(
        0,
        0,
        &ordered_chars,
        &ordered_coeffs,
        &leading_chars,
        &mut assignment,
        &mut used_digits,
    )
}
