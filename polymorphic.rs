// Полиморфизм ООП Rust

fn main() {
	
	let st1 = Star { name: "Sun".to_string(), coord: Point { x: 10.0, y: 20.0 }, color: 1 };
	let st2 = Star { name: "Antares".to_string(), coord: Point { x: 60.0, y: 0.0 }, color: 6 };
	let pl1 = Planet { name: "Mars".to_string(), coord: Point { x: 12.0, y: 17.0 }, population: 123_000_000_000 };
	
	// список объектов. В списке могут быть как звезды, так и планеты
	let objs : Vec<CosmoObject> = vec![
		CosmoObject::Star(st1),
		CosmoObject::Planet(pl1),
		CosmoObject::Star(st2),
	];
	
	println!("\n\nVariant # 1\n");

	for obj in objs {
		match obj {
			CosmoObject::Star(o) => o.print(),   // работает, но как сюда прикрутить trait Printable,
			CosmoObject::Planet(o) => o.print(), // чтобы не вызывать .print() у каждого типа?
		}
	}
	
	// Вариант №2 - с динамическими объектами dyn Printable. Думаю, он сильно медленнее 1го варианта.
	// Так что лучше не полениться и написать хоть 100 одинаковых строк CosmoObject::Xxxxx(o) => o.print() в примере выше

	// не придумал способа, как использовать объекты из 1го набора
	let st1 = Star { name: "Vega".to_string(), coord: Point { x: 0.55, y: 220.0 }, color: 4 };
	let st2 = Star { name: "Sirius".to_string(), coord: Point { x: 15.0, y: 50.0 }, color: 2 };
	let pl1 = Planet { name: "Xoxoxo".to_string(), coord: Point { x: 7.0, y: 7.0 }, population: 900_000_000 };
	
	let o_dyn: Vec<Box<dyn Printable>> = vec![
		Box::new(st1),
		Box::new(pl1),
		Box::new(st2),
	];

	println!("\n\nVariant # 2\n");
	
	for obj in o_dyn {
		obj.print();
	}
}



// космические объекты: звезды и планеты. 
// Перечисление CosmoObject служит для возможности хранить звезды и планеты в одном списке Vec<CosmoObject>
struct Star {
	name: String,
	coord: Point,
	color: i32,
}

struct Planet {
	name: String,
	coord: Point,
	population: i64,
}

enum CosmoObject {
	Star (Star),
	Planet (Planet),
}



// объекты, которые можно вывести на экран. 
// Трейт Printable позволяет хранить звезды и планеты в одном списке Vec<Box<dyn Printable>>
trait Printable {
	fn print(&self);
}

impl Printable for Star {
	fn print(&self) {
		println!("* [{} -> ({}, {}) *{}]", self.name, self.coord.x, self.coord.y, self.color);
	}
}

impl Printable for Planet {
	fn print(&self) {
		println!("o [{} -> ({}, {}) pop: {} mlrd]", self.name, self.coord.x, self.coord.y, self.population / 1_000_000_000);
	}
}



// Вспомогательные типы данных
struct Point { x: f32, y: f32, }
