use crate::sorting::OrderedCopy;

/// Ordena um vetor em ordem crescente usando o algoritmo Merge Sort recursivo.
///
/// # Parâmetros
/// - `array`: Uma referência mutável para um vetor de elementos do tipo `T` que implementam `OrderedCopy`.
///
/// # Comportamento
/// - A função `recursive_merge_sort` realiza a ordenação dividindo recursivamente o vetor em metades.
/// - O vetor é dividido até que cada sub-vetor contenha um único elemento (ou esteja vazio), e então as sub-listas são mescladas.
/// - A função auxiliar `merge_sorting` realiza essa divisão recursiva e chama `merge` para combinar as sub-listas ordenadas.
///
/// # Exemplo de Uso
/// ```
/// let mut vetor = vec![5, 3, 8, 4, 2];
/// recursive_merge_sort(&mut vetor);
/// println!("{:?}", vetor); // Saída: [2, 3, 4, 5, 8]
/// ```
///
/// # Complexidade de Tempo
/// - Pior caso: O(n log n), onde `n` é o número de elementos no vetor.
/// - Melhor caso: O(n log n), pois o Merge Sort sempre divide e mescla os elementos.
///
pub fn recursive_merge_sort(array: &mut Vec<impl OrderedCopy>) {
    *array = merge_sorting(array);
}

fn merge_sorting<T: OrderedCopy>(arr: &mut Vec<T>) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left_arr = &arr[0..mid];
    let right_arr = &arr[mid..];
    let left = merge_sorting(&mut left_arr.to_vec());
    let right = merge_sorting(&mut right_arr.to_vec());

    merge(left, right)
}

fn merge<T: OrderedCopy>(arr_1: Vec<T>, arr_2: Vec<T>) -> Vec<T> {
    let mut i = 0;
    let mut j = 0;

    let mut result = Vec::new();

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
