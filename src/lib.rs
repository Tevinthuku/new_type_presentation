mod benefits;
pub mod advanced_techniques;
pub mod limitations;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU8;

    use url::Url;

    use super::*;

    struct RustTalkLink(Url);

    #[test]
    fn it_works() {
        let mut s = String::from("value");
        s.push('c');
        let new_type = RustTalkLink(Url::parse("input").unwrap());
        let n = NonZeroU8::new(2);
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
