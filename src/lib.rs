// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

// TODO Test start error.
// TODO Test succes
// TODO Make default constructor with identity (None) branch function.

use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::str::Chars;

pub type BranchFn<T> = Option<Box<dyn Fn(Vec<T>, &str) -> T>>;  // TODO Would be very nice to remove the Box here.

enum Progress<'s, T> {
    Some { steps: usize, ctx: ScanCtx<'s, T> },
    No(ScanCtx<'s, T>),
    Error { idx: usize, msg: String },
}

pub struct Rule<T>(Rc<RefCell<_Rule<T>>>);

impl<T> Clone for Rule<T> {
    fn clone(&self) -> Self {
        Self { 0: self.0.clone() }
    }
}

struct _Rule<T> {
    branch_fn: BranchFn<T>,
    instr: Vec<Instr<T>>,
}

#[derive(Debug)]
pub struct RuleError {
    pub col: usize,
    pub line: usize,
    pub msg: String,
}

impl fmt::Display for RuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error found at line {}, column: {}: {}", self.line, self.col, self.msg)
    }
}

impl Error for RuleError {
    fn description(&self) -> &str {
        "Rule error"
    }
}

impl RuleError {
    fn new(text: &str, index: usize, msg: String) -> Self {
        let char_count = text.char_indices().count();
        
        let skip = if char_count == index && index > 0 {
            index - 1
        }
        else {
            index
        };

        let chr_idx = text.char_indices()
            .skip(skip)
            .next()
            .map(|x| x.0)
            .unwrap_or(0);

        let pos = cursor_pos(&text[..chr_idx]);
        
        Self { 
            col: pos.col,
            line: pos.line,
            msg, 
        }
    }
}

enum Instr<T> {
    AnyChar,
    AnyCharExcept(Vec<char>),
    Alter(Vec<(&'static str, &'static str)>),
    AlterString(Vec<(String, String)>),
    AnyOf(Vec<Rule<T>>),
    CharIn(char, char),
    Eof,
    Literal(&'static str),
    LiteralString(String),
    NoBacktrack(String),
    Not(Rule<T>),
    Range(u64, u64, Rule<T>),
}

#[derive(Clone)]
struct ScanErr { 
    idx: usize, 
    msg: String,
}

struct ScanCtx<'s, T> {
    branches: Vec<T>,
    code_iter: Chars<'s>,
    index: usize,
    in_not: bool,   // True when we're scanning in a "not" rule. Which is a special state allowing backtracking even when a "no_backtracking" anchor has been set.
                    // TODO Seems logical but I tested it and it doesn't matter if `in_not` is true or false on that a large tested codebase. Which is weird. Do special tests on this matter.
    lexeme: String,
}

impl<'s, T> ScanCtx<'s, T> {
    fn new(code: &'s str) -> Self {
        Self {
            branches: Vec::new(),
            code_iter: code.chars(),
            index: 0,
            in_not: false,
            lexeme: String::new(),
        }
    }

    fn branch(self, in_not: bool) -> (ScanCtx<'s, T>, ScanCtx<'s, T>) {
        let new_ctx = ScanCtx {
            branches: Vec::new(),
            code_iter: self.code_iter.clone(),
            index: self.index,
            in_not: self.in_not || in_not,
            lexeme: String::new(),
        };

        (new_ctx, self)
    }

    fn merge_with(mut self, mut source: ScanCtx<'s, T>, is_rule: bool, branch_fn: &BranchFn<T>) -> Progress<'s, T> {
        let steps = source.index - self.index;
        
        self.code_iter = source.code_iter;
        self.index = source.index;
        self.lexeme.push_str(&source.lexeme.to_string());
        
        match branch_fn {
            Some(ref f) if is_rule => self.branches.push(f(source.branches, &source.lexeme)),
            _ => self.branches.append(&mut source.branches),
        }
        
        Progress::Some { steps, ctx: self }
    }
}

impl<T> Rule<T> {
    pub fn new(branch_fn: BranchFn<T>) -> Self {
        Rule { 
            0: Rc::new(RefCell::new(_Rule {
                branch_fn: branch_fn,
                instr: Vec::new(),
            }))
        }
    }

