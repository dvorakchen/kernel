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

    pub(crate) fn schedule(&mut self) {
        if self.tasks.is_empty() {
            return;
        }

        let current = self.current_task;

        let next = (current + 1) % self.tasks.len();
        if next != current {
            if let Some(cur_task) = self.tasks.iter_mut().nth(self.current_task) {
                cur_task.status = TaskStatus::Ready;
            }

            if let Some(next_task) = self.tasks.iter_mut().nth(next) {
                next_task.status = TaskStatus::Running;
            }

            unsafe extern "C" {
                fn __switch(old: *mut TaskContext, next: *const TaskContext);
            }

            unsafe {
                let cur_task = &mut self.tasks[current].ctx as *mut TaskContext;
                let next_task = &mut self.tasks[next].ctx as *mut TaskContext;
                self.current_task = next;
                __switch(cur_task, next_task);
            }
        }
    }
}
