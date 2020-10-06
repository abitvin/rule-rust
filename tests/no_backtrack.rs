use rule::Rule;

#[test]
fn no_backtrack_1() {
    let root: Rule<bool> = Rule::new(&|_, _| false);
    root.literal("東東").no_backtrack("Oops!".to_string()).literal("💝💝💝");
    
    if let Err(err) = root.scan("東") {
        assert_eq!(format!("{}", err), "Error found at line 1, column: 0: Syntax error.".to_string());
    }
    else {
        assert!(false);
    }
    
    if let Err(err) = root.scan("東東") {
        assert_eq!(format!("{}", err), "Error found at line 1, column: 2: Oops!".to_string());
    }
    else {
        assert!(false);
    }

    if let Err(err) = root.scan("東東a") {
        assert_eq!(format!("{}", err), "Error found at line 1, column: 2: Oops!".to_string());
    }
    else {
        assert!(false);
    }
    
    if let Err(err) = root.scan("東東ab") {
        assert_eq!(format!("{}", err), "Error found at line 1, column: 2: Oops!".to_string());
    }
    else {
        assert!(false);
    }
    
    if let Err(_) = root.scan("東東💝💝💝") {
        assert!(false);
    }
    else {
        assert!(true);
    }
    
    if let Err(err) = root.scan("東東💝💝💝banana") {
        assert_eq!(format!("{}", err), "Error found at line 1, column: 5: Syntax error.".to_string());
    }
    else {
        assert!(false);
    }
}

#[test]
fn no_backtrack_2() {
    let hart: Rule<bool> = Rule::default();
    hart.literal("東東💝💝💝\n");
    
    let root: Rule<bool> = Rule::default();
    root.none_or_many(&hart);

    if let Err(_) = root.scan("東東💝💝💝\n") {
        assert!(false);
    }
    else {
        assert!(true);
    }
    
    if let Err(err) = root.scan("東東💝💝💝\n東") {
        assert_eq!(format!("{}", err), "Error found at line 2, column: 0: Syntax error.".to_string());
    }
    else {
        assert!(false);
    }
    
    if let Err(err) = root.scan("東東💝💝💝\n東東💝💝💝\n東東💝💝💝") {
        assert_eq!(format!("{}", err), "Error found at line 3, column: 0: Syntax error.".to_string());
    }
    else {
        assert!(false);
    }

    if let Err(err) = root.scan("東東💝💝💝\n東東💝💝💝\n東東💝💝💝\n東") {
        assert_eq!(format!("{}", err), "Error found at line 4, column: 0: Syntax error.".to_string());
    }
    else {
        assert!(false);
    }
}

#[test]
fn no_backtrack_3() {
    let line: Rule<bool> = Rule::default();
    line.literal("東東").no_backtrack("Yikes!".to_string()).literal("💝💝💝\n");
    
    let root: Rule<bool> = Rule::default();
    root.none_or_many(&line);

    if let Err(_) = root.scan("東東💝💝💝\n") {
        assert!(false);
    }
    else {
        assert!(true);
    }
    
    if let Err(err) = root.scan("東東💝💝💝\n東") {
        assert_eq!(format!("{}", err), "Error found at line 2, column: 0: Syntax error.".to_string());
    }
    else {
        assert!(false);
    }
    
    if let Err(err) = root.scan("東東💝💝💝\n東東💝💝💝\n東東💝💝💝") {
        assert_eq!(format!("{}", err), "Error found at line 3, column: 2: Yikes!".to_string());
    }
    else {
        assert!(false);
    }

    if let Err(err) = root.scan("東東💝💝💝\n東東💝💝💝\n東東💝💝💝\n東") {
        assert_eq!(format!("{}", err), "Error found at line 4, column: 0: Syntax error.".to_string());
    }
    else {
        assert!(false);
    }
}