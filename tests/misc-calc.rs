extern crate rule;
use rule::{Rule, RuleError};
use std::cell::RefCell;
use std::rc::Rc;

struct Calc {
    expr: Rule<f64>,
}

impl Calc {
    fn new() -> Self {
        let expr: Rule<f64> = Rule::new(None);
        let add = Rule::new(Some(Box::new(|b, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] + b[1]] } )));
        let mul = Rule::new(Some(Box::new(|b, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] * b[1]] } )));

        let digit = Rule::new(None);
        digit.char_in('0', '9');

        let num = Rule::new(Some(Box::new(|_, l| vec![l.parse().unwrap()])));
        num.at_least(1, &digit);

        let group = Rule::new(None);
        group.literal("(").one(&expr).literal(")"); 

        let mul_right = Rule::new(None);
        mul_right.literal("*").one(&mul);
        mul.any_of(vec![&num, &group]).maybe(&mul_right);

        let add_right = Rule::new(None);
        add_right.literal("+").one(&add);
        add.one(&mul).maybe(&add_right);

        expr.any_of(vec![&add, &group]);

        Self { expr }
    }

    fn eval(&self, expr: &str) -> Result<f64, Vec<RuleError>> {
        self.expr.scan(expr).map(|x| x[0])
    }
}

#[test]
fn misc_calc() {
    let calc = Calc::new();
    
    if let Ok(val) = calc.eval("2*(3*4*5)") {
        assert_eq!(val, 120f64);
    }
    else {
        assert!(false);
    }

    if let Ok(val) = calc.eval("2*(3+4)*5") {
        assert_eq!(val, 70f64);
    }
    else {
        assert!(false);
    }

    if let Ok(val) = calc.eval("((2+3*4+5))") {
        assert_eq!(val, 19f64);
    }
    else {
        assert!(false);
    }
}