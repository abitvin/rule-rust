extern crate rule;
use rule::Rule;

#[test]
fn maybe() {
    let codes = vec![
        "xxx",
        "...xxx",
        "xxx...",
        "...xxx...",
    ];
    
    let dots = Rule::new(None);
    dots.literal("...");
            
    let xxx = Rule::new(Some(Box::new(|_, _| vec!['x'] )));
    xxx.literal("xxx");
            
    let root: Rule<char> = Rule::new(None);
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