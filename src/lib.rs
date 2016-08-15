// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

use std::str::Chars;

// TODO What is the best way to store the branch closure?
// http://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-rust
// TODO Simplify `Option<Box<Fn<...>>>` if we can.
pub type BranchFn<T, S> = Option<Box<Fn(Vec<T>, &str, &mut S) -> Vec<T>>>;

// TODO Better name and conceptual this enum isn't correct either.
// We have Some(0) in our code and Some(0) is no progress! And `No` is a "no match" state. We need a third state. 
enum Progress<'b, T> {
    Some(usize, ScanCtx<'b, T>),
    No(ScanCtx<'b, T>), // TODO We can do without the ScanCtx but we need to clone the errors.
}

pub struct Rule<T, S> {
    pub branch_fn: BranchFn<T, S>,
    parts: Vec<ScanFn<T, S>>,
}

pub struct RuleError {
    pub index: i64,
    pub msg: String,
    // TODO trail: TMeta[]
}

impl Clone for RuleError 
{
    fn clone(&self) -> Self
    {
        RuleError {
            index: self.index,
            msg: self.msg.clone(),
        }
    }
}

enum ScanFn<T, S> {
    All,
    AllExcept(Vec<char>),
    Alter(Vec<(&'static str, &'static str)>),
    AlterString(Vec<(String, String)>),
    AnyOfOwned(Vec<Rule<T, S>>),
    AnyOfRaw(Vec<*const Rule<T, S>>),
    CharIn(char, char),
    Eof,
    Literal(&'static str),
    LiteralString(String),
    NotOwned(Rule<T, S>),
    NotRaw(*const Rule<T, S>),
    RangeOwned(u64, u64, Rule<T, S>),
    RangeRaw(u64, u64, *const Rule<T, S>),
}

impl<T, S> ScanFn<T, S> 
{
    // TODO It's not really a shallow_clone...
    fn shallow_clone(&self) -> Self 
    {
        match *self {
            ScanFn::All => ScanFn::All,
            ScanFn::AllExcept(ref v) => ScanFn::AllExcept(v.clone()),
            ScanFn::Alter(ref v) => ScanFn::Alter(v.clone()),
            ScanFn::AlterString(ref v) => ScanFn::AlterString(v.clone()),
            ScanFn::AnyOfOwned(ref v) => ScanFn::AnyOfOwned(v.iter().map(|r| r.shallow_clone_b()).collect()),
            ScanFn::AnyOfRaw(ref v) => ScanFn::AnyOfRaw(v.clone()),
            ScanFn::CharIn(min, max) => ScanFn::CharIn(min, max),
            ScanFn::Eof => ScanFn::Eof,
            ScanFn::Literal(ref s) => ScanFn::Literal(s),
            ScanFn::LiteralString(ref s) => ScanFn::LiteralString(s.clone()),
            ScanFn::NotOwned(ref r) => ScanFn::NotOwned(r.shallow_clone_b()),
            ScanFn::NotRaw(r) => ScanFn::NotRaw(r),
            ScanFn::RangeOwned(min, max, ref rule) => ScanFn::RangeOwned(min, max, rule.shallow_clone_b()), 
            ScanFn::RangeRaw(min, max, rule) => ScanFn::RangeRaw(min, max, rule),
        }
    }
}

struct ScanCtx<'b, T> {
    branches: Vec<T>,
    code_iter: Chars<'b>,
    errors: Vec<RuleError>,
    index: i64,    // TODO Change to usize? No, because we use an iterator now. Or yes if we don't use Chars.
    lexeme: String,
    // TODO metaPushed: number;
    // TODO trail: TMeta[];
}

impl<T, S> Rule<T, S>
{
    pub fn new(branch_fn: BranchFn<T, S>) -> Self
    {
        Rule { 
            branch_fn: branch_fn,
            parts: Vec::new(),
        }
    }

    // TODO Rename to `any_char`, also in TypeScript
    pub fn all(&mut self) -> &mut Self
    {
        self.parts.push(ScanFn::All);
        self
    }
    
    // TODO Rename to `any_char_except`, also in TypeScript
    pub fn all_except(&mut self, exclude: Vec<char>) -> &mut Self
    {
        if exclude.len() == 0 {
            panic!("List of excluded characters is empty.");
        }
        
        self.parts.push(ScanFn::AllExcept(exclude));
        self
    }
    
    pub fn alter(&mut self, list: Vec<(&'static str, &'static str)>) -> &mut Self
    {
        if list.len() == 0 {
            panic!("List is empty.");
        }
        
        if !list.iter().any(|t| { t.0.len() > 0 && t.1.len() > 0 }) {
            panic!("The strings in the list must be minimal one character long.");
        }
        
        self.parts.push(ScanFn::Alter(list));
        self
    }

    pub fn alter_string(&mut self, list: Vec<(String, String)>) -> &mut Self
    {
        if list.len() == 0 {
            panic!("List is empty.");
        }
        
        if !list.iter().any(|ref t| { t.0.len() > 0 && t.1.len() > 0 }) {
            panic!("The strings in the list must be minimal one character long.");
        }
        
        self.parts.push(ScanFn::AlterString(list));
        self
    }
    
    pub fn any_of(&mut self, mut rules: Vec<Rule<T, S>>) -> &mut Self
    {
        match rules.len() {
            0 => panic!("You must specify rules."),
            1 => self.parts.push(ScanFn::RangeOwned(1, 1, rules.pop().unwrap())),
            _ => self.parts.push(ScanFn::AnyOfOwned(rules)),
        };
        
        self
    }
    
    pub unsafe fn any_of_raw(&mut self, rules: Vec<*const Rule<T, S>>) -> &mut Self
    {
        match rules.len() {
            0 => panic!("You must specify rules."),
            1 => self.parts.push(ScanFn::RangeRaw(1, 1, rules[0])),
            _ => self.parts.push(ScanFn::AnyOfRaw(rules)),  
        };
        
        self
    }
    
    pub fn at_least(&mut self, count: u64, rule: Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeOwned(count, u64::max_value(), rule));
        self
    }
    
    pub unsafe fn at_least_raw(&mut self, count: u64, rule: *const Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeRaw(count, u64::max_value(), rule));
        self
    }
    
    pub fn at_most(&mut self, count: u64, rule: Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeOwned(0, count, rule));
        self
    }
    
    pub unsafe fn at_most_raw(&mut self, count: u64, rule: *const Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeRaw(0, count, rule));
        self
    }
    
    pub fn between(&mut self, min: u64, max: u64, rule: Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeOwned(min, max, rule));
        self
    }
    
    pub unsafe fn between_raw(&mut self, min: u64, max: u64, rule: *const Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeRaw(min, max, rule));
        self
    }
    
    pub fn char_in(&mut self, min: char, max: char) -> &mut Self
    {
        self.parts.push(ScanFn::CharIn(min, max));
        self
    }
    
    pub fn clear(&mut self) -> &mut Self
    {
        self.parts.clear();
        self
    }
    
    pub fn eof(&mut self) -> &mut Self
    {
        self.parts.push(ScanFn::Eof);
        self
    }
    
    pub fn exact(&mut self, count: u64, rule: Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeOwned(count, count, rule));
        self
    }
    
    pub unsafe fn exact_raw(&mut self, count: u64, rule: *const Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeRaw(count, count, rule));
        self
    }
    
    pub fn literal(&mut self, text: &'static str) -> &mut Self
    {
        if text.len() < 1 {
            panic!("Literal text must at least 1 character long.");
        }
            
        self.parts.push(ScanFn::Literal(&text));
        self
    }

    pub fn literal_string(&mut self, text: String) -> &mut Self
    {
        if text.len() < 1 {
            panic!("Literal text must at least 1 character long.");
        }
            
        self.parts.push(ScanFn::LiteralString(text));
        self
    }
    
    pub fn maybe(&mut self, rule: Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeOwned(0, 1, rule));
        self
    }
    
    pub unsafe fn maybe_raw(&mut self, rule: *const Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeRaw(0, 1, rule));
        self
    }
    
    pub fn none_or_many(&mut self, rule: Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeOwned(0, u64::max_value(), rule));
        self
    }
    
    pub unsafe fn none_or_many_raw(&mut self, rule: *const Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeRaw(0, u64::max_value(), rule));
        self
    }
    
    pub fn not(&mut self, rule: Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::NotOwned(rule));
        self
    }
    
    pub unsafe fn not_raw(&mut self, rule: *const Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::NotRaw(rule));
        self
    }
    
    pub fn one(&mut self, rule: Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeOwned(1, 1, rule));
        self
    }
    
    pub unsafe fn one_raw(&mut self, rule: *const Rule<T, S>) -> &mut Self
    {
        self.parts.push(ScanFn::RangeRaw(1, 1, rule));
        self
    }
    
    // TODO If we can add raw Rules then by defenition this `scan` function is unsafe.
    pub fn scan(&self, code: &str, mut shared: &mut S) -> Result<Vec<T>, Vec<RuleError>>
    {
        let mut ctx = ScanCtx {
            branches: Vec::new(),
            code_iter: code.chars(),
            errors: Vec::new(),
            index: 0,
            lexeme: String::new(),
        };
    
        match self.run(ctx, &mut shared) {
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

    // TODO It's not really a shallow_clone...
    pub unsafe fn shallow_clone(&self, branch_fn: BranchFn<T, S>) -> Self
    {
        Rule {
            branch_fn: branch_fn,
            parts: self.parts.iter().map(|p| p.shallow_clone()).collect(),
        }
    }

    // Private functions

    #[allow(unused_variables)]   // TODO This warning is for `is_root_of_rule` which we must implement.
    fn branch<'b>(&'b self, ctx: &ScanCtx<'b, T>, is_root_of_rule: bool) -> ScanCtx<T>
    {
        let new_ctx = ScanCtx {
            branches: Vec::new(),
            code_iter: ctx.code_iter.clone(),
            errors: ctx.errors.clone(),
            index: ctx.index,
            lexeme: String::new(),
            // TODO metaPushed: isRootOfRule ? 0 : ctx.metaPushed,
            // TODO trail: ctx.trail.slice(0)
        };

        /* TODO
        if (isRootOfRule && this._meta)
        {
            newCtx.metaPushed++;
            newCtx.trail.push(this._meta);
        }
        */
        
        new_ctx
    }
    
    fn merge<'b>(&self, mut target: ScanCtx<'b, T>, mut source: ScanCtx<'b, T>, is_root_of_rule: bool, mut state: &mut S) -> Progress<'b, T>
    {
        /* TODO
        if (isRootOfRule)
            while (source.metaPushed-- > 0)
                source.trail.pop();
        */
            
        let step = source.index - target.index;
            
        target.code_iter = source.code_iter;
        target.errors = source.errors;
        target.index = source.index;
        target.lexeme.push_str(&source.lexeme.to_string());
        // TODO target.metaPushed = 0;
        // TODO target.trail = source.trail;
        
        match self.branch_fn {
            Some(ref f) if is_root_of_rule => {
                target.branches.append(&mut f(source.branches, &source.lexeme, &mut state));
            },
            _ => {
                target.branches.append(&mut source.branches);
            },
        }
        
        Progress::Some(step as usize, target)
    }
    
    fn run<'b>(&'b self, ctx: ScanCtx<'b, T>, mut shared: &mut S) -> Progress<T>
    {
        if self.parts.len() == 0 {
            panic!("Rule is not defined.");
        }
        
        let mut new_ctx = self.branch(&ctx, true);
        
        for p in &self.parts {
            let progress = match *p {
                ScanFn::All => self.scan_all_leaf(new_ctx),
                ScanFn::AllExcept(ref exclude) => self.scan_all_except_leaf(&exclude, new_ctx),
                ScanFn::Alter(ref alter) => self.scan_alter_leaf(&alter, new_ctx),
                ScanFn::AlterString(ref alter) => self.scan_alter_string_leaf(&alter, new_ctx),
                ScanFn::AnyOfOwned(ref rules) => self.scan_any_of_owned(rules, new_ctx, &mut shared),
                ScanFn::AnyOfRaw(ref rules) => self.scan_any_of_raw(rules, new_ctx, &mut shared),
                ScanFn::CharIn(min, max) => self.scan_char_in_leaf(min, max, new_ctx),
                ScanFn::Eof => self.scan_eof_leaf(new_ctx),
                ScanFn::Literal(find) => self.scan_literal_leaf(&find, new_ctx),
                ScanFn::LiteralString(ref text) => self.scan_literal_leaf(&text, new_ctx),
                ScanFn::NotOwned(ref r) => self.scan_not(r as *const Rule<T, S>, new_ctx, &mut shared),
                ScanFn::NotRaw(r) => self.scan_not(r, new_ctx, &mut shared),
                ScanFn::RangeOwned(min, max, ref r) => self.scan_rule_range(min, max, r as *const Rule<T, S>, new_ctx, &mut shared),
                ScanFn::RangeRaw(min, max, r) => self.scan_rule_range(min, max, r, new_ctx, &mut shared),
            };

            match progress {
                Progress::Some(_, newer_ctx) => new_ctx = newer_ctx,
                Progress::No(_) => return Progress::No(ctx),
            }
        }
        
        self.merge(ctx, new_ctx, true, &mut shared)
    }
    
    // TODO What about a char with more codepoints?
    fn scan_all_except_leaf<'b>(&'b self, exclude: &Vec<char>, mut ctx: ScanCtx<'b, T>) -> Progress<T>
    {
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
    
    // TODO What about a char with more codepoints?
    fn scan_all_leaf<'b>(&'b self, mut ctx: ScanCtx<'b, T>) -> Progress<T> 
    {
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
    
    fn scan_alter_leaf<'b>(&'b self, list: &Vec<(&'static str, &'static str)>, mut ctx: ScanCtx<'b, T>) -> Progress<T>
    {
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

    fn scan_alter_string_leaf<'b>(&'b self, list: &Vec<(String, String)>, mut ctx: ScanCtx<'b, T>) -> Progress<T>
    {
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
    
    fn scan_any_of_owned<'b>(&'b self, rules: &'b Vec<Rule<T, S>>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
    {
        for r in rules {
            let new_ctx = self.branch(&ctx, false);

            if let Progress::Some(_, new_ctx) = r.run(new_ctx, &mut state) {
                return self.merge(ctx, new_ctx, false, &mut state);
            } 
        }

        Progress::No(ctx)
    }
    
    fn scan_any_of_raw<'b>(&'b self, rules: &Vec<*const Rule<T, S>>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
    {
        for r in rules {
            let new_ctx = self.branch(&ctx, false);

            if let Progress::Some(_, new_ctx) = unsafe { (**r).run(new_ctx, &mut state) } {
                return self.merge(ctx, new_ctx, false, &mut state);
            } 
        }

        Progress::No(ctx)
    }
    
    fn scan_char_in_leaf<'b>(&'b self, min: char, max: char, mut ctx: ScanCtx<'b, T>) -> Progress<T>
    {
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
    
    fn scan_eof_leaf<'b>(&'b self, mut ctx: ScanCtx<'b, T>) -> Progress<T>
    {
        if let None = ctx.code_iter.next() {
            ctx.index += 1;
            Progress::Some(1, ctx)
        }
        else {
            self.update_error(ctx, String::from("No EOF on this position."))
        }
    }
    
    fn scan_literal_leaf<'b>(&'b self, find: &str, mut ctx: ScanCtx<'b, T>) -> Progress<T> 
    {
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
    
    fn scan_not<'b>(&'b self, rule: *const Rule<T, S>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
    {
        match unsafe { (*rule).run(self.branch(&ctx, false), &mut state) } {
            Progress::Some(_, _) => Progress::No(ctx),
            Progress::No(_) => Progress::Some(0, ctx),
        }
    }
    
    fn scan_rule_range<'b>(&'b self, min: u64, max: u64, rule: *const Rule<T, S>, ctx: ScanCtx<'b, T>, mut state: &mut S) -> Progress<T>
    {
        let mut new_ctx = self.branch(&ctx, false);
        let mut count = 0u64;

        loop {
            match unsafe { (*rule).run(new_ctx, &mut state) } {
                Progress::Some(progress, newer_ctx) => {
                    if progress == 0 {
                        return Progress::Some(0, ctx);
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
            self.merge(ctx, new_ctx, false, &mut state)
        }
        else {
            Progress::No(ctx)
        }
    }

    // TODO Better name
    fn shallow_clone_b(&self) -> Self
    {
        if self.branch_fn.is_some() {
            panic!("Could not `shallow_clone` because a rule part intergral of the root rule you want to `shallow_clone` has a branch_fn we cannot clone.");
        }

        Rule {
            branch_fn: None,
            parts: self.parts.iter().map(|p| p.shallow_clone()).collect(),
        }
    }

    fn update_error<'b>(&'b self, mut ctx: ScanCtx<'b, T>, error_msg: String) -> Progress<T>
    {
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