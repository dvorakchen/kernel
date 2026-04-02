//! task.rs
//! 任务，表示进程

pub(crate) mod context;

use alloc::vec::Vec;

use crate::task::context::TaskContext;

/// 任务的状态
pub(crate) enum TaskStatus {
    Ready,
    Running,
    Exited,
}

pub(crate) type TaskId = usize;

/// 任务控制上下文，相当于 TCB
pub(crate) struct TaskControlBlock {
    /// 任务 id
    pub id: TaskId,
    /// 任务状态
    pub status: TaskStatus,
    /// 任务上下文
    pub ctx: TaskContext,
}

pub(crate) struct TaskManager {
    pub tasks: Vec<TaskControlBlock>,
    pub current_task: usize,
}

impl TaskManager {
    pub(crate) fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current_task: 0,
        }
    }
}
