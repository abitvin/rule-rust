// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

use std::cell::RefCell;
use std::rc::Rc;
use std::str::Chars;

pub type BranchFn<T> = Option<Box<Fn(Vec<T>, &str) -> T>>;

enum Progress<'s, T> {
    Some(usize, ScanCtx<'s, T>),
    No(ScanCtx<'s, T>), // TODO We can do without the ScanCtx but we need to clone the errors.
}

pub struct Rule<T>(Rc<RefCell<_Rule<T>>>);

impl<T> Clone for Rule<T> {
    fn clone(&self) -> Self {
        Self { 0: self.0.clone() }
    }
}

struct _Rule<T> {
    branch_fn: BranchFn<T>,
    err_msg: Option<String>,
    parts: Vec<ScanFn<T>>,
}

// TODO implement Error trait.
#[derive(Debug)]
pub struct RuleError {
    pub index: i64,
    pub msg: String,
}

impl Clone for RuleError {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            msg: self.msg.clone(),
        }
    }
}

impl<'s, T> From<ScanCtx<'s, T>> for RuleError {
    fn from(ctx: ScanCtx<'s, T>) -> Self {
        Self {
            index: ctx.err_idx,
            msg: ctx.err_msg.unwrap_or(String::from("General parse error.")),
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
    err_idx: i64,
    err_msg: Option<String>,    // TODO Can we use a borrow instead?
    index: i64,                 // TODO Change to usize? No, because we use an iterator now. Or yes if we don't use Chars.
    lexeme: String,
}

impl<'s, T> ScanCtx<'s, T> {
    fn new(code: &'s str, err_msg: Option<String>) -> Self {
        Self {
            branches: Vec::new(),
            code_iter: code.chars(),
            err_idx: 0,
            err_msg,
            index: 0,
            lexeme: String::new(),
        }
    }

    fn branch(self, err_msg: Option<String>) -> (ScanCtx<'s, T>, ScanCtx<'s, T>) {
        let new_ctx = ScanCtx {
            branches: Vec::new(),
            code_iter: self.code_iter.clone(),
            err_idx: self.index,
            err_msg: err_msg.or(self.err_msg.clone()),
            index: self.index,
            lexeme: String::new(),
        };

        (new_ctx, self)
    }

    fn merge_with(mut self, mut source: ScanCtx<'s, T>, is_root_of_rule: bool, branch_fn: &BranchFn<T>) -> Progress<'s, T> {
        let step = source.index - self.index;
            
        if source.err_idx > self.err_idx {
            self.err_idx = source.err_idx;
            self.err_msg = source.err_msg;
        }

        self.code_iter = source.code_iter;
        self.index = source.index;
        self.lexeme.push_str(&source.lexeme.to_string());
        
        match branch_fn {
            Some(ref f) if is_root_of_rule => {
                self.branches.push(f(source.branches, &source.lexeme));
            },
            _ => {
                self.branches.append(&mut source.branches);
            },
        }
        
        Progress::Some(step as usize, self)
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
                err_msg,
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
        
        let mut ctx = ScanCtx::new(code, r.err_msg.clone());
        let scanner = Scanner {};

        match scanner.run(self, ctx) {
            Progress::Some(_, new_ctx) => ctx = new_ctx,
            Progress::No(new_ctx) => return Err(RuleError::from(new_ctx)),
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
            
            Err(RuleError::from(ctx))
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
        let (mut new_ctx, ctx) = ctx.branch(r.err_msg.clone());      // TODO Do we need to clone?
        
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
                Progress::Some(_, newer_ctx) => new_ctx = newer_ctx,
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
            Progress::Some(1, ctx)
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
            Progress::Some(1, ctx)
        } 
        else {
            Progress::No(ctx)
        }
    }
    
    fn scan_alter_leaf<'s, T>(&self, list: &Vec<(&'static str, &'static str)>, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        for alter in list {
            let find = alter.0;
            let len = find.chars().count();
            let compare: String = ctx.code_iter.clone().take(len).collect();

            if find == compare {
                ctx.code_iter.nth(len - 1);
                ctx.lexeme.push_str(alter.1);
                ctx.index += len as i64;    // TODO As usize instead of i64
                return Progress::Some(len as usize, ctx);
            }
        }

        Progress::No(ctx)
    }
    
    fn scan_alter_string_leaf<'s, T>(&self, list: &Vec<(String, String)>, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        for alter in list {
            let find = &alter.0;
            let len = find.chars().count();
            let compare: String = ctx.code_iter.clone().take(len).collect();

            if *find == compare {
                ctx.code_iter.nth(len - 1);
                ctx.lexeme.push_str(&alter.1);
                ctx.index += len as i64;    // TODO As usize instead of i64
                return Progress::Some(len as usize, ctx);
            }
        }

        Progress::No(ctx)
    }
    
    fn scan_any_of<'s, T>(&self, rules: &Vec<Rule<T>>, ctx: ScanCtx<'s, T>) -> Progress<'s,T> {
        let (mut new_ctx, mut ctx) = ctx.branch(None);

        for r in rules {
            match self.run(r, new_ctx) {
                Progress::Some(_, new_ctx) => {
                    let r = r.0.borrow();
                    return ctx.merge_with(new_ctx, false, &r.branch_fn);
                },
                Progress::No(some_ctx) => {
                    ctx = self.update_error(ctx, some_ctx);

                    let ctxs = ctx.branch(None);
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
                    Progress::Some(1, ctx)
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
            Progress::Some(1, ctx)
        }
        else {
            Progress::No(ctx)
        }
    }
    
    fn scan_literal_leaf<'s, T>(&self, find: &str, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let iter = find.chars();
        let mut step = 0;
            
        for i in iter {
            let n = ctx.code_iter.next();
                
            if let Some(c) = n {
                if i != c {
                    return Progress::No(ctx);
                }
                    
                ctx.index += 1;
                step += 1;
            }
            else {
                return Progress::No(ctx);
            }
        }
        
        ctx.lexeme.push_str(find);
        Progress::Some(step as usize, ctx)
    }
    
    fn scan_not<'s, T>(&self, rule: &Rule<T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let (new_ctx, ctx) = ctx.branch(None);

        match self.run(rule, new_ctx) {
            Progress::Some(_, _) => Progress::No(ctx),
            Progress::No(_) => Progress::Some(0, ctx),
        }
    }
    
    fn scan_rule_range<'s, T>(&self, min: u64, max: u64, rule: &Rule<T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let (mut new_ctx, ctx) = ctx.branch(None);
        let mut count = 0u64;
        
        loop {
            match self.run(rule, new_ctx) {
                Progress::Some(progress, newer_ctx) => {
                    if progress == 0 {
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