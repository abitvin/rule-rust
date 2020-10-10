use rule::Rule;

#[test]
fn any_char_except() {
    let code = "a💝c";

    let f = |_, l: &str| {
        assert_eq!(l, "a💝c");
        Ok(123)
    };
    
    let c = Rule::default();
    c.any_char_except(vec!['A', 'B', '中', '東']);
    
    let r: Rule<u32> = Rule::new(f);
    r.exact(3, &c);
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], 123);
    }
    else {
        assert!(false);
    }
}