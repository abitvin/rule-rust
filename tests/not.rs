use rule::Rule;

#[test]
fn not_1() {
    let not_this = Rule::default();
    not_this.literal("not this");
    
    let r: Rule<i32> = Rule::default();
    r.literal("aaa").not(&not_this).literal("bbb").literal("ccc");
    
    assert!(r.scan("aaabbbccc").is_ok());
    assert!(r.scan("aaanot thisbbbccc").is_err());
}

#[test]
fn not_literal() {
    let monkey = Rule::default();
    monkey.literal("monkey");

    let no_monkey: Rule<i32> = Rule::default();
    no_monkey.not(&monkey);
    
    assert!(no_monkey.scan("").is_ok());
    assert!(no_monkey.scan("pizza").is_err());
    assert!(no_monkey.scan("monkey").is_err());
}

#[test]
fn not_one_literal() {
    let monkey = Rule::default();
    monkey.literal("monkey");

    let one_monkey = Rule::default();
    one_monkey.one(&monkey);

    let no_monkey: Rule<i32> = Rule::default();
    no_monkey.not(&one_monkey);
    
    assert!(no_monkey.scan("").is_ok());
    assert!(no_monkey.scan("pizza").is_err());
    assert!(no_monkey.scan("monkey").is_err());
}

#[test]
fn not_none_or_many() {
    let monkey = Rule::default();
    monkey.literal("monkey");

    let none_or_more_monkeys = Rule::default();
    none_or_more_monkeys.none_or_many(&monkey);
    
    let no_monkeys: Rule<i32> = Rule::default();
    no_monkeys.not(&none_or_more_monkeys);
    
    assert!(no_monkeys.scan("").is_err());
    assert!(no_monkeys.scan("pizza").is_err());
    assert!(no_monkeys.scan("monkey").is_err());
}

#[test]
fn not_at_least_1() {
    let monkey = Rule::default();
    monkey.literal("monkey");

    let at_least_1_monkey = Rule::default();
    at_least_1_monkey.at_least(1, &monkey);
    
    let no_monkeys: Rule<i32> = Rule::default();
    no_monkeys.not(&at_least_1_monkey);
    
    assert!(no_monkeys.scan("").is_ok());
    assert!(no_monkeys.scan("pizza").is_err());
    assert!(no_monkeys.scan("monkey").is_err());
}

#[test]
fn not_no_backtracks_should_be_ignored() {
    let monkey = Rule::default();
    monkey.literal("mon").no_backtrack("Ignore me.".to_string()).literal("key");

    let gorilla = Rule::default();
    gorilla.literal("gorilla");

    let no_monkey: Rule<i32> = Rule::default();
    no_monkey.not(&monkey).one(&gorilla);
    
    assert_eq!(no_monkey.scan("").map_err(|x| format!("{}", x)).unwrap_err(), "Error found at line 1, column: 0: Syntax error.".to_string());
    assert!(no_monkey.scan("gorilla").is_ok());
    assert_eq!(no_monkey.scan("monk").map_err(|x| format!("{}", x)).unwrap_err(), "Error found at line 1, column: 0: Syntax error.".to_string());
    assert_eq!(no_monkey.scan("monkeybananagorilla").map_err(|x| format!("{}", x)).unwrap_err(), "Error found at line 1, column: 0: Syntax error.".to_string());
}

#[test]
fn nested_not_1() {
    let monkey = Rule::default();
    monkey.literal("monkey");

    let no_monkey: Rule<i32> = Rule::default();
    no_monkey.not(&monkey);

    let no_no_monkey: Rule<i32> = Rule::default();
    no_no_monkey.not(&no_monkey);

    assert!(no_no_monkey.scan("").is_err());
    assert!(no_no_monkey.scan("monkey").is_err());

    let no_no_monkey_then_monkey: Rule<i32> = Rule::default();
    no_no_monkey_then_monkey.one(&no_no_monkey).one(&monkey);

    assert!(no_no_monkey_then_monkey.scan("monkey").is_ok());
}

#[test]
fn nested_not_2() {
    let banana = Rule::default();
    banana.literal("banana");

    let monkey = Rule::default();
    monkey.literal("monkey");

    let banana_not_monkey = Rule::default();
    banana_not_monkey.one(&banana).not(&monkey);

    let not_banana_not_monkey: Rule<i32> = Rule::default();
    not_banana_not_monkey.not(&banana_not_monkey);

    assert!(not_banana_not_monkey.scan("").is_ok());
    assert!(not_banana_not_monkey.scan("banana").is_err());
    assert!(not_banana_not_monkey.scan("bananamonkey").is_err());

    let not_banana_not_monkey_then_bananamonkey: Rule<i32> = Rule::default();
    not_banana_not_monkey_then_bananamonkey.one(&not_banana_not_monkey).one(&banana).one(&monkey);

    assert!(not_banana_not_monkey_then_bananamonkey.scan("bananamonkey").is_ok());
}