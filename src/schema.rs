// @generated automatically by Diesel CLI.

diesel::table! {
    contact (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        location -> Varchar,
        #[max_length = 255]
        phone_number -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::table! {
    edukasi (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        major -> Varchar,
        #[max_length = 255]
        periode -> Varchar,
        description -> Longtext,
        #[max_length = 255]
        logo -> Varchar,
    }
}

diesel::table! {
    experience (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        periode -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        division -> Varchar,
        description -> Longtext,
    }
}

diesel::table! {
    hero (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Longtext,
        #[max_length = 255]
        image -> Varchar,
    }
}

diesel::table! {
    portofolio (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Longtext,
        skills -> Text,
        link -> Longtext,
        #[max_length = 255]
        image -> Varchar,
    }
}
diesel::table! {
    skill (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 255]
        skills -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    contact,
    edukasi,
    experience,
    hero,
    portofolio,
    skill,
);
