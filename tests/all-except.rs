extern crate rule;
use rule::Rule;

#[test]
fn all_except()
{
    let mut dummy = false; 
    let code = "abc";
    
    let f = |_: Vec<u32>, l: &str, _: &mut bool| {
        assert_eq!(l, "abc");
        vec![0u32, 1u32, 2u32, 3u32]
    };
    
    let mut c = Rule::new(None);
    c.all_except(vec!['A', 'B', 'C', 'D']);
    
    let mut r: Rule<u32, bool> = Rule::new(Some(Box::new(f)));
    r.exact(3, c);
    
    if let Ok(branches) = r.scan(&code, &mut dummy) {
        assert_eq!(branches[0], 0u32);
        assert_eq!(branches[1], 1u32);
        assert_eq!(branches[2], 2u32);
        assert_eq!(branches[3], 3u32);
    }
    else {
        assert!(false);
    }
}