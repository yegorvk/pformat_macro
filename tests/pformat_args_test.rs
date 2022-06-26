use pformat_macro::pformat_args;

#[test]
fn test_pformat_args_plain_string() {
    assert_eq!(format!("{}", pformat_args!("Hello World!")), "Hello World!")
}

#[test]
fn test_pformat_args() {
    assert_eq!(
        format!(
            "{}",
            pformat_args!("Hello, {}. 1 + 1 = {} for sure!", "Bob", 3)
        ),
        "Hello, Bob. 1 + 1 = 3 for sure!"
    )
}

#[test]
fn test_empty_string() {
    assert_eq!(format!("{}", pformat_args!("")), "")
}
