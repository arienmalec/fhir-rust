use std::fmt;


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
