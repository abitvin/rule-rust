extern crate rule;
use rule::Rule;

#[test]
fn exact()
{
    let mut dummy = false;
    let code = "..........";
    
    let mut dot = Rule::new(Some(Box::new(|_, _, _| vec!['.'] )));
    dot.literal(".");
            
    let mut nope = Rule::new(Some(Box::new(|_, _, _| vec!['x'] )));
    nope.literal("nope");
            
    let mut root: Rule<char, bool> = Rule::new(None);
    
    unsafe {
        if let Ok(branches) = root.exact_raw(10, &dot).scan(&code, &mut dummy) {
            assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = root.clear().exact_raw(9, &dot).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(_) = root.clear().exact_raw(11, &dot).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().exact_raw(0, &nope).exact_raw(10, &dot).exact_raw(0, &nope).scan(&code, &mut dummy) {
            assert!(branches.len() == 10 && branches.into_iter().any(|c| c == '.'));
        }
        else {
            assert!(false);
        }
    }
}