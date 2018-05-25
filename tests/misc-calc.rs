#![feature(nll)]

extern crate rule;
use rule::Rule;

#[test]
fn misc_calc()
{
    // Predeclare add, expr and mul.
    let expr: Rule<f64> = Rule::new(None);
    let add = Rule::new(Some(Box::new(|b, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] + b[1]] } )));
    let mul = Rule::new(Some(Box::new(|b, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] * b[1]] } )));

    let digit = Rule::new(None);
    digit.char_in('0', '9');

    let num = Rule::new(Some(Box::new(|_, l| vec![l.parse().unwrap()])));
    num.at_least(1, &digit);

    let brackets = Rule::new(None);
    brackets.literal("(").one(&expr).literal(")"); 

    let mul_right = Rule::new(None);
    mul_right.literal("*").one(&mul);
    mul.any_of(vec![&num, &brackets]).maybe(&mul_right);

    let add_right = Rule::new(None);
    add_right.literal("+").one(&add);
    add.one(&mul).maybe(&add_right);

    expr.any_of(vec![&add, &brackets]);

    if let Ok(branches) = expr.scan("2*(3*4*5)") {
        assert_eq!(branches[0], 120f64);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = expr.scan("2*(3+4)*5") {
        assert_eq!(branches[0], 70f64);
    }
    else {
        assert!(false);
    }

    if let Ok(branches) = expr.scan("((2+3*4+5))") {
        assert_eq!(branches[0], 19f64);
    }
    else {
        assert!(false);
    }
}