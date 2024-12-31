use arrayvec::Array;
use arrayvec::ArrayString;

pub fn truncated_arraystring<A: Array<Item = u8> + Copy>(mut s: &str) -> ArrayString<A> {
    let mut result = ArrayString::new();
    if s.len() > result.capacity() {
        for n in (0..result.capacity() + 1).rev() {
            if s.is_char_boundary(n) {
                s = &s[..n];
                break;
            }
        }
    }
    result.push_str(s);
    result
}
