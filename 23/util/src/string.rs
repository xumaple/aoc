use super::error::E;
use std::string::ToString;

pub trait SmartString {
    fn ssplit_once<'a, P>(&'a self, delimiter: P) -> (&'a str, &'a str)
    where
        P: std::str::pattern::Pattern<'a> + ToString + Clone;
}

impl<S> SmartString for S
where
    S: AsRef<str>,
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
