table! {
    accounts (id) {
        id -> Varchar,
        name -> Varchar,
        currency -> Varchar,
        description -> Varchar,
    }
}

table! {
    snapshots (id) {
        id -> Int4,
        account_id -> Varchar,
        date -> Date,
        amount -> Int4,
    }
}

joinable!(snapshots -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    snapshots,
);
