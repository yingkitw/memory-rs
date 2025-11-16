//! Batch operations for memory

use crate::Result;
use super::MemoryItem;

/// Batch operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchOpType {
    /// Add memory
    Add,
    /// Update memory
    Update,
    /// Delete memory
    Delete,
}

/// Batch operation
#[derive(Debug, Clone)]
pub struct BatchOp {
    /// Operation type
    pub op_type: BatchOpType,
    /// Memory ID
    pub memory_id: String,
    /// Content (for Add/Update)
    pub content: Option<String>,
    /// Memory type (for Add)
    pub memory_type: Option<String>,
}

impl BatchOp {
    /// Create an add operation
    pub fn add(memory_id: String, content: String, memory_type: String) -> Self {
        Self {
            op_type: BatchOpType::Add,
            memory_id,
            content: Some(content),
            memory_type: Some(memory_type),
        }
    }

    /// Create an update operation
    pub fn update(memory_id: String, content: String) -> Self {
        Self {
            op_type: BatchOpType::Update,
            memory_id,
            content: Some(content),
            memory_type: None,
        }
    }

    /// Create a delete operation
    pub fn delete(memory_id: String) -> Self {
        Self {
            op_type: BatchOpType::Delete,
            memory_id,
            content: None,
            memory_type: None,
        }
    }
}

/// Batch result
#[derive(Debug, Clone)]
pub struct BatchResult {
    /// Total operations
    pub total: usize,
    /// Successful operations
    pub successful: usize,
    /// Failed operations
    pub failed: usize,
    /// Error messages
    pub errors: Vec<String>,
}

impl BatchResult {
    /// Create a new batch result
    pub fn new(total: usize) -> Self {
        Self {
            total,
            successful: 0,
            failed: 0,
            errors: Vec::new(),
        }
    }

    /// Mark operation as successful
    pub fn add_success(&mut self) {
        self.successful += 1;
    }

    /// Mark operation as failed
    pub fn add_error(&mut self, error: String) {
        self.failed += 1;
        self.errors.push(error);
    }

    /// Check if all operations succeeded
    pub fn all_succeeded(&self) -> bool {
        self.failed == 0
    }

    /// Get success rate
    pub fn success_rate(&self) -> f32 {
        if self.total == 0 {
            return 1.0;
        }
        self.successful as f32 / self.total as f32
    }
}

/// Batch processor
pub struct BatchProcessor {
    /// Batch size
    pub batch_size: usize,
    /// Continue on error
    pub continue_on_error: bool,
}

impl BatchProcessor {
    /// Create a new batch processor
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size,
            continue_on_error: true,
        }
    }

    /// Split operations into batches
    pub fn split_into_batches(&self, ops: Vec<BatchOp>) -> Vec<Vec<BatchOp>> {
        ops.chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// Get optimal batch size based on operation count
    pub fn optimize_batch_size(&self, op_count: usize) -> usize {
        if op_count < 10 {
            op_count
        } else if op_count < 100 {
            10
        } else if op_count < 1000 {
            32
        } else {
            64
        }
    }
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new(32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_op_add() {
        let op = BatchOp::add(
            "id_1".to_string(),
            "content".to_string(),
            "fact".to_string(),
        );
        assert_eq!(op.op_type, BatchOpType::Add);
        assert_eq!(op.content, Some("content".to_string()));
    }

    #[test]
    fn test_batch_op_update() {
        let op = BatchOp::update("id_1".to_string(), "new_content".to_string());
        assert_eq!(op.op_type, BatchOpType::Update);
        assert_eq!(op.content, Some("new_content".to_string()));
    }

    #[test]
    fn test_batch_op_delete() {
        let op = BatchOp::delete("id_1".to_string());
        assert_eq!(op.op_type, BatchOpType::Delete);
        assert_eq!(op.content, None);
    }

    #[test]
    fn test_batch_result() {
        let mut result = BatchResult::new(10);
        result.add_success();
        result.add_success();
        result.add_error("error".to_string());

        assert_eq!(result.successful, 2);
        assert_eq!(result.failed, 1);
        assert!(!result.all_succeeded());
        assert!(result.success_rate() > 0.1);
    }

    #[test]
    fn test_split_into_batches() {
        let processor = BatchProcessor::new(3);
        let ops = vec![
            BatchOp::add("1".to_string(), "c1".to_string(), "t1".to_string()),
            BatchOp::add("2".to_string(), "c2".to_string(), "t2".to_string()),
            BatchOp::add("3".to_string(), "c3".to_string(), "t3".to_string()),
            BatchOp::add("4".to_string(), "c4".to_string(), "t4".to_string()),
        ];

        let batches = processor.split_into_batches(ops);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].len(), 3);
        assert_eq!(batches[1].len(), 1);
    }

    #[test]
    fn test_optimize_batch_size() {
        let processor = BatchProcessor::new(32);
        assert_eq!(processor.optimize_batch_size(5), 5);
        assert_eq!(processor.optimize_batch_size(50), 10);
        assert_eq!(processor.optimize_batch_size(500), 32);
        assert_eq!(processor.optimize_batch_size(5000), 64);
    }
}
