extern crate rule;
use rule::Rule;

struct Shared {
    number: i32,
}

#[test]
fn shared_state()
{
    let mut shared = Shared { 
        number: 0 
    };

    let mut a: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: Vec<i32>, _: &str, s: &mut Shared| {
        s.number = 123; 
        Vec::new()
    })));
    a.literal("a");

    let mut b: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: Vec<i32>, _: &str, s: &mut Shared| {
        s.number = 456777; 
        Vec::new() 
    })));
    b.literal("b");
    
    let mut c: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: Vec<i32>, _: &str, s: &mut Shared| { 
        s.number = -999;
        Vec::new()
    })));
    c.literal("c");

    let mut failed: Rule<i32, Shared> = Rule::new(Some(Box::new(|_: Vec<i32>, _: &str, s: &mut Shared| { 
        s.number = 123456;
        Vec::new()
    })));
    failed.literal("---");

    if let Ok(_) = a.scan("a", &mut shared) {
        assert_eq!(shared.number, 123);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = b.scan("b", &mut shared) {
        assert_eq!(shared.number, 456777);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = c.scan("c", &mut shared) {
        assert_eq!(shared.number, -999);
    }
    else {
        assert!(false);
    }

    if let Ok(_) = failed.scan("xxx", &mut shared) {
        assert!(false);
    }
    else {
        assert_eq!(shared.number, -999);
    }
}
