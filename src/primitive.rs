use chrono::{DateTime,FixedOffset};
use chrono::format::{Item,Fixed,Parsed,ParseError,self};
use url::{Url};
use std::fmt;
use std::convert::{From};


#[derive(Debug)]
pub struct Dec {
	raw: String,
	val: Option<f64>,
	precision: Option<usize> 
}


impl Dec {
	fn find_precision(s: &str) -> Option<usize> {
		match s.rfind('.') {
			Some(i) => Some((s.len() - i - 1 )),
			None => Some(0)
		}
	}

	fn from_str(s: &str) -> Self {
		let v = s.parse().ok();
		let p = v.and_then(|_| Dec::find_precision(s));
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
pub struct Time {
	h: u8,
	m: u8,
	s: Option<f64>
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
		let s_msg = (self.s).map_or(String::from(""),|s| format!("{}",s));
		write!(f,"{}:{}{}",self.h,self.m,s_msg)
	}
}

#[derive(Debug)]
pub struct VarDate {
	y: Option<i32>,
	m: Option<u32>,
	d: Option<u32>,
	dt: Option<DateTime<FixedOffset>>
}

impl From<DateTime<FixedOffset>> for VarDate {
	fn from(dt: DateTime<FixedOffset>) -> Self {
		VarDate{
			y: None,
			m: None,
			d: None,
			dt: Some(dt)
		}
	}
}

impl VarDate {
	fn from_year(y: i32) -> Self {
		VarDate { y: Some(y), m: None, d: None, dt: None}
	}

	fn parse(s: &str) -> Result<Self,ParseError> {
        const ITEMS: &'static [Item<'static>] = &[Item::Fixed(Fixed::RFC3339)];
        let mut parsed = Parsed::new();

        match format::parse(&mut parsed, s, ITEMS.iter().cloned()) {
        	Ok(_) => match parsed.to_datetime() {
        		Ok(dt) => Ok(VarDate {y: None, m: None, d: None, dt: Some(dt)}),
        		Err(e) => Err(e)
        	},
        	Err(e) => match (parsed.year, parsed.month, parsed.day) {
        		(None, None, None) => Err(e),
        		_ => Ok(VarDate {y: parsed.year, m: parsed.month, d: parsed.day, dt: None})
        	}
        }
	}

}


impl fmt::Display for VarDate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match (self.dt, self.y, self.m, self.d) {
			(Some(dt),_,_,_) => write!(f,"{}",dt.to_rfc3339()),
			(_,Some(y),None,None) => write!(f,"{}",y),
			(_,Some(y),Some(m),None) => write!(f,"{}-{:02}",y,m), // TODO: pad month
			(_,Some(y),Some(m),Some(d)) => write!(f,"{}-{:02}-{:02}",y,m,d), // TODO: pad day
			_ => panic!("Invalid date")
		}
	}
}



#[derive(Debug)]
pub enum Primitive {
	Boolean(bool),
	Int(i32),
	UInt(u32),
	PInt(u32),
	Decimal(Dec),
	String(String),
	Id(String),
	Uri(Url),
	Oid(Url),
	Base64(String),
	Instant(DateTime<FixedOffset>),
	Date(VarDate),
	DateTime(VarDate),
	Time(Time)

}

// create a From defintion for each type for Primitive
macro_rules! primitive_from {
	($t:ty, $prim:ident) => {
		impl From<$t> for Primitive {
			fn from(v: $t) -> Self {
				Primitive::$prim(v)
			}
		}
	}
}

primitive_from!(bool,Boolean);
primitive_from!(i32,Int);
primitive_from!(u32,UInt);
primitive_from!(Dec,Decimal);
primitive_from!(String,String);
primitive_from!(Time,Time);


impl Primitive {
	fn to_string(&self) -> String {
		match *self {
			Primitive::Boolean(v) => format!("{}",v),
	 		Primitive::Int(i) => format!("{}",i),
	 		Primitive::UInt(i) => format!("{}", i),
	 		Primitive::PInt(i) => format!("{}", i),
	 		Primitive::Decimal(ref d) => format!("{}",d),
	 		Primitive::String(ref s) => format!("{}",s),
	 		Primitive::Id(ref s) => format!("{}",s),
	 		Primitive::Uri(ref v) => format!("{}",v),
	 		Primitive::Oid(ref v) => format!("{}",v),
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
fn test_long_zero_decimal_from_string() {
	let d = Dec::from_str("0.0000000");
	assert_eq!(d.val, Some(0f64));
	assert_eq!(&d.raw, "0.0000000");
	assert_eq!("0.0000000", d.to_string());
	assert_eq!(d.precision, Some(7))
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
	assert_eq!("3", d.to_string());
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

#[test]
fn test_parse_y() {
	let d = VarDate::parse("2015").unwrap();
	assert_eq!(d.y, Some(2015));
	assert_eq!((d.m,d.d,d.dt), (None, None, None));
	assert_eq!("2015", d.to_string());
}

#[test]
fn test_parse_ym() {
	let d = VarDate::parse("2015-05").unwrap();
	assert_eq!(d.y, Some(2015));
	assert_eq!(d.m, Some(5));
	assert_eq!((d.d,d.dt), (None, None));
	assert_eq!("2015-05", d.to_string());
}

#[test]
fn test_parse_ymd() {
	let d = VarDate::parse("2015-05-02").unwrap();
	assert_eq!(d.y, Some(2015));
	assert_eq!(d.m, Some(5));
	assert_eq!(d.d, Some(2));
	assert_eq!(d.dt, None);
	assert_eq!("2015-05-02", d.to_string());
}

#[test]
fn test_parse_complete_date() {
	let d = VarDate::parse("2015-05-02T05:34:00-07:00").unwrap();
	assert!(d.dt.is_some());
	assert_eq!("2015-05-02T05:34:00-07:00", d.to_string());
}
