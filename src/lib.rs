mod lib_test;

mod ring_buffer {
    use std::mem::swap;

    pub struct RingBuffer<T> {
        head: usize,
        tail: usize,
        memory: Vec<Option<T>>,
    }

    impl<T> RingBuffer<T> {
        pub fn with_capacity(capacity: usize) -> RingBuffer<T> {
            let mut memory = Vec::with_capacity(capacity + 1);
            memory.resize_with(capacity + 1, || None);
            RingBuffer { head: 0, tail: 0, memory }
        }

        pub fn push(&mut self, elem: T) -> Result<(), T> {
            self.memory[self.head] = Some(elem);
            self.head += 1;
            if self.head == self.memory.len() { self.head = 0 }
            if self.tail == self.head {
                Err(self.move_tail().expect("Unbelievable"))
            } else {
                Ok(())
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.tail == self.head {
                None
            } else {
                self.move_tail()
            }
        }

        fn move_tail(&mut self) -> Option<T> {
            let mut ret = None;
            swap(&mut self.memory[self.tail], &mut ret);
            self.tail += 1;
            if self.tail == self.memory.len() { self.tail = 0 }
            ret
        }
    }

    pub struct RingBufferIterator<T> {
        buffer: RingBuffer<T>,
    }

    impl<T> IntoIterator for RingBuffer<T> {
        type Item = T;
        type IntoIter = RingBufferIterator<T>;

        fn into_iter(self) -> Self::IntoIter {
            RingBufferIterator { buffer: self }
        }
    }

    impl<T> Iterator for RingBufferIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.buffer.pop()
        }
    }
}
