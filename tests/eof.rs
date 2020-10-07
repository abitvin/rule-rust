use rule::Rule;

#[test]
fn eof() {
    let code = "123";
    
    let r: Rule<char> = Rule::new(|_, _| 'A');
    r.literal("123").eof();
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], 'A');
    }
    else {
        assert!(false);
    }
}