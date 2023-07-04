// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Nullable<Uuid>,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 1]
        middle_initial -> Nullable<Bpchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        frozen -> Nullable<Bool>,
    }
}
