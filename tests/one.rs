extern crate rule;
use rule::Rule;

struct NoShared {}

#[test]
fn one()
{
    let mut dummy = NoShared {};
    let code = "onetwothree";
    
    let mut one: Rule<i32, NoShared> = Rule::new(Some(Box::new(|_, _, _| vec![1] )));
    one.literal("one");
    
    let mut two: Rule<i32, NoShared> = Rule::new(Some(Box::new(|_, _, _| vec![2] )));
    two.literal("two");
    
    let mut three: Rule<i32, NoShared> = Rule::new(Some(Box::new(|_, _, _| vec![3] )));
    three.literal("three");
    
    let mut root: Rule<i32, NoShared> = Rule::new(None);
    root.one(one).one(two).one(three);
    
    if let Ok(branches) = root.scan(&code, &mut dummy) {
        assert_eq!(branches[0], 1);
        assert_eq!(branches[1], 2);
        assert_eq!(branches[2], 3);
    }
    else {
        assert!(false);
    }
}