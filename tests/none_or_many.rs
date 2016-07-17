extern crate rule;
use rule::Rule;

#[test]
fn none_or_many()
{
    let mut dummy = false;

    let mut dot = Rule::new(Some(Box::new(|_, _, _| vec![true])));
    dot.literal(".");
            
    let mut x = Rule::new(Some(Box::new(|_, _, _| vec![false])));
    x.literal("x");
            
    let mut code1: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
    {
        assert_eq!(b.len(), 0);
        assert_eq!(l, "");
        Vec::new()
    })));
    
    let mut code2: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
    {
        assert_eq!(b.len(), 1);
        assert_eq!(b[0], false);
        assert_eq!(l, "x");
        Vec::new()
    })));
    
    let mut code3: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
    {
        assert_eq!(b.len(), 2);
        assert_eq!(b[0], true);
        assert_eq!(b[1], true);
        assert_eq!(l, "..");
        Vec::new()
    })));
    
    let mut code4: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
    {
        assert_eq!(b.len(), 3);
        assert_eq!(b[0], false);
        assert_eq!(b[1], false);
        assert_eq!(b[2], true);
        assert_eq!(l, "xx.");
        Vec::new()
    })));
    
    let mut code5: Rule<bool, bool> = Rule::new(Some(Box::new(|b, l, _|
    {
        assert_eq!(b.len(), 4);
        assert_eq!(b[0], true);
        assert_eq!(b[1], true);
        assert_eq!(b[2], false);
        assert_eq!(b[3], false);
        assert_eq!(l, "..xx");
        Vec::new()
    })));
    
    unsafe {
        if let Err(_) = code1.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("", &mut dummy) {
            assert!(false);
        }
        
        if let Err(_) = code2.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("x", &mut dummy) {
            assert!(false);
        }
        
        if let Err(_) = code3.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("..", &mut dummy) {
            assert!(false);
        }
        
        if let Err(_) = code4.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("xx.", &mut dummy) {
            assert!(false);
        }
        
        if let Err(_) = code5.none_or_many_raw(&dot).none_or_many_raw(&x).none_or_many_raw(&dot).scan("..xx", &mut dummy) {
            assert!(false);
        }
    }
}