//! Memory management and pooling for ARCADIA
//!
//! Provides efficient memory allocation strategies including object pooling,
//! bump allocation, and memory-efficient data structures to reduce heap fragmentation.

use bumpalo::Bump;
use parking_lot::Mutex;
use std::sync::Arc;

/// Memory pool for reusable objects to reduce allocation overhead
pub struct ObjectPool<T> {
    objects: Mutex<Vec<T>>,
    factory: Arc<dyn Fn() -> T + Send + Sync>,
}

impl<T> ObjectPool<T>
where
    T: Send + 'static,
{
    /// Create a new object pool with a factory function
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            objects: Mutex::new(Vec::new()),
            factory: Arc::new(factory),
        }
    }

    /// Get an object from the pool or create a new one
    pub fn acquire(&self) -> PooledObject<T> {
        let obj = self.objects.lock().pop().unwrap_or_else(|| (self.factory)());

        PooledObject {
            obj: Some(obj),
            pool: self,
        }
    }

    /// Return an object to the pool
    fn release(&self, obj: T) {
        let mut objects = self.objects.lock();
        if objects.len() < 1000 {
            // Limit pool size to prevent unbounded growth
            objects.push(obj);
        }
    }

    /// Get current pool size
    pub fn len(&self) -> usize {
        self.objects.lock().len()
    }

    /// Check if pool is empty
    pub fn is_empty(&self) -> bool {
        self.objects.lock().is_empty()
    }
}

/// RAII wrapper for pooled objects that returns them to the pool on drop
pub struct PooledObject<'a, T> {
    obj: Option<T>,
    pool: &'a ObjectPool<T>,
}

impl<'a, T> PooledObject<'a, T> {
    /// Get a reference to the pooled object
    pub fn get(&self) -> &T {
        self.obj.as_ref().unwrap()
    }

    /// Get a mutable reference to the pooled object
    pub fn get_mut(&mut self) -> &mut T {
        self.obj.as_mut().unwrap()
    }
}

impl<'a, T> Drop for PooledObject<'a, T> {
    fn drop(&mut self) {
        if let Some(obj) = self.obj.take() {
            self.pool.release(obj);
        }
    }
}

impl<'a, T> std::ops::Deref for PooledObject<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T> std::ops::DerefMut for PooledObject<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// Bump allocator for temporary allocations with fast deallocation
pub struct BumpAllocator {
    bump: Mutex<Bump>,
}

impl BumpAllocator {
    /// Create a new bump allocator
    pub fn new() -> Self {
        Self {
            bump: Mutex::new(Bump::new()),
        }
    }

    /// Create a bump allocator with a specific capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bump: Mutex::new(Bump::with_capacity(capacity)),
        }
    }

    /// Allocate a value in the bump allocator
    pub fn alloc<T>(&self, value: T) -> &T {
        let bump = self.bump.lock();
        bump.alloc(value)
    }

    /// Reset the allocator, deallocating all memory at once
    pub fn reset(&self) {
        self.bump.lock().reset();
    }

    /// Get allocated bytes
    pub fn allocated_bytes(&self) -> usize {
        self.bump.lock().allocated_bytes()
    }
}

impl Default for BumpAllocator {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory statistics for monitoring
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    /// Total allocated bytes
    pub allocated_bytes: usize,
    /// Total deallocated bytes
    pub deallocated_bytes: usize,
    /// Peak memory usage
    pub peak_bytes: usize,
    /// Current memory usage
    pub current_bytes: usize,
}

impl MemoryStats {
    /// Create new memory statistics
    pub fn new() -> Self {
        Self {
            allocated_bytes: 0,
            deallocated_bytes: 0,
            peak_bytes: 0,
            current_bytes: 0,
        }
    }

    /// Update statistics with a new allocation
    pub fn record_allocation(&mut self, bytes: usize) {
        self.allocated_bytes += bytes;
        self.current_bytes += bytes;
        if self.current_bytes > self.peak_bytes {
            self.peak_bytes = self.current_bytes;
        }
    }

    /// Update statistics with a deallocation
    pub fn record_deallocation(&mut self, bytes: usize) {
        self.deallocated_bytes += bytes;
        self.current_bytes = self.current_bytes.saturating_sub(bytes);
    }

    /// Get memory usage percentage
    pub fn usage_percentage(&self, total_available: usize) -> f64 {
        (self.current_bytes as f64 / total_available as f64) * 100.0
    }
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Global memory manager for tracking allocations across the engine
pub struct MemoryManager {
    stats: Arc<Mutex<MemoryStats>>,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(MemoryStats::new())),
        }
    }

    /// Record an allocation
    pub fn record_allocation(&self, bytes: usize) {
        self.stats.lock().record_allocation(bytes);
    }

    /// Record a deallocation
    pub fn record_deallocation(&self, bytes: usize) {
        self.stats.lock().record_deallocation(bytes);
    }

    /// Get current memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        *self.stats.lock()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        *self.stats.lock() = MemoryStats::new();
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_pool() {
        let pool = ObjectPool::new(|| Vec::<i32>::new());

        {
            let mut obj1 = pool.acquire();
            obj1.push(42);
            assert_eq!(obj1.len(), 1);
        } // obj1 returned to pool

        assert_eq!(pool.len(), 1);

        let obj2 = pool.acquire();
        // Object was reused, but should be cleared in production use
        drop(obj2);
    }

    #[test]
    fn test_bump_allocator() {
        let allocator = BumpAllocator::new();

        let val1 = allocator.alloc(42);
        let val2 = allocator.alloc(100);

        assert_eq!(*val1, 42);
        assert_eq!(*val2, 100);

        let allocated = allocator.allocated_bytes();
        assert!(allocated > 0);

        allocator.reset();
        assert_eq!(allocator.allocated_bytes(), 0);
    }

    #[test]
    fn test_memory_stats() {
        let mut stats = MemoryStats::new();

        stats.record_allocation(1024);
        assert_eq!(stats.current_bytes, 1024);
        assert_eq!(stats.peak_bytes, 1024);

        stats.record_allocation(512);
        assert_eq!(stats.current_bytes, 1536);
        assert_eq!(stats.peak_bytes, 1536);

        stats.record_deallocation(512);
        assert_eq!(stats.current_bytes, 1024);
        assert_eq!(stats.peak_bytes, 1536); // Peak remains
    }

    #[test]
    fn test_memory_manager() {
        let manager = MemoryManager::new();

        manager.record_allocation(2048);
        manager.record_allocation(1024);

        let stats = manager.get_stats();
        assert_eq!(stats.allocated_bytes, 3072);
        assert_eq!(stats.current_bytes, 3072);

        manager.record_deallocation(1024);
        let stats = manager.get_stats();
        assert_eq!(stats.current_bytes, 2048);
    }

    #[test]
    fn test_usage_percentage() {
        let mut stats = MemoryStats::new();
        stats.record_allocation(512);

        let usage = stats.usage_percentage(1024);
        assert_eq!(usage, 50.0);
    }
}
