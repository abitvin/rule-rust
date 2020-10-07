use rule::Rule;

#[test]
fn char_in() {
    let digit = Rule::new(|_, l| (l.chars().next().unwrap() as u32) - 48);
    digit.char_in('0', '9');
    
    let af = Rule::new(|_, l| (l.chars().next().unwrap() as u32) - 55);
    af.char_in('A', 'F');

    let hex = Rule::default();
    hex.any_of(vec![&digit, &af]);

    let parser: Rule<u32> = Rule::new(|b, _| {
        let mut m = 1u32;
        let mut n = 0u32;
        
        for i in b.iter().rev() {
            n += i * m;
            m <<= 4;
        }
        
        n
    });

    parser.between(1, 8, &hex);
    
    if let Ok(branches) = parser.scan("A") {
        assert_eq!(branches[0], 10);
    }
    else {
        assert!(false);
    }
    
    if let Ok(branches) = parser.scan("12345678") {
        assert_eq!(branches[0], 305419896);
    }
    else {
        assert!(false);
    }
    
    if let Ok(branches) = parser.scan("FF") {
        assert_eq!(branches[0], 255);
    }
    else {
        assert!(false);
    }
    
    if let Ok(branches) = parser.scan("FFFFFFFF") {
        assert_eq!(branches[0], u32::max_value());
    }
    else {
        assert!(false);
    }
    
    if let Ok(_) = parser.scan("FFFFFFFFF") {
        assert!(false);
    }
    else {
        assert!(true);
    }
    
    if let Ok(_) = parser.scan("FFxFF") {
        assert!(false);
    }
    else {
        assert!(true);
    }
    
    if let Ok(_) = parser.scan("") {
        assert!(false);
    }
    else {
        assert!(true);
    }
}