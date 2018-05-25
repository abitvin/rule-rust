#![feature(nll)]

extern crate rule;
use rule::Rule;

#[test]
fn exact() {
    let code = "..........";
    
    let dot = Rule::new(Some(Box::new(|_, _| vec!['.'] )));
    dot.literal(".");
            
    let nope = Rule::new(Some(Box::new(|_, _| vec!['x'] )));
    nope.literal("nope");
            
    let test1: Rule<char> = Rule::new(None);
    test1.exact(10, &dot);
    
    if let Ok(branches) = test1.scan(&code) {
        assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
    }
    else {
        assert!(false);
    }

    let test2: Rule<char> = Rule::new(None);
    test2.exact(9, &dot);
    
    if let Ok(_) = test2.scan(&code) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    let test3: Rule<char> = Rule::new(None);
    test3.exact(11, &dot);

    if let Ok(_) = test3.scan(&code) {
        assert!(false);
    }
    else {
        assert!(true);
    }

    let test4: Rule<char> = Rule::new(None);
    test4.exact(0, &nope).exact(10, &dot).exact(0, &nope);

    if let Ok(branches) = test4.scan(&code) {
        assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
    }
    else {
        assert!(false);
    }
}