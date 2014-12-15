pub mod rational;

/*
use std::num::SignedInt;

struct Rational {
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

fn lcm(a: i32, b: i32) -> i32 {
	match(a, b)	{
		(0, 0) => 0,
		(a, b) => (a.abs() / gcd(a, b)) * b.abs()
	}
}

impl Rational	{
	pub fn new(num: i32, den: i32) -> Rational {
		if den == 0	{
			panic!("den cannot be zero");
		}
		let (a, b) = reduce(num, den);
		Rational{num: a, den: b}
	}

	pub fn one() -> Rational	{
		Rational{num: 1, den: 1}
	}
}

#[cfg(test)]
mod test {
	use super::reduce;
	use super::gcd;
	use super::lcm;

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
	fn check_lcm()	{
		assert_eq!(lcm(21,6), 42);
	}
}
*/
