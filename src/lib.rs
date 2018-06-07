// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::str::Chars;

pub type BranchFn<T> = Option<Box<Fn(Vec<T>, &str) -> T>>;

enum Progress<'s, T> {
    Some { steps: usize, ctx: ScanCtx<'s, T> },
    No(ScanCtx<'s, T>),
}

pub struct Rule<T>(Rc<RefCell<_Rule<T>>>);

impl<T> Clone for Rule<T> {
    fn clone(&self) -> Self {
        Self { 0: self.0.clone() }
    }
}

struct _Rule<T> {
    branch_fn: BranchFn<T>,
    err_msg: Option<Rc<String>>,
    parts: Vec<ScanFn<T>>,
}

#[derive(Debug)]
pub struct RuleError {
    pub col: usize,
    pub index: usize,
    pub line: usize,
    pub msg: String,
}

impl fmt::Display for RuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error at line {}, column: {}, index: {}: {}", self.line, self.col, self.index, self.msg)
    }
}

impl Error for RuleError {
    fn description(&self) -> &str {
        "Rule error"
    }
}

impl RuleError {
    fn new<T>(text: &str, ctx: ScanCtx<T>) -> Self {
        let msg = ctx.err_msg
            .map(|x| x.as_ref().clone())
            .unwrap_or(String::from("General parse error."));

        let pos = cursor_pos(&text[..ctx.err_idx]);

        Self { 
            col: pos.col,
            index: ctx.err_idx,
            line: pos.line,
            msg, 
        }
    }
}

enum ScanFn<T> {
    AnyChar,
    AnyCharExcept(Vec<char>),
    Alter(Vec<(&'static str, &'static str)>),
    AlterString(Vec<(String, String)>),
    AnyOf(Vec<Rule<T>>),
    CharIn(char, char),
    Eof,
    Literal(&'static str),
    LiteralString(String),
    Not(Rule<T>),
    Range(u64, u64, Rule<T>),
}

struct ScanCtx<'s, T> {
    branches: Vec<T>,
    code_iter: Chars<'s>,
    err_idx: usize,
    err_msg: Option<Rc<String>>,
    index: usize,
    lexeme: String,
}

impl<'s, T> ScanCtx<'s, T> {
    fn new(code: &'s str, err_msg: &Option<Rc<String>>) -> Self {
        Self {
            branches: Vec::new(),
            code_iter: code.chars(),
            err_idx: 0,
            err_msg: err_msg.clone(),
            index: 0,
            lexeme: String::new(),
        }
    }

    fn branch(self, err_msg: &Option<Rc<String>>) -> (ScanCtx<'s, T>, ScanCtx<'s, T>) {
        let new_ctx = ScanCtx {
            branches: Vec::new(),
            code_iter: self.code_iter.clone(),
            err_idx: self.index,
            err_msg: err_msg.clone().or(self.err_msg.clone()),
            index: self.index,
            lexeme: String::new(),
        };

        (new_ctx, self)
    }

    fn merge_with(mut self, mut source: ScanCtx<'s, T>, is_rule: bool, branch_fn: &BranchFn<T>) -> Progress<'s, T> {
        let steps = source.index - self.index;
            
        if source.err_idx > self.err_idx {
            self.err_idx = source.err_idx;
            self.err_msg = source.err_msg;
        }

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
        Self::new_(branch_fn, None)
    }

    pub fn new_with_err_msg(branch_fn: BranchFn<T>, err_msg: &str) -> Self {
        Self::new_(branch_fn, Some(String::from(err_msg)))
    }

    fn new_(branch_fn: BranchFn<T>, err_msg: Option<String>) -> Self {
        Rule { 
            0: Rc::new(RefCell::new(_Rule {
                branch_fn: branch_fn,
                err_msg: err_msg.map(|x| Rc::new(x)),
                parts: Vec::new(),
            }))
        }
    }

    pub fn any_char(&self) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::AnyChar);
        self
    }
    
    pub fn any_char_except(&self, exclude: Vec<char>) -> &Self {
        if exclude.len() == 0 {
            panic!("List of excluded characters is empty.");
        }
        
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::AnyCharExcept(exclude));
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
        r.parts.push(ScanFn::Alter(list));
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
        r.parts.push(ScanFn::AlterString(list));
        self
    }
    
    pub fn any_of(&self, rules: Vec<&Rule<T>>) -> &Self {
        let mut r = self.0.borrow_mut();

        match rules.len() {
            0 => panic!("You must specify rules."),
            1 => r.parts.push(ScanFn::Range(1, 1, rules[0].clone())),
            _ => r.parts.push(ScanFn::AnyOf(rules.into_iter().map(|x| x.clone()).collect())),  
        };

        self
    }
    
    pub fn at_least(&self, count: u64, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(count, u64::max_value(), rule.clone()));
        self
    }
    
    pub fn at_most(&self, count: u64, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(0, count, rule.clone()));
        self
    }
    
    pub fn between(&self, min: u64, max: u64, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(min, max, rule.clone()));
        self
    }
    
    pub fn char_in(&self, min: char, max: char) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::CharIn(min, max));
        self
    }
    
    pub fn eof(&self) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Eof);
        self
    }
    
    pub fn exact(&self, count: u64, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(count, count, rule.clone()));
        self
    }
    
    pub fn literal(&self, text: &'static str) -> &Self {
        if text.len() < 1 {
            panic!("Literal text must at least 1 character long.");
        }

        let mut r = self.0.borrow_mut();   
        r.parts.push(ScanFn::Literal(&text));
        self
    }

    pub fn literal_string(&self, text: String) -> &Self {
        if text.len() < 1 {
            panic!("Literal text must at least 1 character long.");
        }
            
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::LiteralString(text));
        self
    }

    pub fn maybe(&self, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(0, 1, rule.clone()));
        self
    }

    pub fn none_or_many(&self, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(0, u64::max_value(), rule.clone()));
        self
    }
    
    pub fn not(&self, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Not(rule.clone()));
        self
    }
    
    pub fn one(&self, rule: &Rule<T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(1, 1, rule.clone()));
        self
    }

    pub fn scan(&self, code: &str) -> Result<Vec<T>, RuleError> {
        let r = self.0.borrow();
            
        if r.parts.len() == 0 {
            panic!("Rule is not defined.");
        }
        
        let mut ctx = ScanCtx::new(code, &r.err_msg);
        let scanner = Scanner {};

        match scanner.run(self, ctx) {
            Progress::Some { steps: _, ctx: new_ctx } => ctx = new_ctx,
            Progress::No(new_ctx) => return Err(RuleError::new(code, new_ctx)),
        }
        
        if let Some(_) = ctx.code_iter.next() {
            /*
            TODO Do we need these checks?
            if ctx.has_eof {
                ctx.index -= 1;
            }
            
            if (ctx.index !== ctx.code.length)
                return RuleResult.failed<TBranch, TMeta>(ctx.errors);
            
            */
            
            Err(RuleError::new(code ,ctx))
        }
        else {
            Ok(ctx.branches)
        }
    }
}

