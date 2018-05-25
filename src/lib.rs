// Copyright (c) 2015-2018 Vincent van Ingen <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

//#![feature(nll)]

use std::cell::RefCell;
use std::str::Chars;

pub type BranchFn<T> = Option<Box<Fn(Vec<T>, &str) -> Vec<T>>>;

enum Progress<'s, T: 's> {
    Some(usize, ScanCtx<'s, T>),
    No(ScanCtx<'s, T>), // TODO We can do without the ScanCtx but we need to clone the errors.
}

pub struct Rule<'s, T: 's>(RefCell<_Rule<'s, T>>);

struct _Rule<'s, T: 's> {
    branch_fn: BranchFn<T>,
    parts: Vec<ScanFn<'s, T>>,
}

pub struct RuleError {
    pub index: i64,
    pub msg: String,
}

impl Clone for RuleError {
    fn clone(&self) -> Self {
        RuleError {
            index: self.index,
            msg: self.msg.clone(),
        }
    }
}

enum ScanFn<'s, T: 's> {
    AnyChar,
    AnyCharExcept(Vec<char>),
    Alter(Vec<(&'static str, &'static str)>),
    AlterString(Vec<(String, String)>),
    AnyOf(Vec<&'s Rule<'s, T>>),
    CharIn(char, char),
    Eof,
    Literal(&'static str),
    LiteralString(String),
    Not(&'s Rule<'s, T>),
    Range(u64, u64, &'s Rule<'s, T>),
}

struct ScanCtx<'s, T: 's> {
    branches: Vec<T>,
    code_iter: Chars<'s>,
    errors: Vec<RuleError>,
    index: i64,    // TODO Change to usize? No, because we use an iterator now. Or yes if we don't use Chars.
    lexeme: String,
}

impl<'s, T> ScanCtx<'s, T> {
    fn new(code: &'s str) -> Self {
        Self {
            branches: Vec::new(),
            code_iter: code.chars(),
            errors: Vec::new(),
            index: 0,
            lexeme: String::new(),
        }
    }

    fn branch(self, is_root_of_rule: bool) -> (ScanCtx<'s, T>, ScanCtx<'s, T>) {
        let new_ctx = ScanCtx {
            branches: Vec::new(),
            code_iter: self.code_iter.clone(),
            errors: self.errors.clone(),
            index: self.index,
            lexeme: String::new(),
            // TODO metaPushed: isRootOfRule ? 0 : ctx.metaPushed,
            // TODO trail: ctx.trail.slice(0)
        };

        /* TODO
        if (isRootOfRule && this._meta) {
            newCtx.metaPushed++;
            newCtx.trail.push(this._meta);
        }
        */
        
        (new_ctx, self)
    }

    fn merge_with(mut self, mut source: ScanCtx<'s, T>, is_root_of_rule: bool, branch_fn: &BranchFn<T>) -> Progress<'s, T> {
        /* TODO
        if (isRootOfRule)
            while (source.metaPushed-- > 0)
                source.trail.pop();
        */
            
        let step = source.index - self.index;
            
        self.code_iter = source.code_iter;
        self.errors = source.errors;
        self.index = source.index;
        self.lexeme.push_str(&source.lexeme.to_string());
        // TODO self.metaPushed = 0;
        // TODO self.trail = source.trail;
        
        match branch_fn {
            Some(ref f) if is_root_of_rule => {
                self.branches.append(&mut f(source.branches, &source.lexeme));
            },
            _ => {
                self.branches.append(&mut source.branches);
            },
        }
        
        Progress::Some(step as usize, self)
    }
}

impl<'s, T> Rule<'s, T> {
    pub fn new(branch_fn: BranchFn<T>) -> Self {
        Rule { 
            0: RefCell::new(_Rule {
                branch_fn: branch_fn,
                parts: Vec::new(),
            })
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
    
    pub fn any_of(&self, rules: Vec<&'s Rule<'s, T>>) -> &Self {
        let mut r = self.0.borrow_mut();

        match rules.len() {
            0 => panic!("You must specify rules."),
            1 => r.parts.push(ScanFn::Range(1, 1, rules[0])),
            _ => r.parts.push(ScanFn::AnyOf(rules)),  
        };

        self
    }
    
    pub fn at_least(&self, count: u64, rule: &'s Rule<'s, T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(count, u64::max_value(), rule));
        self
    }
    
    pub fn at_most(&self, count: u64, rule: &'s Rule<'s, T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(0, count, rule));
        self
    }
    
    pub fn between(&self, min: u64, max: u64, rule: &'s Rule<'s, T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(min, max, rule));
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
    
    pub fn exact(&self, count: u64, rule: &'s Rule<'s, T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(count, count, rule));
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

    pub fn maybe(&self, rule: &'s Rule<'s, T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(0, 1, rule));
        self
    }

    pub fn none_or_many(&self, rule: &'s Rule<'s, T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(0, u64::max_value(), rule));
        self
    }
    
    pub fn not(&self, rule: &'s Rule<'s, T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Not(rule));
        self
    }
    
    pub fn one(&self, rule: &'s Rule<'s, T>) -> &Self {
        let mut r = self.0.borrow_mut();
        r.parts.push(ScanFn::Range(1, 1, rule));
        self
    }

    pub fn scan(&'s self, code: &'s str) -> Result<Vec<T>, Vec<RuleError>> {
        let r = self.0.borrow();
            
        if r.parts.len() == 0 {
            panic!("Rule is not defined.");
        }
        
        let mut ctx = ScanCtx::new(code);

        match self.run(ctx) {
            Progress::Some(_, new_ctx) => ctx = new_ctx,
            Progress::No(new_ctx) => return Err(new_ctx.errors),
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
            
            Err(ctx.errors)
        }
        else {
            Ok(ctx.branches)
        }
    }
    
    fn run(&'s self, ctx: ScanCtx<'s, T>) -> Progress<T> {
        let (mut new_ctx, ctx) = ctx.branch(true);
        
        let r = self.0.borrow();

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
                ScanFn::Not(r) => self.scan_not(r, new_ctx),
                ScanFn::Range(min, max, r) => self.scan_rule_range(min, max, r, new_ctx),
            };

            match progress {
                Progress::Some(_, newer_ctx) => new_ctx = newer_ctx,
                Progress::No(_) => return Progress::No(ctx),
            }
        }
        
        ctx.merge_with(new_ctx, true, &r.branch_fn)
    }

