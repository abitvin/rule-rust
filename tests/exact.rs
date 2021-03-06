use rule::Rule;

#[test]
fn exact() {
    let code = "..........";
    
    let dot = Rule::new(|_, _| Ok('.'));
    dot.literal(".");
            
    let nope = Rule::new(|_, _| Ok('x'));
    nope.literal("nope");
            
    let test1: Rule<char> = Rule::default();
    test1.exact(10, &dot);
    
    if let Ok(branches) = test1.scan(&code) {
        assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
    }
    else {
        assert!(false);
    }

    let test2: Rule<char> = Rule::default();
    test2.exact(9, &dot);
    
    if let Ok(_) = test2.scan(&code) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    let test3: Rule<char> = Rule::default();
    test3.exact(11, &dot);

    if let Ok(_) = test3.scan(&code) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    let test4: Rule<char> = Rule::default();
    test4.exact(0, &nope).exact(10, &dot).exact(0, &nope);

    if let Ok(branches) = test4.scan(&code) {
        assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
    }
    else {
        assert!(false);
    }
}