struct Scanner { }

impl Scanner {
    fn run<'s, T>(&self, rule: &Rule<T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let r = rule.0.borrow();
        let (mut new_ctx, ctx) = ctx.branch(&r.err_msg);
        
        for p in &r.parts {
            let progress = match *p {
                ScanFn::AnyChar => self.scan_any_char_leaf(new_ctx),
                ScanFn::AnyCharExcept(ref exclude) => self.scan_any_char_except_leaf(&exclude, new_ctx),
                ScanFn::Alter(ref alter) => self.scan_alter_leaf(&alter, new_ctx),
                ScanFn::AlterString(ref alter) => self.scan_alter_string_leaf(&alter, new_ctx),
                ScanFn::AnyOf(ref rules) => self.scan_any_of(rules, new_ctx),
                ScanFn::CharIn(min, max) => self.scan_char_in_leaf(min, max, new_ctx),
                ScanFn::Eof => self.scan_eof_leaf(new_ctx),
                ScanFn::Literal(text) => self.scan_literal_leaf(&text, new_ctx),
                ScanFn::LiteralString(ref text) => self.scan_literal_leaf(&text, new_ctx),
                ScanFn::Not(ref r) => self.scan_not(r, new_ctx),
                ScanFn::Range(min, max, ref r) => self.scan_rule_range(min, max, r, new_ctx),
            };

            match progress {
                Progress::Some { steps: _, ctx: newer_ctx } => new_ctx = newer_ctx,
                Progress::No(newer_ctx) => return Progress::No(self.update_error(ctx, newer_ctx)),
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
        let (mut new_ctx, mut ctx) = ctx.branch(&None);

        for r in rules {
            match self.run(r, new_ctx) {
                Progress::Some { steps: _, ctx: new_ctx } => {
                    let r = r.0.borrow();
                    return ctx.merge_with(new_ctx, false, &r.branch_fn);
                },
                Progress::No(some_ctx) => {
                    ctx = self.update_error(ctx, some_ctx);

                    let ctxs = ctx.branch(&None);
                    new_ctx = ctxs.0;
                    ctx = ctxs.1;
                }
            }
        }

        Progress::No(self.update_error(ctx, new_ctx))
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
        let (new_ctx, ctx) = ctx.branch(&None);

        match self.run(rule, new_ctx) {
            Progress::Some { steps: _, ctx: _ } => Progress::No(ctx),
            Progress::No(_) => Progress::Some { steps: 0, ctx },
        }
    }
    
    fn scan_rule_range<'s, T>(&self, min: u64, max: u64, rule: &Rule<T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let (mut new_ctx, ctx) = ctx.branch(&None);
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
                Progress::No(initial_ctx) => {
                    new_ctx = initial_ctx;
                    break;
                }
            }
        }
        
        if count >= min && count <= max {
            let r = rule.0.borrow();
            ctx.merge_with(new_ctx, false, &r.branch_fn)
        }
        else {
            Progress::No(self.update_error(ctx, new_ctx))
        }
    }

    fn update_error<'s, T>(&self, mut target_ctx: ScanCtx<'s, T>, source_ctx: ScanCtx<'s, T>) -> ScanCtx<'s, T> {
        if source_ctx.err_idx >= target_ctx.err_idx {
            target_ctx.err_idx = source_ctx.err_idx;
            target_ctx.err_msg = source_ctx.err_msg;
        }

        target_ctx
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

    match line_counter.scan(text) {
        Ok(lines) => {
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
        },
        Err(err) => {
            unreachable!()
        }
    }
}