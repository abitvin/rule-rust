extern crate rule;
use rule::Rule;

#[test]
fn all()
{
    let mut dummy = 80085; 
    let code = "abcdefg";
    
    let f = |_: Vec<bool>, l: &str, _: &mut i32| {
        assert_eq!(l, "abcdefg");
        vec![true, false, false, true]
    };
    
    let mut r: Rule<bool, i32> = Rule::new(Some(Box::new(f)));
    r.all().all().all().all().all().all().all();
    
    if let Ok(branches) = r.scan(&code, &mut dummy) {
        assert_eq!(branches[0], true);
        assert_eq!(branches[1], false);
        assert_eq!(branches[2], false);
        assert_eq!(branches[3], true);
    }
    else {
        assert!(false);
    }
}