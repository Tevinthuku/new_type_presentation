mod rigid {
    use anyhow::bail;
    use derive_more::Display;

    #[derive(Display)]
    pub struct Email(String);

    impl<'a> TryFrom<&'a str> for Email {
        type Error = anyhow::Error;

        fn try_from(value: &'a str) -> Result<Self, Self::Error> {
            bail!("To be implemented")
        }
    }

    pub fn execute(input: Email) -> anyhow::Result<()> {
        bail!("To be implemented")
    }

}

mod flexible {
    use anyhow::bail;
    use derive_more::Display;

    #[derive(Display)]
    struct Email(String);

    impl<'a> TryFrom<&'a str> for Email {
        type Error = anyhow::Error;

        fn try_from(value: &'a str) -> Result<Self, Self::Error> {
            bail!("To be implemented")
        }
    }

    pub fn execute(input: impl AsRef<str>) -> anyhow::Result<()> {
        let input = input.as_ref();
        let email = Email::try_from(input)?;
        println!("{email}");
        Ok(())
    }

}
