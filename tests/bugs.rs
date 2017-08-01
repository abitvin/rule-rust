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
fn bug_0_5_12_test_empty_string()
{
    let block_fn = |b: Vec<u32>, _: &str, _: &mut bool| {
        assert_eq!(b.len(), 0);
        vec![1]
    };

    let root_fn = |b: Vec<u32>, _: &str, _: &mut bool| {
        assert_eq!(b.len(), 1);
        b
    };

    let stmt_fn = |b: Vec<u32>, _: &str, _: &mut bool| {
        assert_eq!(b.len(), 0);
        vec![7]
    };

    let mut dummy = false;

    let mut ws = Rule::new(None);
    ws.literal(" ");

    let mut none_or_many_ws = Rule::new(None);
    none_or_many_ws.none_or_many(ws);

    let mut stmt = Rule::new(Some(Box::new(stmt_fn)));
    stmt.literal("stmt");

    let mut ws_plus_stmt = Rule::new(None);
    unsafe { ws_plus_stmt.one_raw(&none_or_many_ws).one_raw(&stmt); }
    
    let mut stmts = Rule::new(None);
    stmts.one(stmt).none_or_many(ws_plus_stmt);
    
    let mut block = Rule::new(Some(Box::new(block_fn)));
    block.maybe(stmts);
    
    let mut root = Rule::new(Some(Box::new(root_fn)));
    unsafe { root.one_raw(&none_or_many_ws).one(block).one_raw(&none_or_many_ws); }

    let code = "";

    if let Ok(branches) = root.scan(code, &mut dummy) {
        assert!(branches.len() == 1);
    }
    else {
        assert!(false);
    }
}

#[test]
fn bug_0_5_12_test_with_content()
{
    let block_fn = |b: Vec<u32>, _: &str, _: &mut bool| {
        assert_eq!(b.len(), 3);
        vec![1]
    };

    let root_fn = |b: Vec<u32>, _: &str, _: &mut bool| {
        assert_eq!(b.len(), 1);
        b
    };

    let stmt_fn = |b: Vec<u32>, _: &str, _: &mut bool| {
        assert_eq!(b.len(), 0);
        vec![7]
    };

    let mut dummy = false;

    let mut ws = Rule::new(None);
    ws.literal(" ");

    let mut none_or_many_ws = Rule::new(None);
    none_or_many_ws.none_or_many(ws);

    let mut stmt = Rule::new(Some(Box::new(stmt_fn)));
    stmt.literal("stmt");

    let mut ws_plus_stmt = Rule::new(None);
    unsafe { ws_plus_stmt.one_raw(&none_or_many_ws).one_raw(&stmt); }
    
    let mut stmts = Rule::new(None);
    stmts.one(stmt).none_or_many(ws_plus_stmt);
    
    let mut block = Rule::new(Some(Box::new(block_fn)));
    block.maybe(stmts);
    
    let mut root = Rule::new(Some(Box::new(root_fn)));
    unsafe { root.one_raw(&none_or_many_ws).one(block).one_raw(&none_or_many_ws); }

    let code = "  stmt    stmt   stmt  ";

    if let Ok(branches) = root.scan(code, &mut dummy) {
        assert!(branches.len() == 1);
    }
    else {
        assert!(false);
    }
}