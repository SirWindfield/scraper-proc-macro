use scraper_proc_macro::selector;

#[test]
fn parses_valid_selector() {
    let _ = selector!("div");
}

#[test]
#[should_panic]
fn panics_on_invalid_selector() {
    let _ = selector!("div ..");
}
