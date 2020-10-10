extern crate rule;
use rule::Rule;

#[test]
fn error_1() {
    let root: Rule<i32> = Rule::new(|_, _| Err(String::from("Monkey bad.")));
    root.literal("monkey");

    let result = format!("{}", root.scan("monkey").unwrap_err());
    let expect = "Error found at line 1, column 0: Monkey bad.".to_string();

    assert_eq!(result, expect);
}

#[test]
fn error_2() {
    let monkey = Rule::default();
    monkey.literal("monkey");

    let gorilla = Rule::new(|_, _| Err(String::from("Gorilla bad.")));
    gorilla.literal("gorilla");

    let root: Rule<i32> = Rule::default();
    root.one(&monkey).one(&gorilla);

    let result = format!("{}", root.scan("monkeygorilla").unwrap_err());
    let expect = "Error found at line 1, column 6: Gorilla bad.".to_string();

    assert_eq!(result, expect);
}