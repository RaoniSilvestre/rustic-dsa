use crate::sorting::OrderedCopy;

/// Ordena um vetor em ordem crescente usando o algoritmo Quick Sort iterativo.
///
/// # Parâmetros
/// - `array`: Uma referência mutável para um vetor de elementos do tipo `T` que implementam `OrderedCopy`.
///
/// # Comportamento
/// - O algoritmo utiliza uma pilha (`stack`) para simular a recursão do Quick Sort de forma iterativa.
/// - A função `particionar` é chamada para organizar os elementos em torno de um pivô. O pivô é colocado na posição correta, e os elementos menores que o pivô são movidos à esquerda, enquanto os maiores são movidos à direita.
/// - A pilha mantém os limites das partições a serem processadas. O algoritmo continua particionando o vetor até que todas as partições estejam ordenadas.
///
/// # Exemplo de Uso
/// ```
/// let mut vetor = vec![5, 3, 8, 4, 2];
/// iterative_quick_sort(&mut vetor);
/// println!("{:?}", vetor); // Saída: [2, 3, 4, 5, 8]
/// ```
///
/// # Complexidade de Tempo
/// - Pior caso: O(n²), onde `n` é o número de elementos no vetor (quando o pivô é o menor ou maior elemento em todas as partições).
/// - Melhor caso: O(n log n), quando o pivô divide bem o vetor (caso médio).
///
pub fn iterative_quick_sort<T: OrderedCopy>(array: &mut Vec<T>) {
    let mut stack = Vec::new();
    let tamanho = array.len();

    stack.push(0);
    stack.push(tamanho - 1);

    while let Some(right) = stack.pop() {
        if let Some(left) = stack.pop() {
            let pivo_index = particionar(array, left, right);

            if pivo_index > 0 && pivo_index - 1 > left {
                stack.push(left);
                stack.push(pivo_index - 1);
            }

            if pivo_index + 1 < right {
                stack.push(pivo_index + 1);
                stack.push(right);
            }
        }
    }
}

fn particionar<T: OrderedCopy>(array: &mut [T], left: usize, rigth: usize) -> usize {
    let pivo = array[rigth];
    let mut i = left as isize - 1;

    for j in left..(rigth) {
        if array[j] <= pivo {
            i += 1;
            array.swap(i as usize, j);
        }
    }

    array.swap((i + 1) as usize, rigth);
    (i + 1) as usize
}
