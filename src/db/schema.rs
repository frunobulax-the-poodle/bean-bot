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
    rolemenu (id) {
        id -> Int4,
        guild_id -> Int8,
        name -> Text,
        max_selectable -> Nullable<Int4>,
        roles -> Array<Nullable<Int8>>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(fav_msgs, rolemenu,);
