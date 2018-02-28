extern crate rule;
use rule::Rule;

#[test]
fn between()
{
    let code = "zzz";
    
    let mut z = Rule::new(Some(Box::new(|_, _| vec![34])));
    z.literal("z");
            
    let mut root: Rule<i32> = Rule::new(None);
    
    unsafe {
        if let Ok(branches) = root.between_raw(1, 3, &z).scan(&code) {
            assert_eq!(branches[0], 34);
            assert_eq!(branches[1], 34);
            assert_eq!(branches[2], 34);
        }
        else {
            assert!(true);
        }
        
        if let Ok(branches) = root.clear().between_raw(0, 10, &z).scan(&code) {
            assert_eq!(branches[0], 34);
            assert_eq!(branches[1], 34);
            assert_eq!(branches[2], 34);
        }
        else {
            assert!(false);
        }
        
        if let Ok(_) = root.clear().between_raw(4, 5, &z).scan(&code) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }
}