    pub fn any_char(&self) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::AnyChar);
        self
    }
    
    pub fn any_char_except(&self, exclude: Vec<char>) -> &Self {
        if exclude.len() == 0 {
            panic!("List of excluded characters is empty.");
        }
        
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::AnyCharExcept(exclude));
        self
    }
    
    pub fn alter(&self, list: Vec<(&'static str, &'static str)>) -> &Self {
        if list.len() == 0 {
            panic!("List is empty.");
        }
        
        if !list.iter().any(|t| { t.0.len() > 0 && t.1.len() > 0 }) {
            panic!("The strings in the list must be minimal one character long.");
        }
        
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Alter(list));
        self
    }

    pub fn alter_string(&self, list: Vec<(String, String)>) -> &Self {
        if list.len() == 0 {
            panic!("List is empty.");
        }
        
        if !list.iter().any(|ref t| { t.0.len() > 0 && t.1.len() > 0 }) {
            panic!("The strings in the list must be minimal one character long.");
        }
        
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::AlterString(list));
        self
    }
    
    pub fn any_of(&self, rules: Vec<&Rule<T>>) -> &Self {
        let mut r = self.0.borrow_mut();

        match rules.len() {
            0 => panic!("You must specify rules."),
            1 => r.instr.push(Instr::Range(1, 1, rules[0].clone())),
            _ => r.instr.push(Instr::AnyOf(rules.into_iter().map(|x| x.clone()).collect())),  
        };

        self
    }
    
    pub fn at_least(&self, count: u64, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Range(count, u64::max_value(), rule.clone()));
        self
    }
    
    pub fn at_most(&self, count: u64, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Range(0, count, rule.clone()));
        self
    }
    
    pub fn between(&self, min: u64, max: u64, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Range(min, max, rule.clone()));
        self
    }
    
    pub fn char_in(&self, min: char, max: char) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::CharIn(min, max));
        self
    }
    
    pub fn eof(&self) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Eof);
        self
    }
    
    pub fn exact(&self, count: u64, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Range(count, count, rule.clone()));
        self
    }
    
    pub fn literal(&self, text: &'static str) -> &Self {
        if text.len() < 1 {
            panic!("Literal text must at least 1 character long.");
        }

        let mut r = self.0.borrow_mut();   
        r.instr.push(Instr::Literal(&text));
        self
    }

    pub fn literal_string(&self, text: String) -> &Self {
        if text.len() < 1 {
            panic!("Literal text must at least 1 character long.");
        }
            
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::LiteralString(text));
        self
    }

    pub fn maybe(&self, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Range(0, 1, rule.clone()));
        self
    }

    pub fn no_backtrack(&self, err_msg: String) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::NoBacktrack(err_msg));
        self
    }

    pub fn none_or_many(&self, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Range(0, u64::max_value(), rule.clone()));
        self
    }
    
    pub fn not(&self, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Not(rule.clone()));
        self
    }
    
    pub fn one(&self, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.instr.push(Instr::Range(1, 1, rule.clone()));
        self
    }

    pub fn scan(&self, code: &str) -> Result<Vec<T>, RuleError> {
        let r = self.0.borrow();
            
        if r.instr.len() == 0 {
            panic!("Rule is not defined.");
        }
        
        let mut ctx = ScanCtx::new(code);
        let scanner = Scanner::new();

        match scanner.run(self, ctx) {
            Progress::Some { steps: _, ctx: new_ctx } => ctx = new_ctx,
            Progress::No(new_ctx) => return Err(RuleError::new(code, new_ctx.index, String::from("Scan error: Syntax error at the beginning."))),
            Progress::Error { idx, msg } => return Err(RuleError::new(code, idx, msg)),
        }
        
        if let Some(_) = ctx.code_iter.next() {
            /* TODO Really check this out!
            TODO Do we need these checks?
            if ctx.has_eof {
                ctx.index -= 1;
            }
            
            if (ctx.index !== ctx.code.length)
                return RuleResult.failed<TBranch, TMeta>(ctx.errors);
            
            */
            
            Err(RuleError::new(code, ctx.index, format!("Scan error: Successful scan stopped at {}.", ctx.index)))
        }
        else {
            Ok(ctx.branches)
        }
    }
}

