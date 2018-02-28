extern crate rule;
use rule::Rule;

#[test]
fn not()
{
    let mut not_this = Rule::new(None);
    not_this.literal("not this");
    
    let mut r: Rule<i32> = Rule::new(None);
    r.literal("aaa").not(not_this).literal("bbb").literal("ccc");
    
    if let Ok(_) = r.scan("aaabbbccc") {
        assert!(true);
    }
    else {
        assert!(false);
    }
    
    if let Ok(_) = r.scan("aaanot thisbbbccc") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
