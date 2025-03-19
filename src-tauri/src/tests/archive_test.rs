#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};
    use zip::read::ZipArchive;

    #[test]
    fn test_archive() {
        let test_file_path = "/Users/eboniequ/files/data.txt";
        let test_zip_path = "/Users/eboniequ/files/archive.zip";

        let test_data = b"Hello, world!";
        let mut test_file = File::create(test_file_path).expect("Не удалось создать test file");
        test_file.write_all(test_data).expect("Ошибка записи test file");

        archive().expect("Ошибка при создании архива");

        let zip_file = File::open(test_zip_path).expect("Не найден архив");
        let mut zip = ZipArchive::new(zip_file).expect("Ошибка чтения архива");

        assert!(zip.len() > 0, "Архив пуст");
        let mut extracted_file = zip.by_name("data.txt").expect("Файл data.txt не найден в архиве");

        let mut extracted_data = Vec::new();
        extracted_file.read_to_end(&mut extracted_data).expect("Ошибка чтения из архива");
        assert_eq!(extracted_data, test_data, "Содержимое файла не совпадает");

        println!("Тест успешно пройден!");
    }
}