struct Scanner { 
    err: RefCell<ScanErr>,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            err: RefCell::new(ScanErr { idx: 0, msg: String::from("Syntax error.") }),
        }
    }

    fn run<'s, T>(&self, rule: &Rule<T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let r = rule.0.borrow();
        let (mut new_ctx, ctx) = ctx.branch(false);
        
        for p in &r.instr {
            let progress = match *p {
                // Leaves
                Instr::AnyChar => self.scan_any_char_leaf(new_ctx),
                Instr::AnyCharExcept(ref exclude) => self.scan_any_char_except_leaf(&exclude, new_ctx),
                Instr::Alter(ref alter) => self.scan_alter_leaf(&alter, new_ctx),
                Instr::AlterString(ref alter) => self.scan_alter_string_leaf(&alter, new_ctx),
                Instr::CharIn(min, max) => self.scan_char_in_leaf(min, max, new_ctx),
                Instr::Eof => self.scan_eof_leaf(new_ctx),
                Instr::Literal(text) => self.scan_literal_leaf(&text, new_ctx),
                Instr::LiteralString(ref text) => self.scan_literal_leaf(&text, new_ctx),
                
                // Non leaves
                Instr::AnyOf(ref rules) => self.scan_any_of(rules, new_ctx),
                Instr::Not(ref r) => self.scan_not(r, new_ctx),
                Instr::Range(min, max, ref r) => self.scan_rule_range(min, max, r, new_ctx),
                
                // No backtrack
                Instr::NoBacktrack(ref err_msg) => {
                    let mut err = self.err.borrow_mut();
                    *err = ScanErr { idx: new_ctx.index, msg: err_msg.clone() };
                    Progress::Some { steps: 0, ctx: new_ctx }
                },
            };

            match progress {
                Progress::Some { steps: _, ctx: newer_ctx } => new_ctx = newer_ctx,
                Progress::No(_) => return self.negative_progress(ctx),
                Progress::Error { idx, msg } => return Progress::Error { idx, msg },
            }
        }
        
        ctx.merge_with(new_ctx, true, &r.branch_fn)
    }
    
    // TODO What about a char with more codepoints?
    fn scan_any_char_except_leaf<'s, T>(&self, exclude: &Vec<char>, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let n = ctx.code_iter.next();
        
        if let Some(c) = n {
            if exclude.contains(&c) {
                return Progress::No(ctx);
            }
            
            ctx.lexeme.push(c);
            ctx.index += 1;
            Progress::Some { steps: 1, ctx }
        } 
        else {
            Progress::No(ctx)
        }
    }
    
    fn scan_any_char_leaf<'s, T>(&self, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let n = ctx.code_iter.next();
                
        if let Some(c) = n {
            ctx.lexeme.push(c);
            ctx.index += 1;
            Progress::Some { steps: 1, ctx }
        } 
        else {
            Progress::No(ctx)
        }
    }
    
    fn scan_alter_leaf<'s, T>(&self, list: &Vec<(&'static str, &'static str)>, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        for alter in list {
            let find = alter.0;
            let steps = find.chars().count();
            let compare: String = ctx.code_iter.clone().take(steps).collect();

            if find == compare {
                ctx.code_iter.nth(steps - 1);
                ctx.lexeme.push_str(alter.1);
                ctx.index += steps;
                return Progress::Some { steps, ctx };
            }
        }

        Progress::No(ctx)
    }
    
    fn scan_alter_string_leaf<'s, T>(&self, list: &Vec<(String, String)>, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        for alter in list {
            let find = &alter.0;
            let steps = find.chars().count();
            let compare: String = ctx.code_iter.clone().take(steps).collect();

            if *find == compare {
                ctx.code_iter.nth(steps - 1);
                ctx.lexeme.push_str(&alter.1);
                ctx.index += steps;
                return Progress::Some { steps, ctx };
            }
        }

        Progress::No(ctx)
    }
    
    fn scan_any_of<'s, T>(&self, rules: &Vec<Rule<T>>, ctx: ScanCtx<'s, T>) -> Progress<'s,T> {
        let (mut new_ctx, ctx) = ctx.branch(false);
        
        for r in rules {
            match self.run(r, new_ctx) {
                Progress::Some { steps: _, ctx: new_ctx } => {
                    let r = r.0.borrow();
                    return ctx.merge_with(new_ctx, false, &r.branch_fn);
                },
                Progress::No(prev_new_ctx) => {
                    new_ctx = prev_new_ctx;
                },
                Progress::Error { idx, msg } => {
                    return Progress::Error { idx, msg };
                }
            }
        }

        self.negative_progress(ctx)
    }

    fn scan_char_in_leaf<'s, T>(&self, min: char, max: char, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let c = ctx.code_iter.next();

        match c {
            Some(c) => {
                if c < min || c > max {
                    Progress::No(ctx)
                }
                else {
                    ctx.lexeme.push(c);
                    ctx.index += 1;
                    Progress::Some { steps: 1, ctx }
                }
            },
            None => {
                Progress::No(ctx)
            }
        }
    }
    
    fn scan_eof_leaf<'s, T>(&self, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        if let None = ctx.code_iter.next() {
            ctx.index += 1;
            Progress::Some { steps: 1, ctx }
        }
        else {
            Progress::No(ctx)
        }
    }
    
    fn scan_literal_leaf<'s, T>(&self, find: &str, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let iter = find.chars();
        let mut steps = 0;
            
        for i in iter {
            let n = ctx.code_iter.next();
                
            if let Some(c) = n {
                if i != c {
                    return Progress::No(ctx);
                }
                    
                ctx.index += 1;
                steps += 1;
            }
            else {
                return Progress::No(ctx);
            }
        }
        
        ctx.lexeme.push_str(find);
        Progress::Some { steps, ctx }
    }
    
    fn scan_not<'s, T>(&self, rule: &Rule<T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let (new_ctx, ctx) = ctx.branch(true);

        match self.run(rule, new_ctx) {
            Progress::Some { steps: _, ctx: _ } => Progress::No(ctx),
            Progress::No(_) => Progress::Some { steps: 0, ctx },
            Progress::Error { idx: _, msg: _ } => unreachable!(),
        }
    }
    
    fn scan_rule_range<'s, T>(&self, min: u64, max: u64, rule: &Rule<T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let (mut new_ctx, ctx) = ctx.branch(false);
        let mut count = 0u64;
        
        loop {
            match self.run(rule, new_ctx) {
                Progress::Some { steps, ctx: newer_ctx } => {
                    if steps == 0 {
                        let r = rule.0.borrow();
                        return ctx.merge_with(newer_ctx, false, &r.branch_fn);
                    }

                    new_ctx = newer_ctx;
                    count += 1;

                    if count == max {
                        break;
                    }
                },
                Progress::No(prev_new_ctx) => {
                    new_ctx = prev_new_ctx;
                    break;
                },
                Progress::Error { idx, msg } => {
                    return Progress::Error { idx, msg };
                }
            }
        }
        
        if count >= min && count <= max {
            let r = rule.0.borrow();
            ctx.merge_with(new_ctx, false, &r.branch_fn)
        }
        else {
            self.negative_progress(ctx)
        }
    }

    fn negative_progress<'s, T>(&self, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        if ctx.in_not {
            Progress::No(ctx)
        }
        else {
            let err = self.err.borrow();

            if ctx.index < err.idx {
                Progress::Error { idx: err.idx, msg: err.msg.clone() }
            }
            else {
                Progress::No(ctx)
            }
        }
    }
}

