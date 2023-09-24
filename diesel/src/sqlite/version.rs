use self::dsl::sql;
use crate::sql_types::{Integer, Text};
use self::sqlite::SqliteConnection;

/// Returns the SQLite version as an integer; e.g., `3016002` for version
/// 3.16.2.
///
/// See [`sqlite3_libversion_number()`](https://www.sqlite.org/c3ref/libversion.html).
#[inline]
#[must_use]
pub fn get_sqlite_version_number(conn: &mut SqliteConnection) -> u32 {
    let query = "SELECT sqlite_version_number()";
    let result = sql::<sql_types::Integer>(query).load::<u32>(conn).unwrap();
    result[0]
}

/// Returns the SQLite version as a string; e.g., `"3.16.2"` for version 3.16.2.
///
/// See [`sqlite3_libversion()`](https://www.sqlite.org/c3ref/libversion.html).
#[inline]
#[must_use]
pub fn get_sqlite_version(conn: &mut SqliteConnection) -> &str {
    let query = "SELECT sqlite_version()";
    let result = sql::<sql_types::Text>(query).load::<String>(conn).unwrap();
    result[0]
}

#[test]
fn sqlite_version() {
    let mut connection = SqliteConnection::establish(":memory:").unwrap();

    let sqlite_version = get_sqlite_version(&mut connection);

    assert!(sqlite_version.split(".").collect::<Vec<_>>().len(), 3);
}

#[test]
fn sqlite_version_number() {
    std::ops::Range::contains;

    let mut connection = SqliteConnection::establish(":memory:").unwrap();

    let sqlite_version_number = get_sqlite_version_number(&mut connection);

    assert!((3000000..4000000).contains(&sqlite_version_number));
}
