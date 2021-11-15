use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::borrow::Borrow;

#[doc(hidden)]
pub fn deserialize_const<'de, D, T, S>(deserializer: D, expected_val: &T) -> Result<S, D::Error>
where
    S: Default,
    D: Deserializer<'de>,
    T: ?Sized + ToOwned + ToString,
    T::Owned: Deserialize<'de> + PartialEq<T>,
{
    let serialized_owned_val = T::Owned::deserialize(deserializer)?;
    if serialized_owned_val == *expected_val {
        Ok(S::default())
    } else {
        let unexpected = serialized_owned_val.borrow().to_string();
        let expected = expected_val.to_string();
        Err(D::Error::invalid_value(
            ::serde::de::Unexpected::Other(unexpected.as_str()),
            &expected.as_str(),
        ))
    }
}

#[macro_export]
macro_rules! impl_const {
    ($type_name: ident, $const_type: ty, $const_val: literal) => {
        #[derive(
            ::std::cmp::Eq, ::std::cmp::PartialEq, ::std::default::Default, ::std::hash::Hash,
        )]
        pub struct $type_name(());

        impl ::std::fmt::Debug for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(::std::any::type_name::<Self>())
                    .field(&Self::value())
                    .finish()
            }
        }

        impl ::std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&Self::value(), f)
            }
        }

        impl ::serde::Serialize for $type_name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                Self::value().serialize(serializer)
            }
        }
        impl<'de> ::serde::Deserialize<'de> for $type_name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                use ::std::borrow::Borrow;
                $crate::constant::deserialize_const(deserializer, Self::value().borrow())
            }
        }
        impl $type_name {
            pub const fn value() -> $const_type {
                $const_val
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::impl_const;
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

    impl_const!(StrHello, &'static str, "hello");
    impl_const!(Answer, i64, 42);
    impl_const!(False, bool, false);

    #[test]
    fn test_str() {
        assert_tokens(&StrHello::default(), &[Token::String("hello")]);
        assert_de_tokens_error::<StrHello>(
            &[Token::String("wrong string")],
            "invalid value: wrong string, expected hello",
        );
    }
    #[test]
    fn test_number() {
        assert_tokens(&Answer::default(), &[Token::I64(42)]);
        assert_de_tokens_error::<Answer>(&[Token::I64(43)], "invalid value: 43, expected 42");
    }
    #[test]
    fn test_value() {
        const STR_VAL: &str = StrHello::value();
        assert_eq!(STR_VAL, "hello");
        assert_eq!(Answer::value(), 42_i64);
        assert_eq!(False::value(), false);
    }
}
