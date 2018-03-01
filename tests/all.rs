extern crate rule;
use rule::Rule;

#[test]
fn all()
{
    let code = "abcdefg";
    
    let f = |_: Vec<bool>, l: &str| {
        assert_eq!(l, "abcdefg");
        vec![true, false, false, true]
    };
    
    let mut r: Rule<bool> = Rule::new(Some(Box::new(f)));
    r.any_char().any_char().any_char().any_char().any_char().any_char().any_char();
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], true);
        assert_eq!(branches[1], false);
        assert_eq!(branches[2], false);
        assert_eq!(branches[3], true);
    }
    else {
        assert!(false);
    }
}