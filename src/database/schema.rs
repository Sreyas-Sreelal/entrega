table! {
    admins (user_id) {
        user_id -> Integer,
    }
}

table! {
    orders (order_id) {
        order_id -> Integer,
        product_id -> Integer,
        user_id -> Integer,
        ordered_date -> Integer,
        expected_date -> Integer,
    }
}

table! {
    products (product_id) {
        product_id -> Integer,
        product_name -> Text,
        price -> Float,
        rating -> Nullable<Float>,
    }
}

table! {
    users (user_id) {
        user_id -> Nullable<Integer>,
        user_name -> Text,
        password -> Text,
        email -> Text,
        display_name -> Text,
        address -> Text,
    }
}

joinable!(admins -> users (user_id));
joinable!(orders -> products (product_id));
joinable!(orders -> users (user_id));

allow_tables_to_appear_in_same_query!(
    admins,
    orders,
    products,
    users,
);
