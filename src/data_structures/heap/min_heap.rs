use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct MinHeap<T> {
    pub data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;

            if self[index] >= self[parent] {
                break;
            }

            self.swap(index, parent);

            index = parent;
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        let last_index = match self.len() {
            0 => 0,
            n => n - 1,
        };

        loop {
            let left_child = (2 * index) + 1;
            let right_child = (2 * index) + 2;
            let mut lowest = index;

            if left_child <= last_index && self[left_child] < self[lowest] {
                lowest = left_child;
            }

            if right_child <= last_index && self[right_child] < self[lowest] {
                lowest = right_child;
            }

            if lowest == index {
                break;
            }

            self.swap(index, lowest);
            index = lowest;
        }
    }
}

impl<T: Ord> MinHeap<T> {
    pub fn heapsort(mut self) -> Vec<T> {
        let mut sorted = Vec::with_capacity(self.len());

        while let Some(max) = self.pop() {
            sorted.push(max);
        }

        sorted
    }
}

/// Transformação de Vec<T> em MaxHeap<T>
impl<T: Default + Ord> From<Vec<T>> for MinHeap<T> {
    fn from(data: Vec<T>) -> Self {
        let mut heap = MinHeap { data };
        let len = heap.len();
        for i in (0..len / 2).rev() {
            heap.bubble_down(i);
        }
        heap
    }
}

/// Implementação de inserção e remoção
impl<T: Ord> MinHeap<T> {
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
impl<T> MinHeap<T> {
    /// Busca no topo da MinHeap
    /// Tem que retornar sempre o valor com menor prioridade
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    /// Verifica se a MinHeap está vazia
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Verifica o tamanho da MinHeap
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Troca as posições no vetor da MinHeap
    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }
}

impl<T> Index<usize> for MinHeap<T> {
    /// Isso é sintaxe do rust, define qual o tipo que será retornado
    /// no iterador.
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// Implementação de iterador mutável para a heap
/// Similar ao Index, mas possibilita alterar o valor advindo do Index
impl<T> IndexMut<usize> for MinHeap<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
