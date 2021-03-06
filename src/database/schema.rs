table! {
    admin (user_id) {
        user_id -> Nullable<Integer>,
    }
}

table! {
    order (order_id) {
        order_id -> Varchar,
        product_id -> Nullable<Varchar>,
        user_id -> Integer,
        ordered_date -> Integer,
        expected_date -> Integer,
        order_status -> Nullable<Varchar>,
    }
}

table! {
    product (product_id) {
        product_id -> Nullable<Varchar>,
        product_name -> Varchar,
        description -> Nullable<Text>,
        price -> Float,
        rating -> Nullable<Float>,
        quantity -> Nullable<Integer>,
    }
}

table! {
    user (user_id) {
        user_id -> Nullable<Integer>,
        user_name -> Varchar,
        password -> Varchar,
        email -> Varchar,
        display_name -> Varchar,
        address -> Text,
    }
}

joinable!(admin -> user (user_id));
joinable!(order -> product (product_id));
joinable!(order -> user (user_id));

allow_tables_to_appear_in_same_query!(admin, order, product, user,);
