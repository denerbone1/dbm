use dbm::converter::{convert, convert_expression};

#[test]
fn test_positive_result() {
    assert_eq!(
        convert_expression("2~3").unwrap(),
        "{|-1-1| ~ |-2-1|} неа = {5}"
    );
}

#[test]
fn test_negative_result() {
    assert_eq!(
        convert_expression("2|||5").unwrap(),
        "{|-1-1| ||| |-4-1|} -неа = {3}"
    );
}

#[test]
fn test_zero_result() {
    assert_eq!(
        convert_expression("2|||2").unwrap(),
        "{|-1-1| ||| |-1-1|} неа = {0}"
    );
}

#[test]
fn test_negative_zero() {
    assert_eq!(
        convert_expression("-0").unwrap(),
        "{-{()}} неа = {0}"
    );
}

#[test]
fn test_convert_number() {
    assert_eq!(convert(5.0).unwrap(), "|-4-1|");
}