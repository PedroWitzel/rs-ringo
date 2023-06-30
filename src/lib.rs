#[derive(Debug)]
struct RingBuffer<T: Clone> {
    storage: Vec<Option<T>>,
    read_idx: usize,
    write_idx: usize,
}

#[derive(Debug)]
struct Full;

impl<T: Clone> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        RingBuffer {
            storage: vec![None; capacity],
            read_idx: 0,
            write_idx: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.write_idx == self.read_idx && self.storage[self.write_idx].is_none()
    }

    fn is_full(&self) -> bool {
        self.write_idx == self.read_idx && self.storage[self.write_idx].is_some()
    }

    fn next_index(&self, index: usize) -> usize {
        (index + 1) % self.storage.len()
    }

    fn push(&mut self, item: T) -> Result<(), Full> {
        if self.is_full() {
            Err(Full)
        } else {
            self.storage[self.write_idx] = Some(item);
            self.write_idx = self.next_index(self.write_idx);
            Ok(())
        }
    }

    fn pull(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            if let Some(item) = self.storage[self.read_idx].take() {
                self.read_idx = self.next_index(self.read_idx);
                Some(item)
            } else {
                panic!("Internal error - there should be value here")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newly_create_buffer_is_empty() {
        let mut r = RingBuffer::<i32>::new(5);
        assert!(r.is_empty());
    }

    #[test]
    fn push_one_object() {
        let mut r = RingBuffer::<i32>::new(5);
        match r.push(42) {
            Ok(_) => assert!(!r.is_empty()),
            Err(Full) => assert!(false),
        }
    }

    #[test]
    fn push_pull_one_object() {
        let mut r = RingBuffer::<i32>::new(5);
        let _ = r.push(1312);
        match r.pull() {
            Some(item) => {
                assert_eq!(item, 1312);
                assert!(r.is_empty())
            }
            None => assert!(false),
        }
    }

    #[test]
    fn push_all_storage() {
        let mut r = RingBuffer::<i32>::new(5);
        let _ = r.push(13);
        let _ = r.push(12);
        let _ = r.push(13);
        let _ = r.push(12);
        match r.push(666) {
            Ok(_) => assert!(true),
            Err(Full) => assert!(false),
        }
        match r.push(42) {
            Ok(_) => assert!(false),
            Err(Full) => assert!(r.is_full()),
        }
    }

    #[test]
    fn ring_it_once() {
        let mut r = RingBuffer::<i32>::new(5);
        let _ = r.push(13);
        let _ = r.push(12);
        let _ = r.push(42);
        let _ = r.push(64);
        match r.push(666) {
            Ok(_) => assert!(true),
            Err(Full) => assert!(false),
        }

        assert!(r.is_full());

        match r.pull() {
            Some(item) => assert_eq!(item, 13),
            None => assert!(false),
        }
        match r.pull() {
            Some(item) => assert_eq!(item, 12),
            None => assert!(false),
        }

        assert!(!r.is_full());
        assert!(!r.is_empty());

        let _ = r.push(7);
        match r.pull() {
            Some(item) => assert_eq!(item, 42),
            None => assert!(false),
        }
        match r.pull() {
            Some(item) => assert_eq!(item, 64),
            None => assert!(false),
        }
        match r.pull() {
            Some(item) => assert_eq!(item, 666),
            None => assert!(false),
        }
        match r.pull() {
            Some(item) => assert_eq!(item, 7),
            None => assert!(false),
        }
        assert!(r.is_empty());
    }
}
