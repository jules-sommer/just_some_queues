#![allow(dead_code)]

use std::hash::Hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UniqueBufferError {
  #[error("Failed to dequeue: buffer contains no elements")]
  DequeuedEmptyBuffer,
  #[error("Failed to peek the head of the queue, buffer is empty")]
  PeekedEmptyBuffer,
  #[error("Failed to enqueue: buffer is full")]
  EnqueuedFullBuffer,
  #[error("Failed to enqueue: value already in buffer")]
  EnqueuedDuplicateValue,
}

pub trait IsUniqueQueue<T: Clone + Eq + Hash> {
  fn enqueue(&mut self, val: T) -> Result<T, UniqueBufferError>;
  fn dequeue(&mut self) -> Result<T, UniqueBufferError>;
  fn front(&self) -> Result<T, UniqueBufferError>;
  fn back(&mut self) -> Result<T, UniqueBufferError>;
  fn clear(&mut self);
  fn contains(&self, val: T) -> bool;
  fn capacity(&self) -> usize;
  fn size(&self) -> usize;
}
