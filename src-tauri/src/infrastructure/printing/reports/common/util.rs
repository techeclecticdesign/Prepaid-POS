use std::borrow::Cow;

pub fn format_cents(cents: i32) -> String {
    let dollars = cents / 100;
    let remainder = (cents.abs() % 100) as u8;
    format!("${dollars}.{remainder:02}")
}

pub fn truncate_desc(desc: &str, max_len: usize) -> Cow<'_, str> {
    let char_count = desc.chars().count();
    if char_count <= max_len {
        Cow::Borrowed(desc)
    } else {
        let mut s: String = desc.chars().take(max_len).collect();
        s.push('…');
        Cow::Owned(s)
    }
}