struct CursorPos {
    col: usize,
    line: usize,
}

fn cursor_pos(text: &str) -> CursorPos {
    let old_osx: Rule<usize> = Rule::new(None);
    old_osx.literal("\r");  // CR

    let unix: Rule<usize> = Rule::new(None);
    unix.literal("\n");     // LF

    let win: Rule<usize> = Rule::new(None);
    win.literal("\r\n");    // CR+LF

    let new_line: Rule<usize> = Rule::new(None);
    new_line.any_of(vec![&win, &old_osx, &unix]);

    let ch: Rule<usize> = Rule::new(None);
    ch.any_char_except(vec!['\r', '\n']);

    let new_line_only: Rule<usize> = Rule::new(Some(Box::new(|_, _| 0)));
    new_line_only.one(&new_line);

    let text_and_new_line: Rule<usize> = Rule::new(Some(Box::new(|_, _| 0)));
    text_and_new_line.at_least(1, &ch).one(&new_line);

    let text_only: Rule<usize> = Rule::new(Some(Box::new(|_, l| l.len())));
    text_only.at_least(1, &ch);

    let line: Rule<usize> = Rule::new(None);
    line.any_of(vec![&new_line_only, &text_and_new_line, &text_only]);

    let line_counter: Rule<usize> = Rule::new(None);
    line_counter.none_or_many(&line);

    if let Ok(lines) = line_counter.scan(text) {
        if lines.len() == 0 {
            // The scanned `text` is an empty string.
            CursorPos { col: 0, line: 1 }
        }
        else if lines[lines.len() - 1] == 0 {
            // Only the rules `text_and_new_line` where found.
            CursorPos { col: 0, line: lines.len() + 1 }
        }
        else {
            // The last line was `text_only`.
            CursorPos { col: lines[lines.len() - 1], line: lines.len() }
        }
    }
    else {
        unreachable!()
    }
}