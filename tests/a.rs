#[macro_use]
extern crate rgetset;

#[test]
fn a_1() {
    #[derive(RGetter, Default, Clone, Debug)]
    pub struct ABC {
        pub id: i64,
        #[rgetter(skip)]
        pub name: Option<String>,
    }
    let a = ABC {
        id: 1,
        name: Some("name_abc".to_string()),
    };
    println!("-----------{a:?}-----------");
    println!("-----------{a:?}-----id:{}------", a.id());
}

#[test]
fn a_2() {
    #[derive(RSetter, Default, Clone, Debug)]
    pub struct ABC {
        pub id: i64,
        pub name: Option<String>,
    }

    let mut a = ABC {
        id: 1,
        name: Some("name_abc".to_string()),
    };

    let _ = a.set_id(5);
    let _ = a.set_name("new value".to_string());

    println!("----------- {a:#?} -----------");
}

#[test]
fn a_builder_1() {
    // #[derive(RGetter, RSetter, Default, Clone, Debug)]
    // pub struct ABC {
    //     pub id: i64,
    //     pub name: Option<String>,
    // }
    //
    // let a = ABC {
    //     id: 1,
    //     name: Some("name_abc".to_string()),
    // };
    // println!("-----------{a:?}-----------");
    //
    // let mut r = ABC::builder().id(5).name("good name".to_string()).build();
    // r.set_name("good name".to_string());
    // //

    // println!("-----------row: {r:#?}-----------");
    // trace();
}

#[test]
fn a_builder_gen_1() {
    #[derive(RBuilder, RGetter, RSetter, Default, Clone, Debug)]
    #[allow(dead_code)]
    pub struct ABC<T>
    where
        T: Default + Clone,
    {
        pub id: i64,
        pub name: Option<String>,
        pub item: T,
        pub list: Option<T>,
    }

    let r: ABC<i64> = ABC::builder()
        .id(5)
        .name("good name".to_string())
        .item(5)
        .list(10000)
        .build();
    println!("-----------row: {r:#?}-----------");
    println!("-----------row: {r:#?}-----------");
    ABCBuilder::<i64>::attrs();
}
