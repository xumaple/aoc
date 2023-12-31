use super::error::E;
use super::UnsafeFrom;
use std::iter::Iterator;
use std::string::ToString;

pub trait SmartString {
    fn ssplit_once<'a, P>(&'a self, delimiter: P) -> (&'a str, &'a str)
    where
        P: std::str::pattern::Pattern<'a> + ToString + Clone;

    fn split_whitespace_parse<'a, 'func, T>(&'func self) -> impl Iterator<Item = T>
    where
        // Self: Sized,
        T: UnsafeFrom<&'a str>,
        'func: 'a;

    fn remove_whitespace<'a>(&'a self) -> String;
}

impl<S> SmartString for S
where
    S: AsRef<str> + ?Sized,
{
    fn ssplit_once<'a, P>(&'a self, delimiter: P) -> (&'a str, &'a str)
    where
        P: std::str::pattern::Pattern<'a> + ToString + Clone,
    {
        self.as_ref()
            .split_once(delimiter.clone())
            .ok_or(E::SplitError(delimiter.to_string()))
            .unwrap()
    }

    fn split_whitespace_parse<'a, 'func, T>(&'func self) -> impl Iterator<Item = T>
    where
        // Self: Sized,
        T: UnsafeFrom<&'a str>,
        'func: 'a,
    {
        self.as_ref().trim().split_whitespace().map(T::ufrom)
    }

    fn remove_whitespace<'a>(&'a self) -> String {
        self.as_ref().replace(" ", "")
    }
}

#[cfg(test)]
mod smart_string_tests {
    use super::*;

    #[test]
    fn ssplit_once_ok() {
        assert_eq!("abcd".ssplit_once("bc"), ("a", "d"));
    }

    #[test]
    #[should_panic]
    fn ssplit_once_err() {
        let _ = "abcd".ssplit_once(":");
    }
}
