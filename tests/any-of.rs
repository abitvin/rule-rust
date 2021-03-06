use rule::Rule;

#[test]
fn any_of() {
    let code = "aaabbbccc";
    
    let aaa_fn = |_, l: &str| {
        assert_eq!(l, "aaa");
        Ok(111)
    }; 
    
    let bbb_fn = |_, l: &str| {
        assert_eq!(l, "bbb");
        Ok(222)
    };
    
    let ccc_fn = |_, l: &str| {
        assert_eq!(l, "ccc");
        Ok(333)
    };
    
    let aaa = Rule::new(aaa_fn);
    aaa.literal("aaa");
    
    let bbb = Rule::new(bbb_fn);
    bbb.literal("bbb");
    
    let ccc = Rule::new(ccc_fn);
    ccc.literal("ccc");
    
    let any_of_these = Rule::default();
    any_of_these.any_of(vec![&aaa, &bbb, &ccc]);
    
    let root: Rule<i32> = Rule::default();
    root.exact(3, &any_of_these);
    
    if let Ok(branches) = root.scan(&code) {
        assert_eq!(branches[0], 111);
        assert_eq!(branches[1], 222);
        assert_eq!(branches[2], 333);
    }
    else {
        assert!(false);
    }
}