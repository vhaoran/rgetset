#[macro_use]
extern crate rgetter;

use std::process::id;

#[test]
fn a_1() {
    #[derive(RGet, Default, Clone, Debug)]
    pub struct ABC {
        pub id: i64,
        pub name: Option<String>,
    }

    pub struct ABCBuilder {
        inner: ABC,
    }

    let a = ABC {
        id: 1,
        name: Some("name_abc".to_string()),
    };

    // let id = a.id();
    // let name = a.name();

    // println!("-----------id: {id} name: {name}-----------");
}

#[test]
fn a_2() {
    #[derive(RSet, Default, Clone, Debug)]
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
    #[derive(RBuilder, Default, Clone, Debug)]
    pub struct ABC {
        pub id: i64,
        pub name: Option<String>,
    }

    let mut a = ABC {
        id: 1,
        name: Some("name_abc".to_string()),
    };

    let r = ABC::builder().id(5).name("good name".to_string()).build();

    println!("-----------row: {r:#?}-----------");
    // trace();
}

#[test]
fn a_builder_gen_1() {
    #[derive(RBuilder, RGet, RSet, Default, Clone, Debug)]
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

    let mut r: ABC<i64> = ABC::default();
    r.set_id(5);
    r.set_name("good name".to_string());
    r.set_item(5);
    r.set_list(10000);

    println!("-----------row: {r:#?}-----------");
}

#[test]
fn a_builder_pat() {
    #[derive(Default, Clone, Debug)]
    pub struct ABC {
        pub id: i64,
        pub name: Option<String>,
    }

    struct ABCBuilder {
        inner: ABC,
    }
    impl ABCBuilder {
        pub fn set_id(&mut self, v: i64) -> &mut Self {
            self.inner.id = v;
            self
        }
        pub fn set_name(&mut self, v: String) -> &mut Self {
            self.inner.name = Some(v);
            self
        }
    }
    impl ABC {
        pub fn builder() -> ABCBuilder {
            ABCBuilder {
                inner: ABC::default(),
            }
        }
    }
}

#[test]
fn a_gen_1() {
    #[derive(RGet, Default, Clone, Debug)]
    pub struct ABC<T>
    where
        T: Clone + Default,
    {
        pub id: i64,
        pub name: Option<String>,
        pub item: T,
    }

    impl<T> ABC<T>
    where
        T: Clone + Default,
    {
        pub fn builder() -> ABCBuilder<T> {
            ABCBuilder {
                inner: ABC::default(),
            }
        }
    }

    pub struct ABCBuilder<T>
    where
        T: Clone + Default,
    {
        inner: ABC<T>,
    }

    impl<T> ABCBuilder<T>
    where
        T: Clone + Default,
    {
        pub fn id(&mut self, v: i64) -> &mut Self {
            self.inner.id = v;
            self
        }
    }

    let b: &ABCBuilder<i64> = ABC::builder().id(5);

    let a = ABC::<i64> {
        id: 1,
        name: Some("name_abc".to_string()),
        item: 1,
    };

    // let id = a.id();
    // let name = a.name();

    // println!("-----------id: {id} name: {name}-----------");
}
