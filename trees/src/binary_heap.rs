pub struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        // add to end and bubble up
        self.data.push(value);
        let mut current = self.data.len() - 1;

        while current > 0 {
            let parent = (current - 1) / 2;
            if self.data[parent] > self.data[current] {
                self.data.swap(parent, current);
                current = parent;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }

        // min is at the top, swap with last and bubble down
        let result = Some(self.data[0]);
        let last = self.data.pop().unwrap();

        if !self.data.is_empty() {
            self.data[0] = last;
            let mut current = 0;

            while current < self.data.len() {
                let left_child = 2 * current + 1;
                let right_child = 2 * current + 2;
                let mut smallest = current;

                if left_child < self.data.len() && self.data[left_child] < self.data[smallest] {
                    smallest = left_child;
                }

                if right_child < self.data.len() && self.data[right_child] < self.data[smallest] {
                    smallest = right_child;
                }

                if smallest != current {
                    self.data.swap(current, smallest);
                    current = smallest;
                } else {
                    break;
                }
            }
        }

        result
    }
}
