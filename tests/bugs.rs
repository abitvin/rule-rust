#![feature(nll)]

/*

    The following bug was in Rule v0.5.12.
    This bug is fixed in v0.6 but is incompatible with past releases.
    
    For easier reading we're using the Grammer API.

    We add two new rules:
    grammer.add("block", "(<stmt>( <stmt>)*)?", Some(Box::new(block)));
    grammer.add("root", " <block> ", Some(Box::new(entry)));

    The first rule is surrounded by a "maybe" which was causing invalid results. When given the scanner
    an empty string the "root" rule was returning 0 branches which isn't correct because we have
    given it a branch function. The problem was that the branch function wasn't handling the empty result
    but it should have.

*/

extern crate rule;
use rule::Rule;

#[test]
fn bug_0_5_12_test_empty_string() {
    let block_fn = |b: Vec<u32>, _: &str| {
        assert_eq!(b.len(), 0);
        vec![1]
    };

    let root_fn = |b: Vec<u32>, _: &str| {
        assert_eq!(b.len(), 1);
        b
    };

    let stmt_fn = |b: Vec<u32>, _: &str| {
        assert_eq!(b.len(), 0);
        vec![7]
    };

    let ws = Rule::new(None);
    ws.literal(" ");

    let none_or_many_ws = Rule::new(None);
    none_or_many_ws.none_or_many(&ws);

    let stmt = Rule::new(Some(Box::new(stmt_fn)));
    stmt.literal("stmt");

    let ws_plus_stmt = Rule::new(None);
    ws_plus_stmt.one(&none_or_many_ws).one(&stmt);
    
    let stmts = Rule::new(None);
    stmts.one(&stmt).none_or_many(&ws_plus_stmt);
    
    let block = Rule::new(Some(Box::new(block_fn)));
    block.maybe(&stmts);
    
    let root = Rule::new(Some(Box::new(root_fn)));
    root.one(&none_or_many_ws).one(&block).one(&none_or_many_ws);

    let code = "";

    if let Ok(branches) = root.scan(code) {
        assert!(branches.len() == 1);
    }
    else {
        assert!(false);
    }
}

#[test]
fn bug_0_5_12_test_with_content() {
    let block_fn = |b: Vec<u32>, _: &str| {
        assert_eq!(b.len(), 3);
        vec![1]
    };

    let root_fn = |b: Vec<u32>, _: &str| {
        assert_eq!(b.len(), 1);
        b
    };

    let stmt_fn = |b: Vec<u32>, _: &str| {
        assert_eq!(b.len(), 0);
        vec![7]
    };

    let ws = Rule::new(None);
    ws.literal(" ");

    let none_or_many_ws = Rule::new(None);
    none_or_many_ws.none_or_many(&ws);

    let stmt = Rule::new(Some(Box::new(stmt_fn)));
    stmt.literal("stmt");

    let ws_plus_stmt = Rule::new(None);
    ws_plus_stmt.one(&none_or_many_ws).one(&stmt);
    
    let stmts = Rule::new(None);
    stmts.one(&stmt).none_or_many(&ws_plus_stmt);
    
    let block = Rule::new(Some(Box::new(block_fn)));
    block.maybe(&stmts);
    
    let root = Rule::new(Some(Box::new(root_fn)));
    root.one(&none_or_many_ws).one(&block).one(&none_or_many_ws);

    let code = "  stmt    stmt   stmt  ";

    if let Ok(branches) = root.scan(code) {
        assert!(branches.len() == 1);
    }
    else {
        assert!(false);
    }
}