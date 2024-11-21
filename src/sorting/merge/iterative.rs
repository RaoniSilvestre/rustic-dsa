use crate::sorting::OrderedCopy;

/// Ordena um vetor em ordem crescente usando o algoritmo Merge Sort iterativo.
///
/// # Parâmetros
/// - `array`: Uma referência mutável para um vetor de elementos do tipo `T` que implementam `OrderedCopy`.
///
/// # Comportamento
/// - A função `iterative_merge_sort` realiza a ordenação dividindo o vetor em partes menores e as mesclando.
/// - Em cada iteração, as sub-listas de tamanho `i` são mescladas para formar listas ordenadas maiores.
/// - A função `merge_sorting` aplica a divisão iterativa no vetor e chama a função auxiliar `merge` para mesclar sub-listas ordenadas.
/// - `merge` combina duas sub-listas ordenadas em uma única lista ordenada.
///
/// # Exemplo de Uso
/// ```
/// let mut vetor = vec![5, 3, 8, 4, 2];
/// iterative_merge_sort(&mut vetor);
/// println!("{:?}", vetor); // Saída: [2, 3, 4, 5, 8]
/// ```
///
/// # Complexidade de Tempo
/// - Pior caso: O(n log n), onde `n` é o número de elementos no vetor.
/// - Melhor caso: O(n log n), pois o Merge Sort sempre divide e mescla elementos.
///
pub fn iterative_merge_sort(array: &mut Vec<impl OrderedCopy>) {
    *array = merge_sorting(array, array.len());
}

fn merge_sorting<T: OrderedCopy>(arr: &Vec<T>, tamanho: usize) -> Vec<T> {
    let mut result: Vec<T> = arr.to_vec();
    let mut i: usize = 1;

    while i < tamanho {
        let mut esquerda: usize = 0;

        while esquerda < tamanho - 1 {
            let mid: usize = std::cmp::min(esquerda + i - 1, tamanho - 1);
            let right_arr: usize = std::cmp::min(esquerda + 2 * i - 1, tamanho - 1);

            let merged: Vec<T> = merge(
                &result[esquerda..mid + 1].to_vec(),
                &result[mid + 1..right_arr + 1].to_vec(),
            );

            for k in 0..merged.len() {
                result[esquerda + k] = merged[k].clone();
            }

            esquerda += 2 * i;
        }

        i *= 2;
    }

    result
}

fn merge<T: OrderedCopy>(arr_1: &Vec<T>, arr_2: &Vec<T>) -> Vec<T> {
    let mut i: usize = 0;
    let mut j: usize = 0;

    let mut result: Vec<T> = Vec::new();

    while i < arr_1.len() && j < arr_2.len() {
        if arr_2[j] > arr_1[i] {
            result.push(arr_1[i]);
            i += 1;
        } else {
            result.push(arr_2[j]);
            j += 1;
        }
    }

    while i < arr_1.len() {
        result.push(arr_1[i]);
        i += 1;
    }

    while j < arr_2.len() {
        result.push(arr_2[j]);
        j += 1;
    }

    result
}
