// Problem: Shifting Letters II
// Tags: Array, Sorting, Prefix Sum

pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    let len = s.len();
    let mut shifts_str = s
        .chars()
        .map(|c| (c as u8 - b'a') as i32) // Convert 'a'-'z' to 0-25
        .collect::<Vec<i32>>();
    let mut shift_effects = vec![0; len + 1];

    // Apply shifts using prefix sum
    for shift in &shifts {
        let (start, end, direction) = (shift[0] as usize, shift[1] as usize, shift[2]);
        if direction == 1 {
            shift_effects[start] += 1;
            if end + 1 < len {
                shift_effects[end + 1] -= 1;
            }
        } else {
            shift_effects[start] -= 1;
            if end + 1 < len {
                shift_effects[end + 1] += 1;
            }
        }
    }

    // Calculate cumulative shifts
    let mut net_shift = 0;
    for i in 0..len {
        net_shift += shift_effects[i];
        net_shift %= 26; // Ensure cumulative shifts stay within range
        shifts_str[i] = (shifts_str[i] + net_shift + 26) % 26; // Wrap shifts to 0-25 range
    }

    // Convert back to characters
    shifts_str
        .iter()
        .map(|&i| (b'a' + i as u8) as char) // Convert 0-25 back to 'a'-'z'
        .collect::<String>()
}
