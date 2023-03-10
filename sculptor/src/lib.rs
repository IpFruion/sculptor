use err::Error;
use field_type::FieldType;
use modifier::Modifier;

pub mod err;
pub mod field_type;
pub mod modifier;
pub mod rust;
pub mod ts;

pub trait Sculptable {
    type Input;
    fn sculpt<S: Sculptor>(sculptor: &mut S, input: Self::Input) -> Result<S::Ok, S::Error>;
}

pub trait StructSculptable {
    fn sculpt_struct<S: Sculptor>(sculptor: &mut S) -> Result<S::Ok, S::Error>;
}

impl<T: Sculptable<Input = ()>> StructSculptable for T {
    fn sculpt_struct<S: Sculptor>(sculptor: &mut S) -> Result<S::Ok, S::Error> {
        T::sculpt(sculptor, ())
    }
}

pub trait SelfSculptable {
    fn sculpt_self<S: Sculptor>(self, sculptor: &mut S) -> Result<S::Ok, S::Error>;
}

impl<T: Sculptable<Input = Self>> SelfSculptable for T {
    fn sculpt_self<S: Sculptor>(self, sculptor: &mut S) -> Result<S::Ok, S::Error> {
        T::sculpt(sculptor, self)
    }
}

/// Interface for different generated language model schemas.
pub trait Sculptor {
    type Ok: Default;
    type Error: Error;

    fn start(&mut self, modifier: Modifier, name: &str) -> Result<Self::Ok, Self::Error>;
    fn field<'a>(
        &mut self,
        modifier: Modifier,
        name: &str,
        field_type: FieldType<'a>,
    ) -> Result<Self::Ok, Self::Error>;
    fn end(&mut self) -> Result<Self::Ok, Self::Error>;
}

#[cfg(any(test, feature = "mocks"))]
pub mod mocks {
    use crate::{err::SculptureError, field_type::FieldType, modifier::Modifier, Sculptor};

    #[derive(Default)]
    pub struct MockSculptor {
        pub starts: Vec<(Modifier, String)>,
        pub fields: Vec<(Modifier, String, String)>,
        pub ends: usize,
    }

    impl Sculptor for MockSculptor {
        type Ok = ();

        type Error = SculptureError;

        fn start(&mut self, modifier: Modifier, name: &str) -> Result<Self::Ok, Self::Error> {
            self.starts.push((modifier, name.to_owned()));
            Ok(())
        }

        fn field<'a>(
            &mut self,
            modifier: Modifier,
            name: &str,
            field_type: FieldType<'a>,
        ) -> Result<Self::Ok, Self::Error> {
            self.fields
                .push((modifier, name.to_owned(), format!("{:?}", field_type)));
            Ok(())
        }

        fn end(&mut self) -> Result<Self::Ok, Self::Error> {
            self.ends += 1;
            Ok(())
        }
    }
}
