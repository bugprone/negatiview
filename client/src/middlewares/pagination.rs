pub fn limit(count: u32, page: u32) -> String {
    let offset = if page > 0 { page * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
