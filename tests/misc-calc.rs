use rule::{Rule, RuleError};

struct Calc<'a> {
    expr: Rule<'a, f64>,
}

impl<'a> Calc<'a> {
    fn new() -> Self {
        let expr: Rule<f64> = Rule::default();
        let add: Rule<f64> = Rule::new(&|b, _| if b.len() == 1 { b[0] } else { b[0] + b[1] } );
        let mul: Rule<f64> = Rule::new(&|b, _| if b.len() == 1 { b[0] } else { b[0] * b[1] } );

        let digit = Rule::default();
        digit.char_in('0', '9');

        let num = Rule::new(&|_, l| l.parse().unwrap());
        num.at_least(1, &digit);

        let group = Rule::default();
        group.literal("(").one(&expr).literal(")"); 

        let mul_right = Rule::default();
        mul_right.literal("*").one(&mul);
        mul.any_of(vec![&num, &group]).maybe(&mul_right);

        let add_right = Rule::default();
        add_right.literal("+").one(&add);
        add.one(&mul).maybe(&add_right);

        expr.any_of(vec![&add, &group]);

        Self { expr }
    }

    fn eval(&self, expr: &str) -> Result<f64, RuleError> {
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