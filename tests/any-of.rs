extern crate rule;
use rule::Rule;

#[test]
fn any_of()
{
    let code = "aaabbbccc";
    
    let aaa_fn = |_: Vec<i32>, l: &str| {
        assert_eq!(l, "aaa");
        vec![111]
    }; 
    
    let bbb_fn = |_: Vec<i32>, l: &str| {
        assert_eq!(l, "bbb");
        vec![222]
    };
    
    let ccc_fn = |_: Vec<i32>, l: &str| {
        assert_eq!(l, "ccc");
        vec![333]
    };
    
    let mut aaa = Rule::new(Some(Box::new(aaa_fn)));
    aaa.literal("aaa");
    
    let mut bbb = Rule::new(Some(Box::new(bbb_fn)));
    bbb.literal("bbb");
    
    let mut ccc = Rule::new(Some(Box::new(ccc_fn)));
    ccc.literal("ccc");
    
    let mut any_of_these = Rule::new(None);
    any_of_these.any_of(vec![aaa, bbb, ccc]);
    
    let mut root: Rule<i32> = Rule::new(None);
    root.exact(3, any_of_these);
    
    if let Ok(branches) = root.scan(&code) {
        assert_eq!(branches[0], 111);
        assert_eq!(branches[1], 222);
        assert_eq!(branches[2], 333);
    }
    else {
        assert!(false);
    }
}