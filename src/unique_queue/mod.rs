mod types;
use linked_hash_set::LinkedHashSet;
use std::hash::Hash;
pub use types::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniqueQueue<T: Clone + Eq + Hash> {
  queue: LinkedHashSet<T>,
}

impl<T: Clone + Eq + Hash> UniqueQueue<T> {
  pub fn with_capacity(capacity: usize) -> UniqueQueue<T> {
    println!("Creating a new buffer with capacity: {}", capacity);
    UniqueQueue {
      queue: LinkedHashSet::with_capacity(capacity),
    }
  }

  pub fn empty() -> UniqueQueue<T> {
    UniqueQueue {
      queue: LinkedHashSet::new(),
    }
  }

  pub fn default() -> UniqueQueue<T> {
    UniqueQueue {
      queue: LinkedHashSet::with_capacity(100),
    }
  }

  pub fn capacity(&self) -> usize {
    self.queue.capacity()
  }
}

impl<T: Clone + Eq + Hash> IsUniqueQueue<T> for UniqueQueue<T> {
  fn enqueue(&mut self, val: T) -> Result<T, UniqueBufferError> {
    // Check if the buffer is full, early return if it is
    println!("Queue size: {}", self.queue.len());
    println!("Queue capacity: {}", self.queue.capacity());
    if self.queue.len() >= self.queue.capacity() {
      return Err(UniqueBufferError::EnqueuedFullBuffer.into());
    }
    // Check if the value is already in the buffer, early return if it is
    if self.queue.contains(&val) {
      return Err(UniqueBufferError::EnqueuedDuplicateValue.into());
    }
    // Add the value to the buffer, cloning so that we can return the original
    self.queue.insert(val.clone());
    Ok(val)
  }

  fn dequeue(&mut self) -> anyhow::Result<T, UniqueBufferError> {
    if self.queue.is_empty() {
      return Err(UniqueBufferError::DequeuedEmptyBuffer);
    }
    Ok(self.queue.pop_front().unwrap())
  }

  fn front(&self) -> Result<T, UniqueBufferError> {
    match self.queue.front() {
      Some(val) => Ok(val.clone()),
      None => Err(UniqueBufferError::PeekedEmptyBuffer),
    }
  }

  fn back(&mut self) -> Result<T, UniqueBufferError> {
    match self.queue.back() {
      Some(val) => Ok(val.clone()),
      None => Err(UniqueBufferError::PeekedEmptyBuffer),
    }
  }

  fn contains(&self, val: T) -> bool {
    self.queue.contains(&val)
  }

  fn capacity(&self) -> usize {
    self.queue.capacity()
  }

  fn clear(&mut self) {
    self.queue.clear();
  }

  fn size(&self) -> usize {
    self.queue.len()
  }
}

impl<T> IntoIterator for UniqueQueue<T>
where
  T: Clone + Eq + Hash,
{
  type Item = T;
  type IntoIter = linked_hash_set::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.queue.into_iter()
  }
}

impl<'a, T> IntoIterator for &'a UniqueQueue<T>
where
  T: Clone + Eq + Hash + 'a,
{
  type Item = &'a T;
  type IntoIter = linked_hash_set::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.queue.iter()
  }
}
