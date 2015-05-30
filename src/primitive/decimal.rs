use std::fmt;

#[derive(Debug)]
pub struct Dec {
	pub val: f64,
	pub precision: usize 
}


impl Dec {
	fn find_precision(s: &str) -> usize {
		s.rfind('.').map(|i| s.len() - i - 1 ).unwrap_or(0)
	}

	pub fn from_str(s: &str) -> Result<Self,()> {
		let f :f64  = match s.parse() {
			Ok(f) => f,
			_ => return Err(()) // can't handle error due to Bug #24748
		};
		Ok(Dec {
			val: f,
			precision: Dec::find_precision(s)
		})
	}
}

impl fmt::Display for Dec {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{:.*}",self.precision,self.val)
	}
}

#[test]
fn test_decimal_from_string() {
	let d = Dec::from_str("3.14").ok().unwrap();
	assert_eq!(d.val, 3.14f64);
	assert_eq!(d.precision, 2);
}

#[test]
fn test_decimal_representation() {
	let d = Dec::from_str("0.1").ok().unwrap(); //force rounding
	assert_eq!("0.1", d.to_string());
	assert_eq!(0.1f64, d.val);
	assert_eq!(1, d.precision);
}

#[test]
fn test_decimal_representation_with_precision() {
	let d = Dec::from_str("0.10").ok().unwrap(); //force rounding
	assert_eq!("0.10", d.to_string());
	assert_eq!(0.1f64, d.val);
	assert_eq!(2, d.precision);
}

#[test]
fn test_long_zero_decimal_from_string() {
	let d = Dec::from_str("0.0000000").ok().unwrap();
	assert_eq!(0f64, d.val);
	assert_eq!("0.0000000", d.to_string());
	assert_eq!(7, d.precision);
}


#[test]
fn test_long_decimal_from_string() {
	let d = Dec::from_str("3.1415926").ok().unwrap();
	assert_eq!(3.1415926f64, d.val);
	assert_eq!(7, d.precision);
}

#[test]
fn test_integer_with_point_from_string() {
	let d = Dec::from_str("3.").ok().unwrap();
	assert_eq!(3f64, d.val);
	assert_eq!("3", d.to_string());
	assert_eq!(0, d.precision);
}

#[test]
fn test_integer_from_string() {
	let d = Dec::from_str("3").ok().unwrap();
	assert_eq!(3f64, d.val);
	assert_eq!("3", d.to_string());
	assert_eq!(0, d.precision);
}

#[test]
fn test_invalid_decimal_from_string() {
	let d = Dec::from_str("pi");
	assert!(d.is_err());
}


#[test]
fn test_invalid_numberlike_decimal_from_string() {
	let d = Dec::from_str("3.141.23");
	assert!(d.is_err());
}
