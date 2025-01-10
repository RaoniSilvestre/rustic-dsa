use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct MaxHeap<T> {
    data: Vec<T>,
}

/// Alteração de prioridade do MaxHeap
impl<T: Ord> MaxHeap<T> {
    /// Para a função de subir(bubble_up), a implementação é simples.
    /// Pegamos a heap(&mut self) e a posição que ira subir como argumentos.
    /// Devido as propriedades da heap, sabemos que o pai da $self[i]$ está na
    /// posição $i/2$, e dessa forma verificamos se o filho tem prioridade maior que o pai.
    /// Se for o caso, as posições do filho e do pai são trocadas,
    /// e então chama-se a função recursivamente na posição do pai.
    pub fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;

            if self[index] <= self[parent] {
                break;
            }

            self.swap(index, parent);
            index = parent;
        }
    }

    /// Primeiramente, pegamos a quantidade de elementos na Heap e então fazemos uma iteração.
    ///
    /// Para cada iteração, comparamos a prioridade do index atual com a prioridade de seus filhos,
    /// caso algum dos filhos seja maior que o pai, realizamos o swap do pai com o filho, e
    /// repetimos o processo.
    /// Se nenhum dos filhos é maior que o pai, significa que o item desceu até a posição correta,
    /// e paramos o loop.
    fn bubble_down(&mut self, mut index: usize) {
        let last_index = match self.len() {
            0 => 0,
            n => n - 1,
        };

        loop {
            let left_child = (2 * index) + 1;
            let right_child = (2 * index) + 2;
            let mut largest = index;

            if left_child <= last_index && self[left_child] > self[largest] {
                largest = left_child;
            }

            if right_child <= last_index && self[right_child] > self[largest] {
                largest = right_child;
            }

            if largest == index {
                break;
            }

            self.swap(index, largest);
            index = largest;
        }
    }
}

/// Heapsort
impl<T: Ord> MaxHeap<T> {
    /// No heapsort, o que fazemos é consumir a MaxHeap para retornar um Vetor ordenado.
    ///
    /// Para isso, criamos um vetor mutável com o tamanho da MaxHeap, e como o primeiro elemento
    /// da MaxHeap é sempre o maior dela, se você retirar repetidamente os valores da heap, você
    /// terá uma lista ordenada descendente. Quando a MaxHeap é esvaziada, temos uma lista ordenada
    /// "ao contrário", por isso usamos a função reverse para ordenar de forma ascendente.
    pub fn heapsort(mut self) -> Vec<T> {
        let mut sorted = Vec::with_capacity(self.len());

        // Remoção dos elementos da heap.
        while let Some(max) = self.pop() {
            sorted.push(max);
        }

        sorted.reverse();

        sorted
    }
}

/// Transformação de Vec<T> em MaxHeap<T>
impl<T: Default + Ord> From<Vec<T>> for MaxHeap<T> {
    fn from(data: Vec<T>) -> Self {
        let mut heap = MaxHeap { data };
        let len = heap.len();
        for i in (0..len / 2).rev() {
            heap.bubble_down(i);
        }
        heap
    }
}

/// Implementação de inserção e remoção
impl<T: Ord> MaxHeap<T> {
    /// Adiciona o valor ao final da lista e então usa a função subir(bubble_up)
    /// para corrigir a prioridade do novo valor inserido.
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.bubble_up(self.data.len() - 1);
    }

    /// Se a lista está vazia, apenas retorna None.
    /// Se não, troca a posição do último elemento com o primeiro,
    /// retira o novo último da lista e utiliza a função descer(bubble_down)
    /// no novo primeiro elemento.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let last_index = self.len() - 1;
        self.swap(0, last_index);
        let max_value = self.data.pop();
        self.bubble_down(0);
        max_value
    }
}

/// Funções auxiliares
impl<T> MaxHeap<T> {
    /// Busca no topo da MaxHeap
    /// Tem que retornar sempre o valor com maior prioridade
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Verifica se a MaxHeap está vazia
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Verifica o tamanho da heap
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Troca as posições no vetor da MaxHeap
    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }
}

/// Implementação de iterador imutável para a heap
/// Com essa implementação, é possível fazer uma busca desse modo:
///
/// ```
/// use rustic_dsa::data_structures::heap::MaxHeap;
/// use std::ops::Index;
/// let mut x = MaxHeap::default();
/// x.push(3);
/// assert!(*x.index(0) == 3 && x[0] == 3);
/// ```
///
/// A limitação é que você só pode ler o valor, não pode mudar ele.
impl<T> Index<usize> for MaxHeap<T> {
    /// Isso é sintaxe do rust, define qual o tipo que será retornado
    /// no iterador.
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// Implementação de iterador mutável para a heap
/// Similar ao Index, mas possibilita alterar o valor advindo do Index
impl<T> IndexMut<usize> for MaxHeap<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
