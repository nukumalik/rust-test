use rust_test::utils::get_offset_from_page;

#[test]
fn test_get_offset_from_page() {
    assert_eq!(get_offset_from_page(None), 0);
    assert_eq!(get_offset_from_page(Some(1)), 0);
    assert_eq!(get_offset_from_page(Some(2)), 10);
}
