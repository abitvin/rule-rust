extern crate rule;
use rule::Rule;

#[test]
fn eof()
{
    let mut dummy = '@';
    let code = "123";
    
    let mut r: Rule<char, char> = Rule::new(Some(Box::new(|_, _, _| vec!['A', 'B'] )));
    r.literal("123").eof();
    
    if let Ok(branches) = r.scan(&code, &mut dummy) {
        assert_eq!(branches[0], 'A');
        assert_eq!(branches[1], 'B');
    }
    else {
        assert!(false);
    }
}