    // TODO What about a char with more codepoints?
    fn scan_any_char_except_leaf(&'s self, exclude: &Vec<char>, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let n = ctx.code_iter.next();
        
        if let Some(c) = n {
            if exclude.contains(&c) {
                return self.update_error(ctx, format!("Character '{}' is not allowed here.", c));
            }
            
            ctx.lexeme.push(c);
            ctx.index += 1;
            Progress::Some(1, ctx)
        } 
        else {
            self.update_error(ctx, String::from("End of code while checking for not allowed character."))
        }
    }

    fn scan_any_char_leaf(&self, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let n = ctx.code_iter.next();
        
        if let Some(c) = n {
            ctx.lexeme.push(c);
            ctx.index += 1;
            Progress::Some(1, ctx)
        } 
        else {
            self.update_error(ctx, String::from("End of code while checking for not allowed character."))
        }
    }

    fn scan_alter_leaf(&self, list: &Vec<(&'static str, &'static str)>, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
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

        self.update_error(ctx, String::from("Alter characters not found on this position."))
    }
    
    fn scan_alter_string_leaf(&self, list: &Vec<(String, String)>, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
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

        self.update_error(ctx, String::from("Alter characters not found on this position."))
    }
    
    fn scan_any_of(&self, rules: &Vec<&'s Rule<'s, T>>, ctx: ScanCtx<'s, T>) -> Progress<'s,T> {
        let (mut new_ctx, mut ctx) = ctx.branch(false);

        for r in rules {
            if let Progress::Some(_, new_ctx) = r.run(new_ctx) {
                let r = self.0.borrow();
                return ctx.merge_with(new_ctx, false, &r.branch_fn);
            } 

            let ctxs = ctx.branch(false);
            new_ctx = ctxs.0;
            ctx = ctxs.1;
        }

        Progress::No(ctx)
    }
    
    fn scan_char_in_leaf(&self, min: char, max: char, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let c = ctx.code_iter.next();

        match c {
            Some(c) => {
                if c < min || c > max {
                    self.update_error(ctx, format!("Expected a character between '{}' and '{}'; got a {}.", min, max, c))
                }
                else {
                    ctx.lexeme.push(c);
                    ctx.index += 1;
                    Progress::Some(1, ctx)
                }
            },
            None => {
                self.update_error(ctx, format!("End of code. Expected a character between '{}' and '{}'.", min, max))
            }
        }
    }
    
    fn scan_eof_leaf(&self, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        if let None = ctx.code_iter.next() {
            ctx.index += 1;
            Progress::Some(1, ctx)
        }
        else {
            self.update_error(ctx, String::from("No EOF on this position."))
        }
    }
    
    fn scan_literal_leaf(&self, find: &str, mut ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let iter = find.chars();
        let mut step = 0;
            
        for i in iter {
            let n = ctx.code_iter.next();
                
            if let Some(c) = n {
                if i != c {
                    return self.update_error(ctx, format!("The literal '{}' not found.", find));
                }
                    
                ctx.index += 1;
                step += 1;
            }
            else {
                return self.update_error(ctx, format!("End of code. The literal '{}' not found.", find));
            }
        }
        
        ctx.lexeme.push_str(find);
        Progress::Some(step as usize, ctx)
    }

    fn scan_not(&self, rule: &'s Rule<'s, T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let (new_ctx, ctx) = ctx.branch(false);

        match rule.run(new_ctx) {
            Progress::Some(_, _) => Progress::No(ctx),
            Progress::No(_) => Progress::Some(0, ctx),
        }
    }

    fn scan_rule_range(&self, min: u64, max: u64, rule: &'s Rule<'s, T>, ctx: ScanCtx<'s, T>) -> Progress<'s, T> {
        let (mut new_ctx, ctx) = ctx.branch(false);
        let mut count = 0u64;

        loop {
            match rule.run(new_ctx) {
                Progress::Some(progress, newer_ctx) => {
                    if progress == 0 {
                        let r = self.0.borrow();
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
            let r = self.0.borrow();
            ctx.merge_with(new_ctx, false, &r.branch_fn)
        }
        else {
            Progress::No(ctx)
        }
    }

    fn update_error(&self, mut ctx: ScanCtx<'s, T>, error_msg: String) -> Progress<'s, T> {
        if ctx.errors.len() != 0 {
            let err_idx = ctx.errors[0].index;
                
            if ctx.index < err_idx {
                return Progress::No(ctx);
            }
                
            if ctx.index > err_idx {
                ctx.errors.clear();
            }
        }
            
        ctx.errors.push(RuleError {
            index: ctx.index,
            msg: error_msg,
            // TODO trail: newCtx.trail.slice(0)
        });
            
        Progress::No(ctx)
    }
}