mod repetitive {
    use uuid::Uuid;

    struct PersonId(Uuid);

    impl PersonId {
        fn new() -> Self {
            PersonId(Uuid::new_v4())
        }
    }

    impl From<Uuid> for PersonId {
        fn from(value: Uuid) -> Self {
            PersonId(value)
        }
    }

    struct SchoolId(Uuid);

    impl SchoolId {
        fn new() -> Self {
            Self(Uuid::new_v4())
        }
    }

    impl From<Uuid> for SchoolId {
        fn from(value: Uuid) -> Self {
            SchoolId(value)
        }
    }
}

mod macros_for_reuse {
    #[macro_export]
    macro_rules! uuid_key {
        ($TypeName: ident) => {
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $TypeName(uuid::Uuid);

            impl $TypeName {
                pub fn new() -> Self {
                    $TypeName(uuid::Uuid::new_v4())
                }
            }

            impl From<uuid::Uuid> for $TypeName {
                fn from(id: uuid::Uuid) -> Self {
                    $TypeName(id)
                }
            }
        };
    }

    uuid_key!(PersonId);

    uuid_key!(SchoolId);

    #[cfg(test)]
    #[test]
    fn test_macro_created_ids() {
        let person_id = PersonId::new();
        let school_id = SchoolId::new();
    }
}
