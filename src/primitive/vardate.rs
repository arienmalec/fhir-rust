use std::fmt;
use chrono::{DateTime,FixedOffset};
use chrono::format::{Item,Fixed,Parsed,ParseError,self};



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

