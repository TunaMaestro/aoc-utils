use std::{
    array,
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait Element: Copy + Clone + Hash + Eq {}
impl<T: Copy + Clone + Hash + Eq> Element for T {}

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Node<T: Element> {
    value: T,
    priority: usize,
}

/// A queue where the maximum priority is strictly less than N
pub struct BucketQueue<T: Element, const N: usize> {
    inner: [Bucket<T>; N],
    priorities: HashMap<T, usize>,
}

impl<T: Element, const N: usize> BucketQueue<T, N> {
    pub fn create(init: HashMap<T, usize>) -> BucketQueue<T, N> {
        let mut inner = array::from_fn(|_priority| Bucket::create());
        for (&t, &v) in init.iter() {
            assert!(v < N);

            inner[v].insert(t);
        }
        let new = BucketQueue {
            inner: inner,
            priorities: init,
        };

        new
    }

    pub fn modify_key(&mut self, item: T, to: usize) {
        let current_priority = *match self.priorities.get(&item) {
            Some(x) => x,
            None => return,
        };

        self.inner[current_priority].remove(item);
        self.inner[to].insert(item);
        self.priorities.insert(item, to);
    }

    pub fn pop_min(&mut self) -> Option<Node<T>> {
        for (p, bucket) in self.inner.iter_mut().enumerate() {
            if let Some(x) = bucket.items.iter().next().copied() {
                bucket.remove(x);
                return Some(Node {
                    value: x,
                    priority: p,
                });
            }
        }
        None
    }
}

pub struct Bucket<T: Element> {
    items: HashSet<T>,
}

impl<T: Element> Bucket<T> {
    fn create() -> Self {
        Bucket {
            items: HashSet::new(),
        }
    }

    fn insert(&mut self, item: T) {
        self.items.insert(item);
    }

    fn remove(&mut self, item: T) {
        self.items.remove(&item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_queue() -> BucketQueue<&'static str, { 8 + 1 }> {
        let items = HashMap::from([
            ("apple", 1),
            ("granny smith", 1),
            ("stick", 1),
            ("banana", 3),
            ("kiwi", 4),
            ("mango", 5),
            ("orange", 8),
        ]);

        BucketQueue::create(items)
    }

    #[test]
    fn test_basic() {
        let mut queue = create_queue();

        let x = queue.pop_min();
        assert!(x.is_some());
        let x = x.unwrap();
        assert!(x.priority == 1);
        let value = x.value;
        assert!(["apple", "granny smith", "stick"].contains(&value));

        queue.modify_key("orange", 0);
        let x = queue.pop_min().expect("get new lowest which is orange");

        assert_eq!(x.value, "orange");
        assert_eq!(x.priority, 0);
    }

    #[test]
    fn test_pop_order() {
        let mut queue = create_queue();

        let mut one_set = HashSet::new();
        for _ in 0..3 {
            one_set.insert(
                queue
                    .pop_min()
                    .expect("there should be three 1 elements")
                    .value,
            );
        }
        assert_eq!(one_set, HashSet::from(["apple", "granny smith", "stick"]));

        assert_eq!(queue.pop_min().unwrap().value, "banana");
        assert_eq!(queue.pop_min().unwrap().value, "kiwi");
        assert_eq!(queue.pop_min().unwrap().value, "mango");
        assert_eq!(queue.pop_min().unwrap().value, "orange");
    }
}
