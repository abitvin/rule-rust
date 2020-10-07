use rule::Rule;

#[test]
fn at_least() {
    let code = "xxxx";
    
    let x = Rule::new(|_, _| 10);
    x.literal("x");
    
    let test1: Rule<i32> = Rule::default();
    test1.at_least(3, &x);
    
    if let Ok(branches) = test1.scan(&code) {
        assert_eq!(branches[0], 10);
        assert_eq!(branches[1], 10);
        assert_eq!(branches[2], 10);
        assert_eq!(branches[3], 10);
    }
    else {
        assert!(false);
    }

    let test2: Rule<i32> = Rule::default();
    test2.at_least(4, &x);
    
    if let Ok(branches) = test2.scan(&code) {
        assert_eq!(branches[0], 10);
        assert_eq!(branches[1], 10);
        assert_eq!(branches[2], 10);
        assert_eq!(branches[3], 10);
    }
    else {
        assert!(false);
    }

    let test3: Rule<i32> = Rule::default();
    test3.at_least(5, &x);
    
    if let Ok(_) = test3.scan(&code) {
        assert!(false);
    }
    else {
        assert!(true);
    }
}