use rgetter::RGet;

fn main() {
    #[derive(RGetter, Default, Clone, Debug)]
    pub struct ABC<T: Clone> {
        pub i: i64,
        pub id: Option<i64>,
        pub name: Option<String>,
        pub a: T,
    }

    let src: ABC<i64> = ABC {
        i: 10,
        id: Some(1),
        name: Some("hello".to_string()),
        a: 10,
    };

    ABC::<i64>::trace();
    println!("-----------name:{:#?}-----------", src.name());
    println!("-----------i: {:#?}-----------", src.i());
    println!("-----------id: {:#?}-----------", src.id());
    println!("-----------end-----------");
    #[derive(RGetter, Default, Clone, Debug)]
    pub struct AB {
        pub i: i64,
        pub id: Option<i64>,
        pub name: Option<String>,
    }

    let src: AB = AB {
        i: 10,
        id: Some(1),
        name: Some("hello".to_string()),
    };

    AB::trace();
    println!("-----------name:{:#?}-----------", src.name());
    println!("-----------i: {:#?}-----------", src.i());
    println!("-----------id: {:#?}-----------", src.id());
    println!("-----------end-----------");
}
