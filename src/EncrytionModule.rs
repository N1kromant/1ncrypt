#![allow(non_snake_case)]
use rand::Rng;

pub static BLOCKSIZE: usize = 4;

pub fn blockXor(data: &[u8], key: &[u8]) -> Vec<u8> {
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

pub fn encrypt(data: &mut [u8], key: &mut [u8]) -> Vec<u8> {
    let VI: Vec<u8> = getVI();
    let mut result: Vec<u8> = vec![];
    let mut lastEnc: Vec<u8> = vec![];
    let mut thisEnc: Vec<u8> = vec![];

    result.extend(VI.clone());

    for (i, block) in data.chunks_mut(BLOCKSIZE).enumerate() {
        if i == 0 {
            blockXor(block, VI.as_slice());
            thisEnc = blockXor(block, key);
        } else {
            blockXor(block, &lastEnc);
            thisEnc = blockXor(block, key);
        }

        result.extend(blockXor(block, key));
        lastEnc = thisEnc;
    }

    return result;
}

pub fn getVI() -> Vec<u8>{
    let mut rng = rand::thread_rng();
    
    let random_bytes: Vec<u8> = (0..4).map(|_| rng.gen()).collect();
    return random_bytes;
}