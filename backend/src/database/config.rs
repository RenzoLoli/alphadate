use super::Connection;

pub async fn config_database(conn: &Connection) {
    // users table
    conn.db()
        .query(
            "
                DEFINE TABLE users SCHEMAFULL;

                DEFINE FIELD username ON TABLE users TYPE string;
                DEFINE FIELD couplename ON TABLE users TYPE string;
                DEFINE FIELD email ON TABLE users TYPE string
                    ASSERT string::is::email($value);
                DEFINE FIELD password ON TABLE users TYPE string;
                DEFINE FIELD anniversary ON TABLE users TYPE string;
                DEFINE FIELD photo ON TABLE users TYPE string
                    ASSERT string::is::url($value);
            ",
        )
        .await
        .expect("cannot create table users");

    // alphabets table
    conn.db()
        .query(
            "
                DEFINE TABLE alphabets SCHEMAFULL;

                DEFINE FIELD title ON TABLE alphabets TYPE string;
                DEFINE FIELD user_id ON TABLE alphabets TYPE record;
            ",
        )
        .await
        .expect("cannot create table alphabets");

    // user_dates table
    conn.db()
        .query(
            "
                DEFINE TABLE user_dates SCHEMAFULL;

                DEFINE FIELD letter ON TABLE user_dates TYPE string;
                DEFINE FIELD completed ON TABLE user_dates TYPE bool;
                DEFINE FIELD alphabet_id ON TABLE user_dates TYPE record;
                DEFINE FIELD date_idea_id ON TABLE user_dates TYPE record;
            ",
        )
        .await
        .expect("cannot create table user_dates");

    // date_ideas table
    conn.db()
        .query(
            "
                DEFINE TABLE date_ideas SCHEMAFULL;

                DEFINE FIELD idea ON TABLE date_ideas TYPE string;
                DEFINE FIELD description ON TABLE date_ideas TYPE string;
            ",
        )
        .await
        .expect("cannot create table date_ideas");

    // date_idea_tags table
    conn.db()
        .query(
            "
                DEFINE TABLE date_idea_tags SCHEMAFULL;

                DEFINE FIELD date_idea_id ON TABLE date_idea_tags TYPE record;
                DEFINE FIELD tag_id ON TABLE date_idea_tags TYPE record;
            ",
        )
        .await
        .expect("cannot create table date_idea_tags");

    // tags table
    conn.db()
        .query(
            "
                DEFINE TABLE tags SCHEMAFULL;

                DEFINE FIELD name ON TABLE tags TYPE string;
            ",
        )
        .await
        .expect("cannot create table tags");
}
