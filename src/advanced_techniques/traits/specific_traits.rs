mod national_id {
    use std::fmt::Display;

    pub struct NationalId(String);

    impl Display for NationalId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl AsRef<str> for NationalId {
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    #[cfg(test)]
    #[test]
    fn test_display_works() {
        let id = NationalId("1234456".to_owned());
        println!("{id}")
    }
}

mod age_related {
    use derive_more::{Add, Display, From, Sum};

    #[derive(From, Add, Display, Sum)]
    pub struct Age(u32);

    #[cfg(test)]
    #[test]
    fn test_age_addition() {
        let ages = vec![Age(12), Age(33)];

        let sum = ages.into_iter().sum::<Age>();

        println!("{sum}");
    }
}
