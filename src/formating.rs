
pub fn as_f64_formated(num: f64) -> String {
    let mut precision = 0;

    for ch in num.abs().fract().to_string()[2..].chars() {
        precision += 1;
        if is_zero(ch) {
            continue
        } else {
            precision += 1;
            break;
        }
    }

    format!("{:.1$}", num, precision)
}

fn is_zero(ch: char) -> bool {
    match ch {
        '0' => true,
        _ => false
    }
}