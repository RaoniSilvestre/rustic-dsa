use std::fs::File;
use std::io::Write;
use std::path::Path;

use rand::Rng;

/// Gera arquivos de texto com listas de números aleatórios e os salva no diretório `./data/input/`.
///
/// Esta função cria três arquivos de tamanhos diferentes contendo listas de números gerados aleatoriamente,
/// cada arquivo com uma quantidade predefinida de números inteiros. Foi utilizada para criar os
/// inputs para testar os algoritmos de ordenação
///
/// # Detalhes
/// - Diretório de saída: `./data/input/`
/// - Arquivos gerados: `file_1000.txt`, `file_10000.txt`, `file_100000.txt`
/// - Quantidades de números: 1000, 10.000 e 100.000, respectivamente.
///
/// # Comportamento
/// - Para cada tamanho definido em `file_sizes`, a função:
///     1. Gera uma lista de números usando `generate_list`.
///     2. Cria um arquivo correspondente no diretório `./data/input/`.
///     3. Escreve cada número da lista no arquivo em uma nova linha.
/// - Exibe uma mensagem no console quando cada arquivo é criado com sucesso.
///
/// # Panics
/// - A função gera um pânico caso ocorra um erro ao criar ou escrever nos arquivos.
///
/// # Observação
/// Caso a pasta ./data/input não exista, isso aqui não vai funcionar visto que o código não tenta
/// criar a pasta, apenas criar um arquivo dentro dessa pasta.
///
fn main() {
    let dir_path = "./data/input/";

    let file_sizes = [1000, 10_000, 100_000];
    let file_names = ["file_1000.txt", "file_10000.txt", "file_100000.txt"];

    for (i, &size) in file_sizes.iter().enumerate() {
        let numbers = generate_list(size);

        let file_path = Path::new(dir_path).join(file_names[i]);
        let mut file = File::create(file_path).expect("Erro ao criar o arquivo");

        for number in numbers {
            writeln!(file, "{}", number).expect("Erro ao escrever no arquivo");
        }

        println!("Arquivo {} criado com sucesso!", file_names[i]);
    }
}

pub fn generate_list(n: usize) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        list.push(rng.gen_range(0..1000));
    }
    list
}
