use rule::Rule;

#[test]
fn none_or_many() {
    let dot = Rule::new(|_, _| Ok(true));
    dot.literal(".");
            
    let x = Rule::new(|_, _| Ok(false));
    x.literal("x");
            
    let code1: Rule<bool> = Rule::new(|b, l| {
        assert_eq!(b.len(), 0);
        assert_eq!(l, "");
        Ok(false)
    });
    
    let code2: Rule<bool> = Rule::new(|b, l| {
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], false);
        assert_eq!(l, "x");
        Ok(false)
    });
    
    let code3: Rule<bool> = Rule::new(|b, l| {
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], true);
        assert_eq!(b[1], true);
        assert_eq!(l, "..");
        Ok(false)
    });
    
    let code4: Rule<bool> = Rule::new(|b, l| {
        assert_eq!(b.len(), 3);
        assert_eq!(b[0], false);
        assert_eq!(b[1], false);
        assert_eq!(b[2], true);
        assert_eq!(l, "xx.");
        Ok(false)
    });
    
    let code5: Rule<bool> = Rule::new(|b, l| {
        assert_eq!(b.len(), 4);
        assert_eq!(b[0], true);
        assert_eq!(b[1], true);
        assert_eq!(b[2], false);
        assert_eq!(b[3], false);
        assert_eq!(l, "..xx");
        Ok(false)
    });

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

#[test]
fn none_or_many_scan_empty_should_succeed() {
    let monkey = Rule::default();
    monkey.literal("monkey");

    let none_or_many_monkeys: Rule<i32> = Rule::default();
    none_or_many_monkeys.none_or_many(&monkey);

    assert!(none_or_many_monkeys.scan("").is_ok());
}