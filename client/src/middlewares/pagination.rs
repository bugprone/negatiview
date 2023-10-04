pub fn limit(count: usize, page: usize) -> String {
    let offset = if page > 0 { page * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
