use rule::Rule;

#[test]
fn between() {
    let code = "zzz";
    
    let z = Rule::new(|_, _| Ok(34));
    z.literal("z");
            
    let test1: Rule<i32> = Rule::default();
    test1.between(1, 3, &z);

    if let Ok(branches) = test1.scan(&code) {
        assert_eq!(branches[0], 34);
        assert_eq!(branches[1], 34);
        assert_eq!(branches[2], 34);
    }
    else {
        assert!(true);
    }

    let test2: Rule<i32> = Rule::default();
    test2.between(0, 10, &z);
    
    if let Ok(branches) = test2.scan(&code) {
        assert_eq!(branches[0], 34);
        assert_eq!(branches[1], 34);
        assert_eq!(branches[2], 34);
    }
    else {
        assert!(false);
    }

    let test3: Rule<i32> = Rule::default();
    test3.between(4, 5, &z);
    
    if let Ok(_) = test3.scan(&code) {
        assert!(false);
    }
    else {
        assert!(true);
    }
}