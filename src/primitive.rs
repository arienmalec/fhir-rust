use chrono::{DateTime,Date,UTC};
use std::fmt;
use std::convert::{From};


#[derive(Debug)]
struct Dec {
	raw: String,
	val: Option<f64>,
	precision: Option<usize> 
}

fn find_precision(s: &str) -> Option<usize> {
	match s.rfind('.') {
		Some(i) => Some((s.len() - i - 1 )),
		None => Some(0)
	}
}

impl Dec {
	fn from_str(s: &str) -> Self {
		let v = s.parse().ok();
		let p = v.and_then(|_| find_precision(s));
		Dec {
			raw: s.to_string(),
			val: v,
			precision: p
		}

	}
}

impl fmt::Display for Dec {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.val {
			Some(v) =>  write!(f,"{:.*}",self.precision.unwrap(),v),
			None => f.write_str(&(self.raw))
		}
	}
}

#[derive(Debug)]
struct Time {
	h: u8,
	m: u8,
	s: Option<u8>
}

impl Time {
	pub fn from_hm(h: u8, m: u8) -> Option<Self> {
		match (h,m) {
			(0...23,0...59) => Some(Time {h: h, m: m, s: None}),
			_ => None
		}
	}
}

impl fmt::Display for Time {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s_msg = match self.s {
			Some(s) => format!(":{}",s),
			None => "".to_string()
		};
		write!(f,"{}:{}{}",self.h,self.m,s_msg)
	}
}

#[derive(Debug)]
pub enum Primitive {
	Boolean(bool),
	Integer(i32),
	Decimal(Dec),
	String(String),
	Uri(String),
	Base64(String),
	Instant(DateTime<UTC>),
	Date(Date<UTC>),
	DateTime(DateTime<UTC>),
	Time(Time)

}

impl From<bool> for Primitive {
	fn from(b: bool) -> Self {
		Primitive::Boolean(b)
	}
}

impl Primitive {
	fn to_string(&self) -> String {
		match *self {
			Primitive::Boolean(v) => format!("{}",v),
	 		Primitive::Integer(i) => format!("{}",i),
	 		Primitive::Decimal(ref d) => format!("{}",d),
	 		Primitive::String(ref s) => format!("{}",s),
	 		Primitive::Uri(ref s) => format!("{}",s),
	 		Primitive::Base64(ref s) => format!("{}",s),
	 		Primitive::Instant(ref x) => format!("{}",x),
	 		Primitive::Date(ref x) => format!("{}",x),
	 		Primitive::DateTime(ref x) => format!("{}",x),
	 		Primitive::Time(ref x) => format!("{}",x),
		}
	}
}

impl fmt::Display for Primitive {
	 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	 	write!(f,"{}",self.to_string())
    }

}

#[test]
fn test_decimal_from_string() {
	let d = Dec::from_str("3.14");
	assert_eq!(d.val, Some(3.14f64));
	assert_eq!(&d.raw, "3.14");
	assert_eq!(d.precision, Some(2))
}

#[test]
fn test_decimal_representation() {
	let d = Dec::from_str("0.1"); //force rounding
	assert_eq!("0.1", d.to_string());
	assert_eq!(Some(0.1f64), d.val);
}

#[test]
fn test_decimal_representation_with_precision() {
	let d = Dec::from_str("0.10"); //force rounding
	assert_eq!("0.10", d.to_string());
	assert_eq!(Some(0.1f64), d.val);
}


#[test]
fn test_long_decimal_from_string() {
	let d = Dec::from_str("3.1415926");
	assert_eq!(d.val, Some(3.1415926f64));
	assert_eq!(&d.raw, "3.1415926");
	assert_eq!(d.precision, Some(7))
}

#[test]
fn test_integer_with_point_from_string() {
	let d = Dec::from_str("3.");
	assert_eq!(d.val, Some(3.0f64));
	assert_eq!(&d.raw, "3.");
	assert_eq!(d.precision, Some(0))
}

#[test]
fn test_integer_from_string() {
	let d = Dec::from_str("3");
	assert_eq!(d.val, Some(3.0f64));
	assert_eq!(&d.raw, "3");
	assert_eq!(d.precision, Some(0))
}

#[test]
fn test_invalid_decimal_from_string() {
	let d = Dec::from_str("pi");
	assert_eq!(d.val,  None);
	assert_eq!(&d.raw, "pi");
	assert_eq!(d.precision, None)
}


#[test]
fn test_invalid_numberlike_decimal_from_string() {
	let d = Dec::from_str("3.141.23");
	assert_eq!(d.val, None);
	assert_eq!(&d.raw, "3.141.23");
	assert_eq!(d.precision, None)
}

#[test]
fn test_time_repr() {
	let t = Time::from_hm(12,30).unwrap();
	assert_eq!("12:30",format!("{}",t));
}

#[test]
fn test_time_invalid() {
	let t = Time::from_hm(53,30);
	assert!(t.is_none());
}