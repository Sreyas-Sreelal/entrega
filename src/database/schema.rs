table! {
    admin (user_id) {
        user_id -> Nullable<Integer>,
    }
}

table! {
    order (order_id) {
        order_id -> Integer,
        product_id -> Nullable<Text>,
        user_id -> Integer,
        ordered_date -> Integer,
        expected_date -> Integer,
    }
}

table! {
    product (product_id) {
        product_id -> Nullable<Text>,
        product_name -> Text,
        price -> Float,
        rating -> Nullable<Float>,
    }
}

table! {
    user (user_id) {
        user_id -> Nullable<Integer>,
        user_name -> Text,
        password -> Text,
        email -> Text,
        display_name -> Text,
        address -> Text,
    }
}

joinable!(admin -> user (user_id));
joinable!(order -> product (product_id));
joinable!(order -> user (user_id));

allow_tables_to_appear_in_same_query!(admin, order, product, user,);
