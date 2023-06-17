mod old {
    struct User {
        name: String,
        email: String,
    }

    impl User {
        fn validate_user(name: String, email: String) -> anyhow::Result<Self> {
            let email = validate_email(email)?;
            Ok(Self { name, email })
        }
    }

    fn validate_email(email: String) -> anyhow::Result<String> {
        todo!()
    }

    struct Business {
        name: String,
        email: String,
    }

    mod communication {
        use super::{validate_email, Business};

        async fn contact_business(business: &Business) -> anyhow::Result<()> {
            let valid_business_email = validate_email(business.email.clone())?;

            todo!()
        }
    }
}

mod new {
    struct Email(String);

    impl Email {
        fn new(value: String) -> anyhow::Result<Self> {
            todo!()
        }
    }

    struct User {
        name: String,
        email: Email,
    }

    impl User {
        fn validate_user(name: String, email: String) -> anyhow::Result<Self> {
            let email = Email::new(email)?;
            Ok(Self { name, email })
        }
    }

    struct Business {
        name: String,
        email: Email,
    }

    mod communication {
        use super::Business;

        async fn contact_business(business: &Business) -> anyhow::Result<()> {
            // no need to validate email here
            todo!()
        }
    }
}
