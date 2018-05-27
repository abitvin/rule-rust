extern crate rule;
use rule::Rule;

#[test]
fn literal() {
    let code = "y̆y̆y̆x̆";
    
    let r: Rule<u64> = Rule::new(Some(Box::new(|_, l| {
        assert_eq!(l, "y̆y̆y̆x̆");
        vec![7777u64, 8888u64, 9999u64]
    })));
    
    r.literal("y̆y̆").literal("y̆").literal("x̆");
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], 7777u64);
        assert_eq!(branches[1], 8888u64);
        assert_eq!(branches[2], 9999u64);
    }
    else {
        assert!(false);
    }
}