mod old {
    struct Student {
        id: u64,
        name: String,
        library_card_id: String,
    }

    struct ClassRoom {
        students: Vec<u64>,
        teacher: String,
    }

    struct Teacher {
        id: u64,
        name: String,
    }

    struct Subject {
        name: String,
        enrolled_students: Vec<u64>,
    }
}

mod new {
    struct StudentId(u64); // change from u64 to Uuid;

    struct Student {
        id: StudentId,
        name: String,
        library_card_id: String,
    }

    struct ClassRoom {
        students: Vec<StudentId>,
        teacher: String,
    }

    struct Teacher {
        id: u64,
        name: String,
    }

    struct Subject {
        name: String,
        enrolled_students: Vec<StudentId>,
    }
}
