/* TODO Update this test.
use rule::Rule;

#[test]
fn err_msgs() {
    let msg_banana = "Expected text \"banana\".";
    let msg_monkey = "Expected text \"monkey\".";
    let msg_abc = "Expected character 'a', 'b' or 'c'.";
    let msg_apple = "Expected text \"apple\".";

    let banana: Rule<()> = Rule::new(None);
    banana.literal("banana").no_backtrack("");
    
    let a: Rule<()> = Rule::new(None);
    a.literal("a").one(&banana);

    let b: Rule<()> = Rule::new(None);
    b.literal("b");

    let c: Rule<()> = Rule::new(None);
    c.literal("c");
    /*
    let abc: Rule<()> = Rule::new_with_err_msg(None, msg_abc);
    abc.any_of(vec![&c, &b, &a]);

    let monkeyabc: Rule<()> = Rule::new_with_err_msg(None, msg_monkey);
    monkeyabc.literal("monkey").one(&abc);

    let applemonkeyabc: Rule<()> = Rule::new_with_err_msg(None, msg_apple);
    applemonkeyabc.literal("apple").one(&monkeyabc);

    if let Err(err) = applemonkeyabc.scan("") {
        assert_eq!(err.msg, msg_apple);
    }
    else {
        assert!(false);
    }

    if let Err(err) = applemonkeyabc.scan("a") {
        assert_eq!(err.msg, msg_apple);
    }
    else {
        assert!(false);
    }

    if let Err(err) = applemonkeyabc.scan("apple") {
        assert_eq!(err.msg, msg_monkey);
    }
    else {
        assert!(false);
    }

    if let Err(err) = applemonkeyabc.scan("applem") {
        assert_eq!(err.msg, msg_monkey);
    }
    else {
        assert!(false);
    }

    if let Err(err) = applemonkeyabc.scan("applemonkeyd") {
        assert_eq!(err.msg, msg_abc);
    }
    else {
        assert!(false);
    }

    if let Err(err) = applemonkeyabc.scan("applemonkeya") {
        assert_eq!(err.msg, msg_banana);
    }
    else {
        assert!(false);
    }
    */
}

#[test]
fn backtrack_err_msgs() {
    let a: Rule<()> = Rule::new(None);
    a.literal("a");

    let b: Rule<()> = Rule::new(None);
    b.literal("b");

    let c: Rule<()> = Rule::new(None);
    c.literal("c");

    let abc_or_block: Rule<()> = Rule::new(None);
    let block: Rule<()> = Rule::new(None);

    abc_or_block.any_of(vec![&c, &b, &a, &block]);
    block.literal("[").no_backtrack("Expected a closing bracket.").at_least(1, &abc_or_block).literal("]");

    let root: Rule<()> = Rule::new(None);
    root.at_least(1, &abc_or_block);

    if let Err(err) = root.scan("abcabc[abcabc[aa]aa]aaa") {
        println!("{}", err);
        assert!(false);
    }
    else {
        assert!(false);
    }

    /*if let Ok(_) = root.scan("abcabc[abcabc]") {
        assert!(true);
    }
    else {
        assert!(false);
    }*/
}
*/