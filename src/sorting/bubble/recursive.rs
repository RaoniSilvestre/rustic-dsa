use crate::sorting::OrderedCopy;

/// Ordena um vetor em ordem crescente usando o algoritmo Bubble Sort recursivo.
///
/// # Parâmetros
/// - `array`: Uma referência mutável para um vetor de elementos do tipo `T` que implementam `OrderedCopy`.
///
/// # Comportamento
/// - A função `recursive_bubble_sort` chama uma função auxiliar `recursive_bubble` para realizar as passagens recursivas.
/// - Cada chamada recursiva de `bubble_pass` percorre o vetor até que o maior elemento "suba" para o final.
/// - O processo é repetido em sub-vetores cada vez menores até que o vetor esteja completamente ordenado.
///
/// # Exemplo de Uso
/// ```
/// let mut vetor = vec![5, 3, 8, 4, 2];
/// recursive_bubble_sort(&mut vetor);
/// println!("{:?}", vetor); // Saída: [2, 3, 4, 5, 8]
/// ```
///
/// # Complexidade de Tempo
/// Onde `n` é o numero de elementos no vetor;
/// - Pior caso: O(n²).
/// - Melhor caso: O(mega)(n²).
pub fn recursive_bubble_sort<T: OrderedCopy>(array: &mut Vec<T>) {
    let len = array.len();
    recursive_bubble(array, len)
}

fn recursive_bubble<T: OrderedCopy>(vec: &mut Vec<T>, n: usize) {
    if n == 1 {
        return;
    }
    bubble_pass(vec, 0, n);
    recursive_bubble(vec, n - 1);
}

fn bubble_pass<T: OrderedCopy>(vec: &mut Vec<T>, i: usize, j: usize) {
    if i == j - 1 {
        return;
    }

    if vec[i] > vec[i + 1] {
        vec.swap(i, i + 1);
    }

    bubble_pass(vec, i + 1, j);
}
