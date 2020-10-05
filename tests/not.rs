use rule::Rule;

#[test]
fn not() {
    let not_this = Rule::default();
    not_this.literal("not this");
    
    let r: Rule<i32> = Rule::default();
    r.literal("aaa").not(&not_this).literal("bbb").literal("ccc");
    
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