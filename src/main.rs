mod primitive;
mod element;

extern crate chrono;

fn main() {

	let e1 = element::Element {
		name: "foo".to_string(),
		data: element::ElementType::Primitive(primitive::Primitive::Boolean(false))
	};
	let e2 = element::Element {
		name: "bar".to_string(),
		data:element::ElementType::Primitive(primitive::Primitive::Boolean(false))
	};
	let e3 = element::Element {
		name: "baz".to_string(),
		data:element::ElementType::Primitive(primitive::Primitive::Integer(23))
	};
	let e_second = element::Element {
		name: "second".to_string(),
		data: element::ElementType::Element(vec![e3])
	};
	let e_top = element::Element {
		name: "top".to_string(),
		data: element::ElementType::Element(vec![e1,e2,e_second])
	};

    println!("{}",e_top.to_string());
}
