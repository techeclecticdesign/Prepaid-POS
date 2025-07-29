pub fn format_cents(cents: i32) -> String {
    let dollars = cents / 100;
    let remainder = (cents.abs() % 100) as u8;
    format!("${}.{:02}", dollars, remainder)
}
