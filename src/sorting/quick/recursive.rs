use crate::sorting::OrderedCopy;
/// Ordena um vetor em ordem crescente usando o algoritmo Quick Sort recursivo.
///
/// # Parâmetros
/// - `array`: Uma referência mutável para um vetor de elementos do tipo `T` que implementam `OrderedCopy`.
///
/// # Comportamento
/// - A função `sorting` realiza a ordenação recursiva. Ela chama a função `partition` para dividir o vetor em duas partes ao redor de um pivô e depois chama recursivamente para ordenar essas partes.
/// - O pivô é escolhido como o primeiro elemento da partição, e a partição é feita de forma que todos os elementos menores que o pivô fiquem à esquerda e os maiores à direita.
///
/// # Exemplo de Uso
/// ```
/// let mut vetor = vec![5, 3, 8, 4, 2];
/// recursive_quick_sort(&mut vetor);
/// println!("{:?}", vetor); // Saída: [2, 3, 4, 5, 8]
/// ```
///
/// # Complexidade de Tempo
/// - Pior caso: O(n²), onde `n` é o número de elementos no vetor (quando o pivô é o menor ou maior elemento em todas as partições).
/// - Melhor caso: O(n log n), quando o pivô divide bem o vetor (caso médio).
///
pub fn recursive_quick_sort<T: OrderedCopy>(array: &mut Vec<T>) {
    sorting(array, 0, array.len() - 1)
}

fn sorting<T: OrderedCopy>(lista: &mut Vec<T>, left: usize, right: usize) {
    if left < right {
        let index_pivot = partition(lista, left, right);
        sorting(lista, left, index_pivot);
        sorting(lista, index_pivot + 1, right);
    }
}

fn partition<T: OrderedCopy>(lista: &mut Vec<T>, mut left: usize, mut right: usize) -> usize {
    let pivot = left;
    left += 1;
    while left <= right {
        while left < lista.len() && lista[left] < lista[pivot] {
            left += 1;
        }
        while lista[right] > lista[pivot] {
            if right == 0 {
                break;
            }
            right -= 1;
        }
        if right >= left {
            lista.swap(left, right);
            if right != 0 {
                right -= 1;
            }
        }
    }
    lista.swap(pivot, right);
    right
}
