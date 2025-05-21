use core::{fmt, hash};
use std::{
    cell::RefCell, hash::{DefaultHasher, Hasher}, marker::PhantomData, rc::Rc
};

pub struct LruCache<K, V> {
    buckets: Vec<Vec<Item<K, V>>>,

    free: usize,
    head: Option<Item<K, V>>,
    tail: Option<Item<K, V>>,
}

unsafe impl<K: Send, V: Send> Send for LruCache<K, V> {}

impl<K, V> LruCache<K, V> {
    pub fn new(n_lru: usize) -> Self {
        assert!(n_lru.count_ones() == 1, "n_lru must be a power of 2: {}", n_lru);

        let mut buckets = Vec::new();
        buckets.resize_with(2 * n_lru, || Vec::new());

        Self {
            buckets,

            free: n_lru,
            head: None,
            tail: None,
        }
    }
}
    
impl<K, V> LruCache<K, V> 
where
    K: Clone + hash::Hash + Eq + fmt::Debug
{
    pub fn get_or_insert(
        &mut self, 
        key: K, 
        mut new: impl FnMut() -> V
    ) -> Entry<K, V> {
        let bucket = self.bucket(&key);

        let pos = self.pos(bucket, &key);

        let pos = match pos {
            Some(pos) => {
                let item = self.buckets[bucket][pos].clone();

                self.update_lru(item);

                pos
            },
            None => {
                let mut pos = self.buckets[bucket].len();
                let item = Item::new(key.clone(), new());
                self.buckets[bucket].push(item.clone());

                if let Some(remove_bucket) = self.insert_lru(item) {
                    if bucket == remove_bucket {
                        pos = self.pos(bucket, &key).unwrap();
                    }
                }

                pos
            },
        };

        Entry {
            cache: self,
            bucket,
            pos
        }
    }

    fn pos(&self, bucket: usize, key: &K) -> Option<usize> {
        self.buckets[bucket].iter().position(|item| &item.0.borrow().key == key)
    }

    fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        let len = self.buckets.len();
        let bucket = (hash & (len as u64 - 1)) as usize;

        bucket
    }

    fn update_lru(&mut self, item: Item<K, V>) {
        let prev = item.0.borrow_mut().prev.take();
        let next = item.0.borrow_mut().next.take();

        match &prev {
            Some(prev) => {
                prev.0.borrow_mut().next = next.clone();
                let head = self.head.replace(item.clone()).unwrap();
                head.0.borrow_mut().prev = Some(item.clone());
                item.0.borrow_mut().next = Some(head);
            },
            None => {
                self.head = Some(item.clone());
            }
        }

        match &next {
            Some(next) => {
                next.0.borrow_mut().prev = prev.clone();
            },
            None => {
                if prev.is_some() {
                    self.tail = prev;
                } else {
                    self.tail = Some(item.clone());
                }
            }
        }
    }

    fn insert_lru(&mut self, item: Item<K, V>) -> Option<usize> {
        let result = if self.free > 0 {
            self.free -= 1;
            None
        } else {
            self.remove_lru()
        };

        let head = self.head.replace(item.clone());

        if let Some(head) = head {
            head.0.borrow_mut().prev = Some(item.clone());
            item.0.borrow_mut().next = Some(head);
        } else {
            self.tail = Some(item);
        }

        result
    }

    fn remove_lru(&mut self) -> Option<usize> {
        let tail = self.tail.take().unwrap();

        let prev = tail.0.borrow_mut().prev.take().unwrap();

        prev.0.borrow_mut().next = None;

        self.tail = Some(prev);

        let key = tail.0.borrow().key.clone();

        let bucket = self.bucket(&key);

        self.buckets[bucket].retain(|item| item.0.borrow().key != key);

        Some(bucket)
    }
}

pub struct Entry<'a, K: 'static, V: 'static> {
    cache: &'a LruCache<K, V>,
    bucket: usize,
    pos: usize,
}

impl<K, V> Entry<'_, K, V> {
    pub fn write<R>(&mut self, f: impl FnOnce(&mut V) -> R) -> R {
        f(&mut self.cache.buckets[self.bucket][self.pos].0.borrow_mut().value)
    }
}

//struct Item<K, V>(Rc<RefCell<ItemInner<K, V>>>);

struct Item<K, V>(Rc<RefCell<ItemInner<K, V>>>);

impl<K, V> Clone for Item<K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K, V> Item<K, V> {
    fn new(key: K, value: V) -> Self {
        Self(Rc::new(RefCell::new(ItemInner {
            key,
            value,
            prev: None,
            next: None,
        })))
    }
}

struct ItemInner<K, V> {
    key: K,
    value: V,

    prev: Option<Item<K, V>>,
    next: Option<Item<K, V>>,
}

#[cfg(test)]
mod test {
    use super::LruCache;

    #[test]
    fn basic_hit() {
        let mut lru = LruCache::<usize, Value>::new(2);

        lru.get_or_insert(1, || Value::new(0))
            .write(|v| { assert_eq!(v.value, 0); v.value += 1; });

        lru.get_or_insert(1, || Value::new(99))
            .write(|v| { assert_eq!(v.value, 1); v.value += 1; });

        lru.get_or_insert(1, || Value::new(99))
            .write(|v| { assert_eq!(v.value, 2); v.value += 1; });

        lru.get_or_insert(1, || Value::new(99))
            .write(|v| { assert_eq!(v.value, 3); v.value += 1; });
    }

    #[test]
    fn basic_lru() {
        let mut lru = LruCache::<usize, Value>::new(2);

        lru.get_or_insert(1, || Value::new(1))
            .write(|v| { assert_eq!(v.value, 1); });

        lru.get_or_insert(2, || Value::new(2))
            .write(|v| { assert_eq!(v.value, 2); });

        lru.get_or_insert(3, || Value::new(3))
            .write(|v| { assert_eq!(v.value, 3); });

        lru.get_or_insert(1, || Value::new(101))
            .write(|v| { assert_eq!(v.value, 101); });

        lru.get_or_insert(3, || Value::new(103))
            .write(|v| { assert_eq!(v.value, 3); });

        lru.get_or_insert(1, || Value::new(201))
            .write(|v| { assert_eq!(v.value, 101); });

        lru.get_or_insert(2, || Value::new(202))
            .write(|v| { assert_eq!(v.value, 202); });
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    struct Value {
        value: usize,
    }

    impl Value {
        fn new(value: usize) -> Self {
            Self {
                value,
            }
        }
    }
}
