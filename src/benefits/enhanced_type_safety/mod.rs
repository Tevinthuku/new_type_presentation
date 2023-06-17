mod old {
    struct Student {
        id: String,
        name: String,
        library_card_id: String,
    }

    impl Student {
        pub fn new(id: String, name: String, library_card_id: String) -> Self {
            Self {
                id,
                name,
                library_card_id,
            }
        }
    }

    async fn find_student_by_id(id: String) {
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use crate::benefits::enhanced_type_safety::old::{find_student_by_id, Student};

        #[tokio::test]
        async fn test_finding_student_by_id_works() {
            let student = Student::new("name".to_owned(), "id".to_owned(), "ABX123".to_owned());

            find_student_by_id(student.library_card_id).await;
        }
    }
}

mod new {
    struct StudentId(String);

    struct Student {
        id: StudentId,
        name: String,
        library_card_id: String,
    }

    impl Student {
        pub fn new(id: StudentId, name: String, library_card_id: String) -> Self {
            Self {
                id,
                name,
                library_card_id,
            }
        }
    }

    async fn find_student_by_id(id: StudentId) {
        todo!()
    }

    #[cfg(test)]
    mod tests {
        use crate::benefits::enhanced_type_safety::new::{find_student_by_id, Student, StudentId};

        #[tokio::test]
        async fn test_finding_student_by_id_works() {
            let student = Student::new(
                StudentId("name".to_owned()),
                "id".to_owned(),
                "ABX123".to_owned(),
            );

            find_student_by_id(student.id).await;
        }
    }
}
