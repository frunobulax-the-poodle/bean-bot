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
