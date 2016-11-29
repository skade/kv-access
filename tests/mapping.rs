extern crate attr;

use attr::retrieve;
use attr::Traverse;

#[derive(Debug)]
pub struct Foo {
    bar: String,
    vector: Vec<Bla>
}

#[derive(Debug)]
pub struct Bla {
    name: String
}

#[derive(Debug)]
pub struct Top {
    foo: Foo
}


pub mod foo {
    use attr::Attr;
    use attr::IndexableAttr;
    use attr::IterableAttr;
    use attr::Attributes;

    use super::Foo;
    use super::Bla;

    #[derive(Default)]
    pub struct Bar;
    #[derive(Default)]
    pub struct Vector;

    #[derive(Default)]
    pub struct FooAttributes {
        pub bar: Bar,
        pub numbers: Vector
    }

    impl Attributes<FooAttributes> for Foo {
        fn attrs() -> FooAttributes {
            FooAttributes::default()
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b Foo> for Bar {
        type Output = &'a str;

        fn get(&self, i: &'b Foo) -> Self::Output {
            i.bar.as_ref()
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b Foo> for Vector {
        type Output = &'a [Bla];

        fn get(&self, i: &'b Foo) -> &'a [Bla] {
            i.vector.as_ref()
        }

        fn name(&self) -> &'static str {
            "vector"
        }
    }

    impl<'a, 'b : 'a> IndexableAttr<'a, 'b, &'b Foo, usize> for Vector {
        type Output = &'a Bla;

        fn at(&self, i: &'b Foo, idx: usize) -> &'a Bla {
            unsafe { self.get(i).get_unchecked(idx) }
        }
    }

    impl<'a, 'b: 'a> IterableAttr<'a, 'b, &'b Foo> for Vector {
        type Item = &'a Bla;

        fn iter(&self, i: &'b Foo) -> Box<Iterator<Item=&'a Bla> + 'a> {
            Box::new(self.get(i).iter())
        }
    }
}


pub mod bla {
    use attr::Attr;
    use attr::Attributes;

    use super::Bla;

    #[derive(Default)]
    pub struct Name;

    #[derive(Default)]
    pub struct BlaAttributes {
        pub name: Name,
    }

    impl Attributes<BlaAttributes> for Bla {
        fn attrs() -> BlaAttributes {
            BlaAttributes::default()
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b Bla> for Name {
        type Output = &'a str;

        fn get(&self, i: &'b Bla) -> &'a str {
            i.name.as_ref()
        }

        fn name(&self) -> &'static str {
            "name"
        }
    }
}

pub mod top {
    use attr::Attr;
    use attr::Attributes;

    use super::Top;
    use super::Foo;

    #[derive(Default)]
    pub struct FooField;

    #[derive(Default)]
    pub struct TopAttributes {
        pub foo: FooField,
    }

    impl Attributes<TopAttributes> for Top {
        fn attrs() -> TopAttributes {
            TopAttributes::default()
        }
    }

    impl<'a, 'b: 'a> Attr<'a, 'b, &'b Top> for FooField {
        type Output = &'a Foo;

        fn get(&self, i: &'b Top) -> &'a Foo {
            &i.foo
        }

        fn name(&self) -> &'static str {
            "foo"
        }
    }
}

#[test]
fn test_access() {
    let b1 = Bla { name: "foo".into() };

    let path = retrieve(bla::Name);

    assert_eq!(path.traverse(&b1), "foo");
}

#[test]
fn test_mapped() {
    let b1 = Bla { name: "foo".into() };
    let b2 = Bla { name: "bla".into() };

    let foo = Foo { bar: "bar".into(), vector: vec![b1,b2] };
    let path = retrieve(bla::Name).mapped(foo::Vector);

    let result = path.traverse(&foo).collect::<Vec<_>>();
    assert_eq!(result, vec!["foo", "bla"]);
}

#[test]
fn test_complex_mapped() {
    let b1 = Bla { name: "foo".into() };
    let b2 = Bla { name: "bla".into() };

    let foo = Foo { bar: "bar".into(), vector: vec![b1,b2] };
    let top = Top { foo: foo };

    let path = retrieve(bla::Name).mapped(foo::Vector).from(top::FooField);

    let result = path.traverse(&top).collect::<Vec<_>>();
    assert_eq!(result, vec!["foo", "bla"]);
}
