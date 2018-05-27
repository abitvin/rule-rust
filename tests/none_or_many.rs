extern crate rule;
use rule::Rule;

#[test]
fn none_or_many() {
    let dot = Rule::new(Some(Box::new(|_, _| vec![true])));
    dot.literal(".");
            
    let x = Rule::new(Some(Box::new(|_, _| vec![false])));
    x.literal("x");
            
    let code1: Rule<bool> = Rule::new(Some(Box::new(|b, l| {
        assert_eq!(b.len(), 0);
        assert_eq!(l, "");
        Vec::new()
    })));
    
    let code2: Rule<bool> = Rule::new(Some(Box::new(|b, l| {
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], false);
        assert_eq!(l, "x");
        Vec::new()
    })));
    
    let code3: Rule<bool> = Rule::new(Some(Box::new(|b, l| {
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], true);
        assert_eq!(b[1], true);
        assert_eq!(l, "..");
        Vec::new()
    })));
    
    let code4: Rule<bool> = Rule::new(Some(Box::new(|b, l| {
        assert_eq!(b.len(), 3);
        assert_eq!(b[0], false);
        assert_eq!(b[1], false);
        assert_eq!(b[2], true);
        assert_eq!(l, "xx.");
        Vec::new()
    })));
    
    let code5: Rule<bool> = Rule::new(Some(Box::new(|b, l| {
        assert_eq!(b.len(), 4);
        assert_eq!(b[0], true);
        assert_eq!(b[1], true);
        assert_eq!(b[2], false);
        assert_eq!(b[3], false);
        assert_eq!(l, "..xx");
        Vec::new()
    })));

    if let Err(_) = code1.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("") {
        assert!(false);
    }
    
    if let Err(_) = code2.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("x") {
        assert!(false);
    }
    
    if let Err(_) = code3.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("..") {
        assert!(false);
    }
    
    if let Err(_) = code4.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("xx.") {
        assert!(false);
    }
    
    if let Err(_) = code5.none_or_many(&dot).none_or_many(&x).none_or_many(&dot).scan("..xx") {
        assert!(false);
    }
}