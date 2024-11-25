#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldElement {
    num: u64,
    prime: u64,
}

impl FieldElement {
    fn new(num: u64, prime: u64) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        FieldElement { num, prime }
    }

    fn add(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    fn sub(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let num = (self.num + self.prime - other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    fn mul(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        let num = (self.num * other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    fn pow(&self, exponent: u64) -> FieldElement {
        let exp = exponent % (self.prime - 1);
        let num = self.num.pow(exp as u32) % self.prime;
        FieldElement::new(num, self.prime)
    }

    fn div(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        let num = (self.num * other.pow(self.prime - 2).num) % self.prime;
        FieldElement::new(num, self.prime)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Self {
        match (&x, &y) {
            (Some(x), Some(y)) => {
                if y.pow(2) != x.pow(3).add(&a.mul(x).add(&b)) {
                    panic!("({},,{} is not on the curve", x.num, y.num);
                }
            }
            _ => {}
        }
        Point { x, y, a, b }
    }

    fn add(&self, other: &Point) -> Point {
        if self.a != other.a || self.b != other.b {
            panic!("Points are not on the same curve");
        }

        // 無限遠点の処理
        if self.x.is_none() {
            return other.clone();
        }
        if other.x.is_none() {
            return self.clone();
        }

        let x1 = self.x.as_ref().unwrap();
        let y1 = self.y.as_ref().unwrap();
        let x2 = other.x.as_ref().unwrap();
        let y2 = other.y.as_ref().unwrap();

        if x1 == x2 && y1 != y2 {
            return Point::new(None, None, self.a.clone(), self.b.clone());
        }

        let s = if x1 == x2 {
            // 同じ点の加算
            let num = x1.pow(2).mul(&FieldElement::new(3, x1.prime)).add(&self.a);
            let denom = y1.mul(&FieldElement::new(2, y1.prime));
            num.div(&denom)
        } else {
            // 異なる点の加算
            let num = y2.sub(y1);
            let denom = x2.sub(x1);
            num.div(&denom)
        };

        let x3 = s.pow(2).sub(x1).sub(x2);
        let y3 = s.mul(&x1.sub(&x3)).sub(y1);

        Point::new(Some(x3), Some(y3), self.a.clone(), self.b.clone())
    }

    fn scalar_mul(&self, coefficient: u64) -> Point {
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = Point::new(None, None, self.a.clone(), self.b.clone());

        while coef > 0 {
            if coef & 1 == 1 {
                result = result.add(&current);
            }
            current = current.add(&current);
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_addition() {
        let a = FieldElement::new(0, 223);
        let b = FieldElement::new(7, 223);

        let x1 = FieldElement::new(192, 223);
        let y1 = FieldElement::new(105, 223);
        let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone());

        let x2 = FieldElement::new(17, 223);
        let y2 = FieldElement::new(56, 223);
        let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone());

        let x3 = FieldElement::new(170, 223);
        let y3 = FieldElement::new(142, 223);
        let expected = Point::new(Some(x3), Some(y3), a.clone(), b.clone());

        assert_eq!(p1.add(&p2), expected);
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = FieldElement::new(0, 223);
        let b = FieldElement::new(7, 223);

        let x = FieldElement::new(47, 223);
        let y = FieldElement::new(71, 223);
        let p = Point::new(Some(x), Some(y), a.clone(), b.clone());

        let x2 = FieldElement::new(36, 223);
        let y2 = FieldElement::new(111, 223);
        let expected = Point::new(Some(x2), Some(y2), a.clone(), b.clone());

        assert_eq!(p.scalar_mul(2), expected);
    }
}
