extern crate attr;

use attr::Attr;

pub struct Foo {
    bar: String,
    batz: i32
}

pub struct Bar {
    batz: String,
}

#[test]
fn simple_access() {
    #[derive(Default)]
    struct FooAttributeBar;
    #[derive(Default)]
    struct FooAttributeBatz;

    trait Attributes<AttributeType> {
        fn attrs() -> AttributeType;
    }

    #[derive(Default)]
    struct FooAttributes {
        bar: FooAttributeBar,
        batz: FooAttributeBatz
    }

    impl Attributes<FooAttributes> for Foo {
        fn attrs() -> FooAttributes {
            FooAttributes::default()
        }
    }

    impl<'a> Attr<&'a Foo> for FooAttributeBar {
        type Output = &'a str;

        fn get(&self, i: &'a Foo) -> &'a str {
            i.bar.as_ref()
        }

        fn name(&self) -> &'static str {
            "bar"
        }
    }

    impl<'a> Attr<&'a Foo> for FooAttributeBatz {
        type Output = i32;

        fn get(&self, i: &'a Foo) -> i32 {
            i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    #[derive(Default)]
    struct BarAttributeBatz;

    #[derive(Default)]
    struct BarAttributes {
        batz: BarAttributeBatz,
    }

    impl Attributes<BarAttributes> for Bar {
        fn attrs() -> BarAttributes {
            BarAttributes::default()
        }
    }

    impl<'a> Attr<&'a Bar> for BarAttributeBatz {
        type Output = &'a str;

        fn get(&self, i: &'a Bar) -> Self::Output {
            i.batz.as_ref()
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    impl<'a> Attr<&'a mut Bar> for BarAttributeBatz {
        type Output = &'a mut String;

        fn get(&self, i: &'a mut Bar) -> Self::Output {
             &mut i.batz
        }

        fn name(&self) -> &'static str {
            "batz"
        }
    }

    let f = Foo { bar: "foobar".into(), batz: 20 };
    Foo::attrs().bar.get(&f);
    Foo::attrs().batz.get(&f);

    let f = Bar { batz: "foobar".into() };
    Bar::attrs().batz.get(&f);

    let mut f = Bar { batz: "foobar".into() };

    let mut batz = Bar::attrs().batz.get(&mut f);
    batz.push('b');
}
