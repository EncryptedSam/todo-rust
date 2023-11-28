// @generated automatically by Diesel CLI.

diesel::table! {
    todo_groups (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        group_priority -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    todo_list (id) {
        id -> Int4,
        group_id -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        task_priority -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 100]
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(todo_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    todo_groups,
    todo_list,
    users,
);
