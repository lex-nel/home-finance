extern crate rusqlite;

use self::rusqlite::Connection;

pub fn tables() {
    let conn = Connection::open("./home_finance.sqlite3").unwrap();

    users(&conn);
}

fn users(conn: &Connection) {
    match conn.execute(
        "create table users (
            id          int unsigned auto_increment primary key,
            username    varchar(255) default '' not null,
            password    varchar(100) default '' not null,
            first_name  varchar(255)            null,
            last_name   varchar(255)            null,
            mid_name    varchar(255)            null,
            birthday    timestamp               null,
            description text                    null,
            created_at  timestamp               null,
            updated_at  timestamp               null,
            constraint username
            unique (username)
        )",
        &[],
    ) {
        Ok(_) => println!("table users created"),
        Err(err) => println!("{}", err),
    }
}
