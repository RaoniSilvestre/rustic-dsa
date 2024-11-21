use crate::sorting::OrderedCopy;

/// Ordena um vetor em ordem crescente usando o algoritmo Bubble Sort iterativo.
///
/// # Parâmetros
/// - `array`: Uma referência mutável para um vetor de elementos do tipo `T` que implementam `OrderedCopy`.
///
/// # Comportamento
/// - A função itera sobre o vetor, repetidamente trocando elementos adjacentes que estão fora de ordem.
/// - Em cada passagem, o maior elemento restante "sobe" para o final da parte não ordenada do vetor.
/// - Após cada iteração, o algoritmo marca o vetor como ordenado. Se nenhuma troca ocorrer durante uma iteração, o loop é encerrado antecipadamente.
///
///
/// # Exemplo de Uso
/// ```
/// let mut vetor = vec![5, 3, 8, 4, 2];
/// iterative_bubble_sort(&mut vetor);
/// println!("{:?}", vetor); // Saída: [2, 3, 4, 5, 8]
/// ```
///
/// # Complexidade de Tempo
/// - Pior caso: O(n²), onde `n` é o número de elementos no vetor.
/// - Melhor caso: O(n²), quando o vetor já está ordenado.
pub fn iterative_bubble_sort<T: OrderedCopy>(array: &mut Vec<T>) {
    let len = array.len();

    for i in 0..len {
        for j in 0..len - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1)
            }
        }
    }
}
