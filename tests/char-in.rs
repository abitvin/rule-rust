extern crate rule;
use rule::Rule;

#[test]
fn char_in()
{
    let mut dummy = 1234;

    let mut digit = Rule::new(Some(Box::new(|_, l, _| vec![(l.chars().next().unwrap() as u32) - 48])));
    digit.char_in('0', '9');
    
    let mut af = Rule::new(Some(Box::new(|_, l, _| vec![(l.chars().next().unwrap() as u32) - 55])));
    af.char_in('A', 'F');

    let mut hex = Rule::new(None);
    hex.any_of(vec![digit, af]);

    let mut parser: Rule<u32, i32> = Rule::new(Some(Box::new(|b, _, _| 
    {
        let mut m = 1u32;
        let mut n = 0u32;
        
        for i in b.iter().rev() {
            n += i * m;
            m <<= 4;
        }
        
        vec![n]
    })));
    parser.between(1, 8, hex);
    
    if let Ok(branches) = parser.scan("A", &mut dummy) {
        assert_eq!(branches[0], 10);
    }
    else {
        assert!(false);
    }
    
    if let Ok(branches) = parser.scan("12345678", &mut dummy) {
        assert_eq!(branches[0], 305419896);
    }
    else {
        assert!(false);
    }
    
    if let Ok(branches) = parser.scan("FF", &mut dummy) {
        assert_eq!(branches[0], 255);
    }
    else {
        assert!(false);
    }
    
    if let Ok(branches) = parser.scan("FFFFFFFF", &mut dummy) {
        assert_eq!(branches[0], u32::max_value());
    }
    else {
        assert!(false);
    }
    
    if let Ok(_) = parser.scan("FFFFFFFFF", &mut dummy) {
        assert!(false);
    }
    else {
        assert!(true);
    }
    
    if let Ok(_) = parser.scan("FFxFF", &mut dummy) {
        assert!(false);
    }
    else {
        assert!(true);
    }
    
    if let Ok(_) = parser.scan("", &mut dummy) {
        assert!(false);
    }
    else {
        assert!(true);
    }
}