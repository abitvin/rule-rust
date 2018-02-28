extern crate rule;
use rule::Rule;

#[test]
fn misc_calc()
{
    unsafe {
        // Predeclare add, expr and mul.
        let mut expr: Rule<f64> = Rule::new(None);
        let mut add = Rule::new(Some(Box::new(|b, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] + b[1]] } )));
        let mut mul = Rule::new(Some(Box::new(|b, _| if b.len() == 1 { vec![b[0]] } else { vec![b[0] * b[1]] } )));

        let mut digit = Rule::new(None);
        digit.char_in('0', '9');

        let mut num = Rule::new(Some(Box::new(|_, l| vec![l.parse().unwrap()])));
        num.at_least(1, digit);

        let mut brackets = Rule::new(None);
        brackets.literal("(").one_raw(&expr).literal(")"); 

        let mut mul_right = Rule::new(None);
        mul_right.literal("*").one_raw(&mul);
        mul.any_of_raw(vec![&num, &brackets]).maybe(mul_right);

        let mut add_right = Rule::new(None);
        add_right.literal("+").one_raw(&add);
        add.one_raw(&mul).maybe(add_right);

        expr.any_of_raw(vec![&add, &brackets]);

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
}