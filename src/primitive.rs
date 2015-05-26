use std::fmt;
use chrono::{DateTime,FixedOffset};
use chrono::format::{Item,Fixed,Parsed,ParseError,self};
use url::{Url};
use std::convert::{From};
use rustc_serialize::json::{self, ToJson, Json};

#[derive(Debug)]
pub struct Dec {
	val: f64,
	precision: usize 
}


impl Dec {
	fn find_precision(s: &str) -> usize {
		s.rfind('.').map(|i| s.len() - i - 1 ).unwrap_or(0)
	}

	fn from_str(s: &str) -> Result<Self,()> {
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

	fn _from_parsed_result(r: Result<(),ParseError>, p: Parsed) -> Result<Self,ParseError> {
		match r {
        	Ok(_) => match p.to_datetime() {
        		Ok(dt) => Ok(VarDate {y: None, m: None, d: None, dt: Some(dt)}),
        		Err(e) => Err(e)
        	},
        	Err(e) => match (p.year, p.month, p.day) {
        		(None, None, None) => Err(e),
        		_ => Ok(VarDate {y: p.year, m: p.month, d: p.day, dt: None})
        	}
        }
	}

	pub fn parse(s: &str) -> Result<Self,ParseError> {
        const ITEMS: &'static [Item<'static>] = &[Item::Fixed(Fixed::RFC3339)];
        let mut parsed = Parsed::new();

        let r = format::parse(&mut parsed, s, ITEMS.iter().cloned());
        VarDate::_from_parsed_result(r, parsed)
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

impl ToJson for Primitive {
	fn to_json(&self) -> Json {
		match *self {
			Primitive::Boolean(v) => Json::Boolean(v),
	 		Primitive::Int(i) => Json::I64(i as i64),
	 		Primitive::UInt(i) => Json::U64(i as u64),
	 		Primitive::PInt(i) => Json::U64(i as u64),
	 		Primitive::Decimal(ref d) => Json::F64(d.val),
	 		Primitive::String(ref s) => Json::String(s.clone()),
	 		Primitive::Id(ref v) => Json::String(v.to_string()),
	 		Primitive::Uri(ref v) => Json::String(v.to_string()),
	 		Primitive::Oid(ref v) => Json::String(v.to_string()),
	 		Primitive::Base64(ref v) => Json::String(v.to_string()),
	 		Primitive::Instant(ref v) => Json::String(v.to_string()),
	 		Primitive::Date(ref v) => Json::String(v.to_string()),
	 		Primitive::DateTime(ref v) => Json::String(v.to_string()),
	 		Primitive::Time(ref v) => Json::String(v.to_string()),
	 	}
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
