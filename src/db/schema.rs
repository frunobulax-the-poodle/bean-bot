// @generated automatically by Diesel CLI.

diesel::table! {
    fav_msgs (id) {
        id -> Integer,
        user_id -> BigInt,
        guild_id -> BigInt,
        channel_id -> BigInt,
        message_id -> BigInt,
    }
}
