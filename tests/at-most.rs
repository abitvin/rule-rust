extern crate rule;
use rule::Rule;

#[test]
fn at_most()
{
    let mut dummy = 1234;
    let code = "yyy";
    
    let mut y = Rule::new(Some(Box::new(|_, _, _| vec![14] )));
    y.literal("y");
            
    let mut root: Rule<i32, i32> = Rule::new(None);
    
    unsafe {
        if let Ok(_) = root.at_most_raw(2, &y).scan(&code, &mut dummy) {
            assert!(false);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().at_most_raw(3, &y).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 14);
            assert_eq!(branches[1], 14);
            assert_eq!(branches[2], 14);
        }
        else {
            assert!(false);
        }
        
        if let Ok(branches) = root.clear().at_most_raw(4, &y).scan(&code, &mut dummy) {
            assert_eq!(branches[0], 14);
            assert_eq!(branches[1], 14);
            assert_eq!(branches[2], 14);
        }
        else {
            assert!(false);
        }
    }
}