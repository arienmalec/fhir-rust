use std::fmt;
use std::convert::{From};
use primitive::Primitive;


pub enum ElementType {
	Primitive(Primitive),
	Element(Vec<Element>)
}

impl From<bool> for ElementType {
	fn from(b:bool) -> Self {
		ElementType::Primitive(Primitive::from(b))
	}
}

impl From<i32> for ElementType {
	fn from(v: i32) -> Self {
		ElementType::Primitive(Primitive::Integer(v))
	}
}


pub struct Element {
	pub name: String,
	pub data: ElementType
	//pub id: Option<String>,
	//pub extensions: Option<Vec<Extension>>
}

trait NamedFrom<T> {
	fn from_name_val(name: &str, val: T) -> Self;
}

impl NamedFrom<bool> for Element {
	fn from_name_val(name: &str, val: bool) -> Self {
		Element {
			name: name.to_string(),
			data: ElementType::from(val)
		}
	}
}

impl NamedFrom<i32> for Element {
	fn from_name_val(name: &str, val: i32) -> Self {
		Element {
			name: name.to_string(),
			data: ElementType::from(val)
		}
	}
}

impl Element {
	pub fn to_string(&self) -> String {
		self.recursive_to_string(0)
	}

	fn recursive_to_string(&self,level: u32) -> String {
		let spaces : String = (0..level).map(|_| " ").collect::<Vec<&str>>().concat();

		let label = format!("{}: ",self.name);

		let value = match self.data {
 			ElementType::Primitive(ref v) => format!("{}",v),
 			ElementType::Element(ref v) => format!("\n{}",recursive_elt_vec_to_string(v, level + 1))			
		};
		format!("{}{}{}", spaces, label, value)
	}
}

fn recursive_elt_vec_to_string (v: &Vec<Element>, level: u32) -> String {
	let l: Vec<String> = v.iter().map(|e| e.recursive_to_string(level)).collect();
	l.connect("\n")
}

impl fmt::Display for Element {
 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
 		write!(f,"{}",self.to_string())
 	}
}

#[test]
fn test_simple_primitive_elt() {
	let e = Element {
		name: "foo".to_string(),
		data: ElementType::from(false)
	};
	assert_eq!("foo: false", format!("{}", e));
}

fn make_test_elt() -> Element {
	let e1 = Element::from_name_val("foo",false);
	let e2 = Element::from_name_val("bar",false);
	let e3 = Element::from_name_val("baz",23i32);
	let e_second = Element {
		name: "second".to_string(),
		data: ElementType::Element(vec![e3])
	};
	let e_top = Element {
		name: "top".to_string(),
		data: ElementType::Element(vec![e1,e2,e_second])
	};
	e_top

}

#[test]
fn test_compound_elt() {

	let expected = "top: 
 foo: false
 bar: false
 second: 
  baz: 23";
  assert_eq!(expected, make_test_elt().to_string());

}