use std::error::Error;

pub fn long_division(dividend: i64, divisor: i64) -> Result<(i64, i64), Box<dyn Error>> {
    if divisor == 0 {
        return Err("Cannot divide by zero".into());
    }

    let mut quotient = 0;
    let dividend_sign = if dividend < 0 { -1 } else { 1 };
    let divisor_sign = if divisor < 0 { -1 } else { 1 };
    let mut remainder = dividend.abs();
    let divisor = divisor.abs();

    loop {
        if remainder - divisor < 0 {
            break;
        }

        quotient += 1;
        remainder -= divisor;
    }

    if dividend_sign == -1 && divisor_sign == -1 {
        remainder *= -1;
    } else if dividend_sign == 1 && divisor_sign == -1 {
        quotient *= -1;
    } else if dividend_sign == -1 && divisor_sign == 1 {
        quotient *= -1;
        remainder *= -1;
    }

    Ok((quotient, remainder))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_division_by_zero() {
        let res = long_division(9, 0);
        assert!(res.is_err());
    }

    #[test]
    fn remainder_correctness() -> Result<(), Box<dyn Error>> {
        let (q, r) = long_division(27, 5)?;
        assert_eq!(q, 5);
        assert_eq!(r, 2);

        let (q, r) = long_division(-27, 5)?;
        assert_eq!(q, -5);
        assert_eq!(r, -2);

        let (q, r) = long_division(-27, -5)?;
        assert_eq!(q, 5);
        assert_eq!(r, -2);

        let (q, r) = long_division(27, -5)?;
        assert_eq!(q, -5);
        assert_eq!(r, 2);

        Ok(())
    }
}
