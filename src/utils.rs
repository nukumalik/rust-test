pub fn get_offset_from_page(page: Option<u32>) -> u32 {
    page.map(|value| value - 1).unwrap_or(0) * 10
}
