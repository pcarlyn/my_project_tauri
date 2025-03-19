#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_createdb() {
        let conn = Connection::open_in_memory().unwrap();
        let result = createdb();
        assert!(result.is_ok(), "Ошибка при создании таблиц: {:?}", result);
    }

    #[test]
    fn test_insert_data() {
        let conn = Connection::open_in_memory().unwrap();
        let _ = createdb();

        let result = insert_data(
            "Иван",
            "Иванов",
            "2000-01-01",
            "+79991234567",
            "ivanivanov",
            "ivan@example.com",
        );

        assert!(result.is_ok(), "Ошибка при вставке данных: {:?}", result);
    }
}
