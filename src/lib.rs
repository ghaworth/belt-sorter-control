/// belt-sorter-control
///
/// A small Rust model of queue logic for a multi-belt conveyor sorter.
/// The goal is to reproduce a control-logic bug (and its fix) using `cargo test`
/// with a deterministic state-machine model.

pub const MAX_ITEMS: usize = 10;
pub const MIN_BELT: usize = 1;
pub const MAX_BELT: usize = 11;

/// One tracked item in the sorter queue.
#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub active: bool,
    pub task_number: u8,
    pub position: u8,
    pub drop_done: bool,
}

/// Queue state for the sorter.
///
/// This is deliberately minimal for now; more fields will be added as we port
/// the original SCL logic.
#[derive(Debug)]
pub struct SorterQueue {
    pub items: [Item; MAX_ITEMS],
    pub conveyor_item: [i8; MAX_BELT + 1], // 0 unused, 1..11 used
    pub write_index: u8,
    pub count: u8,
    pub staged_task: u8,
    pub python_task: u8,
    pub last_trigger: bool,
    pub bug_fixed: bool,
}

impl SorterQueue {
    /// Construct the buggy variant of the queue logic.
    pub fn new_buggy() -> Self {
        Self::new(false)
    }

    /// Construct the corrected variant of the queue logic.
    pub fn new_fixed() -> Self {
        Self::new(true)
    }

    fn new(bug_fixed: bool) -> Self {
        Self {
            items: [Item {
                active: false,
                task_number: 0,
                position: 0,
                drop_done: false,
            }; MAX_ITEMS],
            conveyor_item: [-1; MAX_BELT + 1],
            write_index: 0,
            count: 0,
            staged_task: 0,
            python_task: 0,
            last_trigger: false,
            bug_fixed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_initialise_empty_state() {
        let q_buggy = SorterQueue::new_buggy();
        let q_fixed = SorterQueue::new_fixed();

        assert_eq!(q_buggy.count, 0);
        assert_eq!(q_fixed.count, 0);

        for item in q_buggy.items.iter() {
            assert!(!item.active);
            assert_eq!(item.task_number, 0);
            assert_eq!(item.position, 0);
            assert!(!item.drop_done);
        }

        assert!(q_buggy.conveyor_item[1..=MAX_BELT].iter().all(|&x| x == -1));
        assert!(q_fixed.conveyor_item[1..=MAX_BELT].iter().all(|&x| x == -1));
    }
}
