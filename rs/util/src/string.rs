use super::error::E;
use super::UnsafeFrom;
use std::iter::Iterator;
use std::string::ToString;

pub trait SmartString {
    fn ssplit_once<'a, P>(&'a self, delimiter: P) -> (&'a str, &'a str)
    where
        P: std::str::pattern::Pattern + ToString + Clone;

    fn before<'a, P>(&'a self, delimiter: P) -> &'a str
    where
        P: std::str::pattern::Pattern + ToString + Clone,
    {
        self.ssplit_once(delimiter).0
    }

    fn after<'a, P>(&'a self, delimiter: P) -> &'a str
    where
        P: std::str::pattern::Pattern + ToString + Clone,
    {
        self.ssplit_once(delimiter).1
    }

    fn split_whitespace_parse<'a, 'func, T>(&'func self) -> impl Iterator<Item = T>
    where
        // Self: Sized,
        T: UnsafeFrom<&'a str>,
        'func: 'a;

    fn remove_whitespace<'a>(&'a self) -> String;

    fn rev(&self) -> String;

    fn indices<'a, P>(&'a self, pattern: P) -> impl Iterator<Item = usize>
    where
        P: std::str::pattern::Pattern + ToString + Clone;
}

impl<S> SmartString for S
where
    S: AsRef<str> + ?Sized,
{
    fn ssplit_once<'a, P>(&'a self, delimiter: P) -> (&'a str, &'a str)
    where
        P: std::str::pattern::Pattern + ToString + Clone,
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

    fn rev(&self) -> String {
        self.as_ref().chars().rev().collect::<String>()
    }

    fn indices<'a, P>(&'a self, pattern: P) -> impl Iterator<Item = usize>
        where
            P: std::str::pattern::Pattern + ToString + Clone {
        self.as_ref().match_indices(pattern).map(|(idx, _)| idx)
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
