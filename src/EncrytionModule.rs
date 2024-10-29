pub fn blockEncrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // Создаем новый вектор для хранения результата
    let mut result = Vec::with_capacity(data.len());

    // Проходим по каждому байту в data
    for (i, &byte) in data.iter().enumerate() {
        // Получаем соответствующий байт из key, используя оператор `%` для повторного использования ключа
        let key_byte = key[i % key.len()];

        // Выполняем XOR и добавляем результат в вектор
        result.push(byte ^ key_byte);
    }

    result
}