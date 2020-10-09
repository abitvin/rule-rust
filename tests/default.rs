use rule::Rule;

#[test]
#[should_panic]
fn default_rule_should_panic_when_scanned() {
    let empty: Rule<i32> = Rule::default();
    assert!(empty.scan("").is_ok());
}