extern crate rule;
use rule::Rule;

#[test]
fn at_least()
{
    let mut dummy = false;
    let code = "xxxx";
    
    let mut x = Rule::new(Some(Box::new(|_, _, _| vec![10])));
    x.literal("x");
    
    let mut root: Rule<i32, bool> = Rule::new(None);
    
    unsafe {
        if let Ok(branches) = root.at_least_raw(3, &x).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 10);
            assert_eq!(branches[1], 10);
            assert_eq!(branches[2], 10);
            assert_eq!(branches[3], 10);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().at_least_raw(4, &x).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 10);
            assert_eq!(branches[1], 10);
            assert_eq!(branches[2], 10);
            assert_eq!(branches[3], 10);
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = root.clear().at_least_raw(5, &x).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
}