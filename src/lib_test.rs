#[cfg(test)]
mod lib_test {
    use crate::ring_buffer::RingBuffer;

    #[test]
    fn test_simple() {
        let mut buffer = RingBuffer::with_capacity(10);
        for i in 0..10 {
            assert_eq!(Result::Ok(()), buffer.push(i));
        }
        for i in 10..20 {
            assert_eq!(Result::Err(i - 10), buffer.push(i));
        }
        for i in 20..30 {
            assert_eq!(Option::Some(i - 10), buffer.pop());
        }
        assert_eq!(None, buffer.pop());
    }

    #[test]
    fn test_iterator() {
        let mut buffer = RingBuffer::with_capacity(10);
        for i in 0..10 {
            assert_eq!(Result::Ok(()), buffer.push(i));
        }
        let mut j = 0;
        for i in buffer {
            assert_eq!(i, j);
            j += 1;
        }
        let mut buffer = RingBuffer::with_capacity(10);
        for i in 0..10 {
            assert_eq!(Result::Ok(()), buffer.push(i));
        }
        assert_eq!(buffer.into_iter().sum::<i32>(), 45);
    }
}