use rule::Rule;

#[test]
fn one() {
    let code = "onetwothree";
    
    let one: Rule<i32> = Rule::new(Some(Box::new(|_, _| 1)));
    one.literal("one");
    
    let two: Rule<i32> = Rule::new(Some(Box::new(|_, _| 2)));
    two.literal("two");
    
    let three: Rule<i32> = Rule::new(Some(Box::new(|_, _| 3)));
    three.literal("three");
    
    let root: Rule<i32> = Rule::new(None);
    root.one(&one).one(&two).one(&three);
    
    if let Ok(branches) = root.scan(&code) {
        assert_eq!(branches[0], 1);
        assert_eq!(branches[1], 2);
        assert_eq!(branches[2], 3);
    }
    else {
        assert!(false);
    }
}