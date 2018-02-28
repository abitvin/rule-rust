extern crate rule;
use rule::Rule;

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn clear()
{
    let code = "Ello'";
    
    let mut r: Rule<char> = Rule::new(None);
    r.literal("Ello'");
    r.clear();
    r.scan(&code);   // Panic! We cleared the rule.
}