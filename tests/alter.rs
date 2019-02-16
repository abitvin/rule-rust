use rule::Rule;

#[test]
fn alter() {
    let code = "\\<東\\<💝\\>中\\>"; // There are gonna be 7 replacements.
    
    let alterations = vec![
        ("\\<", "<"),
        ("\\>", ">"),
        ("東", "AAA"),
        ("💝", "BBB"),
        ("中", "CCC"),
    ];
    
    let a = Rule::new(None);
    a.alter(alterations);

    let f = |_, l: &str| {
        assert_eq!(l, "<AAA<BBB>CCC>");
        111
    }; 
    
    let r: Rule<i32> = Rule::new(Some(Box::new(f)));
    r.exact(7, &a);
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], 111);
    }
    else {
        assert!(false);
    }
}