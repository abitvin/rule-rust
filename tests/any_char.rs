use rule::Rule;

#[test]
fn all() {
    let code = "abcdefg";
    
    let f = |_, l: &str| {
        assert_eq!(l, "abcdefg");
        Ok(true)
    };
    
    let r: Rule<bool> = Rule::new(f);
    r.any_char().any_char().any_char().any_char().any_char().any_char().any_char();
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], true);
    }
    else {
        assert!(false);
    }
}