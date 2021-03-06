use rule::Rule;

#[test]
fn literal() {
    let code = "y̆y̆y̆x̆";
    
    let r: Rule<u64> = Rule::new(|_, l| {
        assert_eq!(l, "y̆y̆y̆x̆");
        Ok(7777u64)
    });
    
    r.literal("y̆y̆").literal("y̆").literal("x̆");
    
    if let Ok(branches) = r.scan(&code) {
        assert_eq!(branches[0], 7777u64);
    }
    else {
        assert!(false);
    }
}

#[test]
#[should_panic]
fn empty_literal_should_panic() {
    let empty: Rule<i32> = Rule::default();
    empty.literal("");
}