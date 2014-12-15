use std::num::SignedInt;

#[deriving(Show)]
pub struct Rational {
	num: i32,
	den: i32
}

fn gcd(m:i32, n: i32)	-> i32	{
	let mut a = m;
	let mut b = n;
	while b != 0	{
		let r = a % b;
		a = b;
		b =	r;	
	}
	a.abs()
}

fn reduce(n: i32, d: i32)	-> (i32, i32)	{
	let g = gcd(n, d);
	let mut a = n / g;
	let mut b = d / g;

	if b.is_negative()	{
		a = -a;
		b = -b;
	}
	(a, b)
}

impl Rational	{
	pub fn new(num: i32, den: i32) -> Rational {
		if den == 0	{
			panic!("den cannot be zero");
		}
		let (a, b) = reduce(num, den);
		Rational{num: a, den: b}
	}

	pub fn num(&self) -> i32 {
		self.num
	}
	pub fn den(&self) -> i32 {
		self.den
	}

	pub fn one() -> Rational	{
		Rational{num: 1, den: 1}
	}

	pub fn zero() -> Rational {
		Rational{num: 0, den: 1}
	}
}

impl Copy for Rational	{
}

impl Add<Rational, Rational> for Rational {
	fn add(&self, other: &Rational) -> Rational {
		Rational::new(self.num * other.den + other.num * self.den, self.den * other.den)
	}
}

impl Sub<Rational, Rational> for Rational	{
	fn sub(&self, other: &Rational) -> Rational	{
		Rational::new(self.num * other.den - other.num * self.den, self.den * other.den)
	}
}

impl Mul<Rational, Rational> for Rational	{
	fn mul(&self, other: &Rational) -> Rational	{
		Rational::new(self.num *  other.num, self.den * other.den)
	}
}

#[cfg(test)]
mod test {
	use super::Rational;
	use super::reduce;
	use super::gcd;

	#[test]
	fn check_reduce()	{
		assert_eq!(reduce(2, 3), (2, 3));
		assert_eq!(reduce(-2, 3), (-2, 3));
		assert_eq!(reduce(2, 4), (1, 2));
		assert_eq!(reduce(13, 37), (13, 37));
	}

	#[test]
	fn check_gdc()	{
		assert_eq!(gcd(12, 0), 12);
		assert_eq!(gcd(21, 6), 3);
		assert_eq!(gcd(1, 1), 1);
		assert_eq!(gcd(2, 4), 2);
		assert_eq!(gcd(-2, 4), 2);
		assert_eq!(gcd(2, -4), 2);
	}

	#[test]
	fn check_add()	{
		let x = Rational::new(1, 2) + Rational::new(1, 2);
		assert_eq!(x.num, 1);
		assert_eq!(x.den, 1);
	}

	#[test]
	fn check_sub()	{
		let x = Rational::new(1, 2) - Rational::new(1, 2);
		assert_eq!(x.num, 0);
		assert_eq!(x.den, 1);
	}

	#[test]
	fn check_mul()	{
		let x = Rational::new(1, 2) * Rational::new(1, 2);
		assert_eq!(x.num, 1);
		assert_eq!(x.den, 4);
	}
}
