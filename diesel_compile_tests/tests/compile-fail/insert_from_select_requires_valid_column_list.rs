#[macro_use]
extern crate diesel;

use diesel::*;
use diesel::pg::PgConnection;

table! {
    users {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
    }
}

table! {
    posts (user_id) {
        user_id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
    }
}

table! {
    comments (post_id) {
        post_id -> Integer,
        body -> Nullable<Text>,
    }
}

fn main() {
    use users::dsl::*;
    use posts::dsl::*;
    let conn = PgConnection::establish("").unwrap();

    // Sanity check, valid query with no column list
    users
        .insert_into(posts)
        .execute(&conn)
        .unwrap();

    // Sanity check, valid query with single column
    users.select(id)
        .insert_into(posts)
        .into_columns(user_id)
        .execute(&conn)
        .unwrap();

    // Sanity check, valid query with column list
    users.select((name, hair_color))
        .insert_into(posts)
        .into_columns((title, body))
        .execute(&conn)
        .unwrap();

    // No column list, mismatched types
    users.select((name, hair_color))
        .insert_into(posts)
        .execute(&conn)
        //~^ ERROR type mismatch
        .unwrap();

    // Single column, wrong table
    users.select(id)
        .insert_into(posts)
        .into_columns(comments::post_id);
        //~^ ERROR type mismatch

    // Single column, wrong type
    users.select(id)
        .insert_into(posts)
        .into_columns(title);
        //~^ ERROR type mismatch

    // Multiple columns, one from wrong table
    users.select((id, name))
        .insert_into(posts)
        .into_columns((comments::post_id, title));
        //~^ ERROR E0277

    // Multiple columns, both from wrong table
    users.select((id, hair_color))
        .insert_into(posts)
        .into_columns((comments::post_id, comments::body));
        //~^ ERROR type mismatch
        //~| ERROR type mismatch

    // Multiple columns, one wrong type
    users.select((id, name))
        .insert_into(posts)
        .into_columns((user_id, body));
        //~^ ERROR type mismatch

    // Multiple columns, both wrong types
    users.select((id, name))
        .insert_into(posts)
        .into_columns((title, body));
        //~^ ERROR type mismatch
}
