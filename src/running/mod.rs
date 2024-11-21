use std::{fmt::Display, fs::File, io::Write, path::Path, time::Instant};

use crate::sorting::{
    bubble::{iterative::iterative_bubble_sort, recursive::recursive_bubble_sort},
    merge::{iterative::iterative_merge_sort, recursive::recursive_merge_sort},
    quick::{iterative::iterative_quick_sort, recursive::recursive_quick_sort},
    OrderedCopy,
};

type SortFunction<T> = fn(&mut Vec<T>);

/// Executa uma função de ordenação em um vetor e mede o tempo necessário para completá-la.
///
/// # Parâmetros
/// - `array`: Um vetor de elementos a serem ordenados.
/// - `sort_function`: Uma função de ordenação que será aplicada ao vetor.
/// - `function_name`: O nome da função de ordenação, usado para exibir resultados e gerar o nome do arquivo de saída.
///
/// # Requisitos de Tipo
/// - `T`: Deve implementar `OrderedCopy` e `Display`, para que possa ser ordenado e exibido.
/// - `F`: Deve ser uma função mutável (`FnMut`) que recebe uma referência mutável para `Vec<T>`.
///
/// # Comportamento
/// - A função calcula o tempo de execução da função de ordenação fornecida usando `Instant::now`.
/// - Exibe o nome da função de ordenação e o tempo necessário para completá-la.
/// - Salva o vetor ordenado em um arquivo de texto localizado em `./data/output/<function_name>.txt`.
///
/// # Panics
/// - A função gera um pânico (para o programa) se houver um erro ao criar ou escrever no arquivo.
///
/// # Exemplo
/// ```
/// run(vec![3, 1, 4, 1, 5], some_sort_function, String::from("SORT_NAME"));
/// ```
pub fn run<T, F>(mut array: Vec<T>, mut sort_function: F, function_name: String)
where
    T: OrderedCopy + Display,
    F: FnMut(&mut Vec<T>),
{
    let now = Instant::now();
    sort_function(&mut array);
    let elapsed = now.elapsed();

    println!("{}:\n  - TIMING = {:.2?}\n", function_name, elapsed);

    let dir_path = "./data/output/";
    let mut file_name = function_name.clone();
    file_name.push_str(".txt");

    let file_path = Path::new(dir_path).join(file_name.clone());
    let mut file = File::create(file_path).expect("Erro ao criar o arquivo");

    for number in array {
        writeln!(file, "{}", number).expect("Erro ao escrever no arquivo");
    }
}

/// Executa múltiplos algoritmos de ordenação em uma cópia do vetor fornecido e exibe o tempo de execução de cada um.
///
/// # Parâmetros
/// - `array`: Um vetor de inteiros a ser ordenado usando diferentes algoritmos.
///
/// # Comportamento
/// - A função cria uma lista de pares contendo nomes de funções de ordenação e as respectivas funções.
/// - Para cada algoritmo, chama `run`, passando uma cópia do vetor original, a função de ordenação e o nome da função.
/// - Exibe o tamanho do array no início da execução.
///
/// # Exemplo
/// ```
/// run_array(vec![10, 7, 3, 2, 1]);
/// ```
///
/// # Observação
/// Cada função de ordenação é aplicada a uma cópia do array, então o array original permanece inalterado.
pub fn run_array(array: Vec<i32>) {
    let array_of_sorts: Vec<(&str, SortFunction<i32>)> = vec![
        ("RECURSIVE_QUICKSORT", recursive_quick_sort),
        ("ITERATIVE_QUICKSORT", iterative_quick_sort),
        ("RECURSIVE_MERGESORT", recursive_merge_sort),
        ("ITERATIVE_MERGESORT", iterative_merge_sort),
        ("RECURSIVE_BUBBLESORT", recursive_bubble_sort),
        ("ITERATIVE_BUBBLESORT", iterative_bubble_sort),
    ];

    println!("---------------------------\n");
    println!("Array Length: {}\n", array.len());

    for (name, func) in array_of_sorts {
        run::<i32, _>(array.clone(), func, String::from(name));
    }
}
