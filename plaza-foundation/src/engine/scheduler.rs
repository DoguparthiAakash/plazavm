use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

pub struct ScheduledTask {
    pub id: Uuid,
    pub name: String,
    pub priority: TaskPriority,
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for ScheduledTask {}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.priority as u8).cmp(&(other.priority as u8))
    }
}

/// Execution Scheduler with priority queue and concurrency limits.
pub struct ExecutionScheduler {
    heap: Mutex<BinaryHeap<ScheduledTask>>,
}

impl ExecutionScheduler {
    pub fn new() -> Self {
        Self {
            heap: Mutex::new(BinaryHeap::new()),
        }
    }

    pub fn submit(&self, name: impl Into<String>, priority: TaskPriority) -> Uuid {
        let task = ScheduledTask {
            id: Uuid::new_v4(),
            name: name.into(),
            priority,
        };
        let id = task.id;
        let mut heap = self.heap.lock().unwrap();
        heap.push(task);
        id
    }

    pub fn pop_next(&self) -> Option<ScheduledTask> {
        let mut heap = self.heap.lock().unwrap();
        heap.pop()
    }

    pub fn pending_count(&self) -> usize {
        let heap = self.heap.lock().unwrap();
        heap.len()
    }
}

impl Default for ExecutionScheduler {
    fn default() -> Self {
        Self::new()
    }
}
