#![feature(nll)]

extern crate rule;
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

    let f = |_: Vec<i32>, l: &str| {
        assert_eq!(l, "<AAA<BBB>CCC>");
        vec![111, 222]
    }; 
    
    let r: Rule<i32> = Rule::new(Some(Box::new(f)));
    r.exact(7, &a);
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], 111);
        assert_eq!(branches[1], 222);
    }
    else {
        assert!(false);
    }
}