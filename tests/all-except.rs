extern crate rule;
use rule::Rule;

#[test]
fn all_except() {
    let code = "abc";

    let f = |_: Vec<u32>, l: &str| {
        assert_eq!(l, "abc");
        123
    };
    
    let c = Rule::new(None);
    c.any_char_except(vec!['A', 'B', 'C', 'D']);
    
    let r: Rule<u32> = Rule::new(Some(Box::new(f)));
    r.exact(3, &c);
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], 123);
    }
    else {
        assert!(false);
    }
}