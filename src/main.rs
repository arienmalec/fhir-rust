extern crate chrono;
extern crate url;
extern crate rustc_serialize;
mod primitive;
mod element;

use element::{Element,NamedFrom};
use rustc_serialize::json::{Json, ToJson};


fn main() {

	let e1 = Element::from_name_val("foo", false);
	let e2 = Element::from_name_val("bar", false);
	let e3 = Element::from_name_val("baz",23i32);
	let e_second = Element::from_name_val("second", vec![e3]);
	let e_top = Element::from_name_val("top", vec![e1,e2,e_second]);
	let j = e_top.to_json();

    println!("{}",j.to_string());

}
