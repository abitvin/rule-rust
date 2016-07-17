extern crate rule;
use rule::Rule;

#[test]
fn maybe()
{
    let mut dummy = false;

    let codes = vec![
        "xxx",
        "...xxx",
        "xxx...",
        "...xxx...",
    ];
    
    let mut dots = Rule::new(None);
    dots.literal("...");
            
    let mut xxx = Rule::new(Some(Box::new(|_, _, _| vec!['x'] )));
    xxx.literal("xxx");
            
    let mut root: Rule<char, bool> = Rule::new(None);
    unsafe { root.maybe_raw(&dots).one(xxx).maybe_raw(&dots); }
    
    for c in codes {
        if let Ok(branches) = root.scan(&c, &mut dummy) {
            assert!(branches.len() == 1 && branches[0] == 'x');
        }
        else {
            assert!(false);
        }
    }
}