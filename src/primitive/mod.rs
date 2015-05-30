use std::fmt;
use url::{Url};
use std::convert::{From};
use chrono::{DateTime,FixedOffset};
use rustc_serialize::json::{ToJson, Json};

mod decimal;
use primitive::decimal::{Dec};
mod time;
use primitive::time::{Time};
mod vardate;
use primitive::vardate::{VarDate};


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
macro_rules! gen_from {
	($t:ty, $prim:ident) => {
		impl From<$t> for Primitive {
			fn from(v: $t) -> Self {
				Primitive::$prim(v)
			}
		}
	}
}

gen_from!(bool,Boolean);
gen_from!(i32,Int);
gen_from!(u32,UInt);
gen_from!(Dec,Decimal);
gen_from!(String,String);
gen_from!(Time,Time);
gen_from!(Url,Uri);
gen_from!(DateTime<FixedOffset>,Instant);

impl<'a> From<&'a str> for Primitive {
	fn from(s: &'a str) -> Self {
		Primitive::String(s.to_string())
	}
}

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
	 		Primitive::Instant(ref x) => x.to_rfc3339(),
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
	 		Primitive::Instant(ref v) => Json::String(v.to_rfc3339()),
	 		Primitive::Date(ref v) => Json::String(v.to_string()),
	 		Primitive::DateTime(ref v) => Json::String(v.to_string()),
	 		Primitive::Time(ref v) => Json::String(v.to_string()),
	 	}
	}
}

#[test]
fn test_bool() {
	let p = Primitive::from(true);
	assert_eq!("true",p.to_string());
	assert_eq!(Json::Boolean(true),p.to_json());
}

#[test]
fn test_i32() {
	let p = Primitive::from(5);
	assert_eq!("5",p.to_string());
	assert_eq!(Json::I64(5),p.to_json());
}

#[test]
fn test_u32() {
	let p = Primitive::from(5u32);
	assert_eq!("5",p.to_string());
	assert_eq!(Json::U64(5),p.to_json());
}

#[test]
fn test_positive() {
	let p = Primitive::PInt(5);
	assert_eq!("5",p.to_string());
	assert_eq!(Json::U64(5),p.to_json());
}


#[test]
fn test_decimal() {
	let d = Dec::from_str("3.14").ok().unwrap();
	let p = Primitive::from(d);
	assert_eq!("3.14",p.to_string());
	assert_eq!(Json::F64(3.14),p.to_json());
}

#[test]
fn test_string() {
	let p = Primitive::from("hello, world");
	assert_eq!("hello, world",p.to_string());
	assert_eq!(Json::String("hello, world".to_string()),p.to_json());
}

#[test]
fn test_url() {
	let u = Url::parse("http://example.org/foo").ok().unwrap();
	let p = Primitive::from(u);
	assert_eq!("http://example.org/foo", p.to_string());
	assert_eq!(Json::String("http://example.org/foo".to_string()),p.to_json());
}

#[test]
fn test_urn() {
	let u = Url::parse("urn:foo:bar").ok().unwrap();
	let p = Primitive::Uri(u);
	assert_eq!("urn:foo:bar", p.to_string());
	assert_eq!(Json::String("urn:foo:bar".to_string()),p.to_json());
}

#[test]
fn test_oid() {
	let u = Url::parse("urn:oid:0.0.0.0").ok().unwrap();
	let p = Primitive::Oid(u);
	assert_eq!("urn:oid:0.0.0.0", p.to_string());
	assert_eq!(Json::String("urn:oid:0.0.0.0".to_string()),p.to_json());
}

#[test]
fn test_id() {
	let p = Primitive::Id("hello, world".to_string());
	assert_eq!("hello, world",p.to_string());
	assert_eq!(Json::String("hello, world".to_string()),p.to_json());
}

#[test]
fn test_base64() {
	let p = Primitive::Base64("hello, world".to_string()); //TODO: Actual base64
	assert_eq!("hello, world",p.to_string());
	assert_eq!(Json::String("hello, world".to_string()),p.to_json());
}

#[test]
fn test_instant() {
	let dt: DateTime<FixedOffset> = "2015-05-02T05:34:00-07:00".parse().ok().unwrap();
	let p = Primitive::from(dt);
	assert_eq!("2015-05-02T05:34:00-07:00",p.to_string());
	assert_eq!(Json::String("2015-05-02T05:34:00-07:00".to_string()),p.to_json());
}	

#[test]
fn test_date() {
	let dt: VarDate = VarDate::parse("2015").unwrap();
	let p = Primitive::Date(dt);
	assert_eq!("2015",p.to_string());
	assert_eq!(Json::String("2015".to_string()),p.to_json());
}

#[test]
fn test_datetime() {
	let dt: VarDate = VarDate::parse("2015").unwrap();
	let p = Primitive::DateTime(dt);
	assert_eq!("2015",p.to_string());
	assert_eq!(Json::String("2015".to_string()),p.to_json());
}	