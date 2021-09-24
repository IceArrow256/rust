fn entropy(alpha: &str, key: &str) -> f32 {
    return (alpha.len().pow(key.len() as u32) as f32).log2();
}

fn caesar_cipher(alpha: &str, key: i16, value: &str) -> String {
    let mut res = String::new();
    for c in value.chars() {
        res.push(
            alpha
                .chars()
                .nth(((alpha.find(c).unwrap() as i16 + key) % alpha.len() as i16) as usize)
                .unwrap(),
        );
    }
    return res;
}

fn vigenere_cipher(alpha: &str, key: &str, value: &str) -> String {
    let mut i: usize = 0;
    let mut res = String::new();
    for c in value.chars() {
        let shift = alpha.find(key.chars().nth(i).unwrap()).unwrap() as i16;
        res.push(
            alpha
                .chars()
                .nth(((alpha.find(c).unwrap() as i16 + shift) % alpha.len() as i16) as usize)
                .unwrap(),
        );
        i = i + 1 % key.len();
    }
    return res;
}

fn print_line(cipher: &str, alphabet: &str, key: &str, value: &str, result: &str, entropy: f32) {
    println!(
        "| {:<15} | {:<62} | {:<10} | {:<10} | {:<10} | {:<10.4} |",
        cipher, alphabet, key, value, result, entropy
    );
}

fn main() {
    let alpha_lower = "abcdefghijklmnopqrstuvwxyz";
    let alpha_all = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let alpha_number = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    println!("{:-<136}", "");
    println!("| {:<15} | {:<62} | {:<10} | {:<10} | {:<10} | {:<10} |", "Cipher", "Alphabet", "Key", "Value", "Result", "Entropy");
    println!("{:-<136}", "");
    // Caesar
    print_line(
        "Caesar's",
        alpha_lower,
        &1.to_string(),
        "abc",
        &caesar_cipher(alpha_lower, 1, "abc"),
        entropy(alpha_lower, "1"),
    );
    print_line(
        "Caesar's",
        alpha_all,
        &1.to_string(),
        "AbC",
        &caesar_cipher(alpha_all, 1, "AbC"),
        entropy(alpha_all, "1"),
    );
    println!("{:-<136}", "");
    // Vigenere
    print_line(
        "Vigenère's",
        alpha_lower,
        "xyz",
        "abc",
        &vigenere_cipher(alpha_lower, "xyz", "abc"),
        entropy(alpha_lower, "xyz"),
    );
    print_line(
        "Vigenère's",
        alpha_all,
        "xyz",
        "AbC",
        &vigenere_cipher(alpha_all, "xyz", "AbC"),
        entropy(alpha_all, "xyz"),
    );
    print_line(
        "Vigenère's",
        alpha_number,
        "xyz123",
        "AbC123",
        &vigenere_cipher(alpha_number, "xyz123", "AbC123"),
        entropy(alpha_number, "xyz123"),
    );
    println!("{:-<136}", "");
    print_line(
        "Vigenère's",
        alpha_lower,
        "xyz",
        "abc",
        &vigenere_cipher(alpha_lower, "xyz", &caesar_cipher(alpha_lower, 5, "abc")),
        entropy(alpha_lower, "xyz"),
    );
    print_line(
        "Vigenère's",
        alpha_all,
        "xyz",
        "AbC",
        &vigenere_cipher(alpha_all, "xyz", &caesar_cipher(alpha_all, 5, "AbC")),
        entropy(alpha_all, "xyz"),
    );
    print_line(
        "Vigenère's",
        alpha_number,
        "xyz123",
        "AbC123",
        &vigenere_cipher(
            alpha_number,
            "xyz123",
            &caesar_cipher(alpha_number, 5, "AbC123"),
        ),
        entropy(alpha_number, "xyz123"),
    );
    println!("{:-<136}", "");
}