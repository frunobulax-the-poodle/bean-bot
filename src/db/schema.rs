// @generated automatically by Diesel CLI.

diesel::table! {
    fav_msgs (id) {
        id -> Int4,
        user_id -> Int8,
        guild_id -> Int8,
        channel_id -> Int8,
        message_id -> Int8,
    }
}

diesel::table! {
    role_menu (id) {
        id -> Int4,
        guild_id -> Int8,
        name -> Text,
        max_selectable -> Nullable<Int4>,
    }
}

diesel::table! {
    role_option (id) {
        id -> Int4,
        role_id -> Int8,
        description -> Nullable<Text>,
        emoji -> Nullable<Text>,
        role_menu_id -> Int4,
    }
}

diesel::joinable!(role_option -> role_menu (role_menu_id));

diesel::allow_tables_to_appear_in_same_query!(fav_msgs, role_menu, role_option,);
