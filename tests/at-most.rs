use rule::Rule;

#[test]
fn at_most() {
    let code = "yyy";
    
    let y = Rule::new(Box::new(|_, _| 14));
    y.literal("y");
            
    let test1: Rule<i32> = Rule::default();
    test1.at_most(2, &y);
    
    if let Ok(_) = test1.scan(&code) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    let test2: Rule<i32> = Rule::default();
    test2.at_most(3, &y);
    
    if let Ok(branches) = test2.scan(&code) {
        assert_eq!(branches[0], 14);
        assert_eq!(branches[1], 14);
        assert_eq!(branches[2], 14);
    }
    else {
        assert!(false);
    }

    let test3: Rule<i32> = Rule::default();
    test3.at_most(4, &y);
    
    if let Ok(branches) = test3.scan(&code) {
        assert_eq!(branches[0], 14);
        assert_eq!(branches[1], 14);
        assert_eq!(branches[2], 14);
    }
    else {
        assert!(false);
    }
}