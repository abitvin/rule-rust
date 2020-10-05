use rule::Rule;

#[test]
fn maybe() {
    let codes = vec![
        "xxx",
        "...xxx",
        "xxx...",
        "...xxx...",
    ];
    
    let dots = Rule::default();
    dots.literal("...");
            
    let xxx = Rule::new(Box::new(|_, _| 'x'));
    xxx.literal("xxx");
            
    let root: Rule<char> = Rule::default();
    root.maybe(&dots).one(&xxx).maybe(&dots);
    
    for c in codes {
        if let Ok(branches) = root.scan(&c) {
            assert!(branches.len() == 1 && branches[0] == 'x');
        }
        else {
            assert!(false);
        }
    }
}