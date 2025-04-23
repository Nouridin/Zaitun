use std::collections::{HashMap as StdHashMap, HashSet as StdHashSet, VecDeque, BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::fmt;
use std::iter::FromIterator;

/// A dynamically-sized array
pub struct Vector<T> {
    inner: Vec<T>,
}

impl<T> Vector<T> {
    /// Create a new empty vector
    pub fn new() -> Self {
        Vector {
            inner: Vec::new(),
        }
    }
    
    /// Create a new vector with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Vector {
            inner: Vec::with_capacity(capacity),
        }
    }
    
    /// Add an element to the end of the vector
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }
    
    /// Remove and return the last element
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }
    
    /// Get a reference to the element at the specified index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
    
    /// Get a mutable reference to the element at the specified index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }
    
    /// Get the number of elements in the vector
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Remove all elements from the vector
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    /// Insert an element at the specified index
    pub fn insert(&mut self, index: usize, value: T) {
        self.inner.insert(index, value);
    }
    
    /// Remove and return the element at the specified index
    pub fn remove(&mut self, index: usize) -> T {
        self.inner.remove(index)
    }
    
    /// Get an iterator over the elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    
    /// Get a mutable iterator over the elements
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.inner.iter_mut()
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Vector::new()
    }
}

impl<T: Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector {
            inner: self.inner.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Vector {
            inner: Vec::from_iter(iter),
        }
    }
}

/// A hash map implementation
pub struct HashMap<K, V> {
    inner: StdHashMap<K, V>,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    /// Create a new empty hash map
    pub fn new() -> Self {
        HashMap {
            inner: StdHashMap::new(),
        }
    }
    
    /// Create a new hash map with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        HashMap {
            inner: StdHashMap::with_capacity(capacity),
        }
    }
    
    /// Insert a key-value pair into the map
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }
    
    /// Get a reference to the value associated with the key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
    
    /// Get a mutable reference to the value associated with the key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }
    
    /// Remove a key-value pair from the map
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
    
    /// Check if the map contains the specified key
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }
    
    /// Get the number of key-value pairs in the map
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Remove all key-value pairs from the map
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    /// Get an iterator over the key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.inner.iter()
    }
    
    /// Get a mutable iterator over the key-value pairs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.inner.iter_mut()
    }
    
    /// Get an iterator over the keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }
    
    /// Get an iterator over the values
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        HashMap::new()
    }
}

impl<K: Clone + Eq + Hash, V: Clone> Clone for HashMap<K, V> {
    fn clone(&self) -> Self {
        HashMap {
            inner: self.inner.clone(),
        }
    }
}

impl<K: fmt::Debug + Eq + Hash, V: fmt::Debug> fmt::Debug for HashMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// A hash set implementation
pub struct HashSet<T> {
    inner: StdHashSet<T>,
}

impl<T> HashSet<T>
where
    T: Eq + Hash,
{
    /// Create a new empty hash set
    pub fn new() -> Self {
        HashSet {
            inner: StdHashSet::new(),
        }
    }
    
    /// Create a new hash set with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        HashSet {
            inner: StdHashSet::with_capacity(capacity),
        }
    }
    
    /// Insert a value into the set
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }
    
    /// Remove a value from the set
    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    }
    
    /// Check if the set contains the specified value
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }
    
    /// Get the number of values in the set
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Remove all values from the set
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    /// Get an iterator over the values
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}

impl<T> Default for HashSet<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        HashSet::new()
    }
}

impl<T: Clone + Eq + Hash> Clone for HashSet<T> {
    fn clone(&self) -> Self {
        HashSet {
            inner: self.inner.clone(),
        }
    }
}

impl<T: fmt::Debug + Eq + Hash> fmt::Debug for HashSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// A double-ended queue implementation
pub struct Queue<T> {
    inner: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Create a new empty queue
    pub fn new() -> Self {
        Queue {
            inner: VecDeque::new(),
        }
    }
    
    /// Create a new queue with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Queue {
            inner: VecDeque::with_capacity(capacity),
        }
    }
    
    /// Add an element to the back of the queue
    pub fn push_back(&mut self, value: T) {
        self.inner.push_back(value);
    }
    
    /// Add an element to the front of the queue
    pub fn push_front(&mut self, value: T) {
        self.inner.push_front(value);
    }
    
    /// Remove and return the element at the front of the queue
    pub fn pop_front(&mut self) -> Option<T> {
        self.inner.pop_front()
    }
    
    /// Remove and return the element at the back of the queue
    pub fn pop_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    }
    
    /// Get a reference to the element at the front of the queue
    pub fn front(&self) -> Option<&T> {
        self.inner.front()
    }
    
    /// Get a mutable reference to the element at the front of the queue
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.inner.front_mut()
    }
    
    /// Get a reference to the element at the back of the queue
    pub fn back(&self) -> Option<&T> {
        self.inner.back()
    }
    
    /// Get a mutable reference to the element at the back of the queue
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.inner.back_mut()
    }
    
    /// Get the number of elements in the queue
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    /// Remove all elements from the queue
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    
    /// Get an iterator over the elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    
    /// Get a mutable iterator over the elements
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.inner.iter_mut()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Queue::new()
    }
}

impl<T: Clone> Clone for Queue<T> {
    fn clone(&self) -> Self {
        Queue {
            inner: self.inner.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Queue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}