use serde::{Serialize, Deserialize};

trait MyTrait {
    fn foo(&self);
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct A {
    name: String,
    tr: dyn MyTrait,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct MyStruct {
    value: i32,
}

impl MyTrait for MyStruct {
    fn foo(&self) {
        println!("bar");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> std::result::Result<(), Box<dyn Error>> {
        let a = A { 
            name: "hello".to_string(), 
            tr: MyStruct { value: 1,}
        };

        let buf = a.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    }
}
