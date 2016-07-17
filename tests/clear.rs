extern crate rule;
use rule::Rule;

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn clear()
{
    let mut dummy = 'X';
    let code = "Ello'";
    
    let mut r: Rule<char, char> = Rule::new(None);
    r.literal("Ello'");
    r.clear();
    r.scan(&code, &mut dummy);   // Panic! We cleared the rule.
}