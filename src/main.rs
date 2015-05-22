mod primitive;
mod element;
use element::{Element,NamedFrom};

extern crate chrono;
extern crate url;

fn main() {

	let e1 = Element::from_name_val("foo", false);
	let e2 = Element::from_name_val("bar", false);
	let e3 = Element::from_name_val("baz",23i32);
	let e_second = Element::from_name_val("second", vec![e3]);
	let e_top = Element::from_name_val("top", vec![e1,e2,e_second]);

    println!("{}",e_top.to_string());

}
