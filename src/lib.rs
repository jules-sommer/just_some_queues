#![feature(exclusive_range_pattern)]

mod queue;
mod unique_queue;

#[allow(unused_imports)] // this import is used in the tests
use crate::unique_queue::{IsUniqueQueue, UniqueQueue};

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Global atomic counter to track allocations
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
    System.alloc(layout)
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    System.dealloc(ptr, layout)
  }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

// Utility function to get the current allocation count
fn allocation_count() -> usize {
  ALLOC_COUNT.load(Ordering::SeqCst)
}
#[cfg(test)]
mod tests {
  use crate::unique_queue::UniqueBufferError;

  use super::*;

  #[test]
  fn test_queue_init() {
    let queue: UniqueQueue<i32> = UniqueQueue::empty();
    assert_eq!(queue.capacity(), 0);

    let queue: UniqueQueue<i32> = UniqueQueue::default();
    assert!(queue.capacity() >= 100);

    let queue: UniqueQueue<i32> = UniqueQueue::with_capacity(10);
    assert!(queue.capacity() >= 10);
  }

  #[test]
  fn queue_with_capacity() {
    // Your test or usage scenario
    println!("Allocations before: {}", super::allocation_count());
    // Perform operations that might allocate...
    let mut queue: UniqueQueue<i32> = UniqueQueue::with_capacity(10);
    assert!(queue.capacity() >= 10); // Note: capacity might be larger due to allocation strategy
    for i in 0..10 {
      match queue.enqueue(i) {
        Ok(_) => {
          assert_eq!(queue.size(), (i + 1).try_into().unwrap());
          assert_eq!(queue.front().unwrap(), 0);
          assert_eq!(queue.back().unwrap(), i);
        }
        Err(_) => panic!("Failed to enqueue {}", i),
      }
    }
    println!("Allocations after: {}", super::allocation_count());
  }

  #[test]
  fn enqueue_and_dequeue() {
    let mut queue = UniqueQueue::with_capacity(10);
    assert!(queue.enqueue(1).is_ok());
    assert!(queue.enqueue(2).is_ok());
    println!("Queue: {:?}", queue);

    assert_eq!(queue.contains(1) && queue.contains(2), true);

    assert_eq!(queue.dequeue().unwrap(), 1);
    assert_eq!(queue.dequeue().unwrap(), 2);
    assert_eq!(queue.size(), 0);

    println!("Queue: {:?}", queue);

    assert_eq!(queue.contains(1) || queue.contains(2), false);
  }

  #[test]
  fn enqueue_duplicate() {
    let mut queue = UniqueQueue::default();
    assert!(queue.enqueue(1).is_ok());
    assert!(matches!(
      queue.enqueue(1).err().unwrap(),
      UniqueBufferError::EnqueuedDuplicateValue
    ));
  }

  #[test]
  fn dequeue_from_empty() {
    let mut queue: UniqueQueue<i32> = UniqueQueue::default();
    assert!(queue.dequeue().is_err());
  }

  #[test]
  fn peek_elements() {
    let mut queue = UniqueQueue::default();
    queue.enqueue(1).unwrap();
    queue.enqueue(2).unwrap();
    assert_eq!(queue.front().unwrap(), 1);
    assert_eq!(queue.back().unwrap(), 2);
  }

  #[test]
  fn peek_empty() {
    let mut queue: UniqueQueue<i32> = UniqueQueue::default();
    assert!(queue.front().is_err());
    assert!(queue.back().is_err());
  }
}
