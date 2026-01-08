use colored::Colorize;

pub fn format_sol(amount: f64) -> String {
    format!("{:.4} SOL", amount)
}

pub fn format_usd(amount: f64) -> String {
    let formatted = if amount >= 1_000.0 {
        let whole = amount as u64;
        let frac = ((amount - whole as f64) * 100.0).round() as u64;
        let whole_str = add_thousands_separator(whole);
        format!("${}.{:02}", whole_str, frac)
    } else {
        format!("${:.2}", amount)
    };
    formatted
}

fn add_thousands_separator(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}

