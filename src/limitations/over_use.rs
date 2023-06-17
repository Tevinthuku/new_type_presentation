pub mod over_use_example {

    pub struct PersonId(uuid::Uuid);

    pub struct Name(String);

    pub struct Age(u8);

    pub struct Address(String);
    pub struct Person {
        pub id: PersonId,
        pub name: Name,
        pub age: Age,
        pub address: Address,
    }

    impl Person {
        fn new(id: PersonId, name: Name, age: Age, address: Address) -> Self {
            Self {
                id,
                name,
                age,
                address,
            }
        }
    }
}

mod sensible_usage {
    use anyhow::bail;

    pub struct Name(String);
    impl TryFrom<String> for Name {
        type Error = anyhow::Error;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.trim().len() < 3 {
                bail!("Not a valid name")
            }
            Ok(Self(value.trim().to_owned()))
        }
    }

    struct Person {
        name: Name,
        address: String,
        age: u8,
        year_joined: u16,
    }
}
