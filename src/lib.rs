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

pub struct QueueInputs {
    pub python_task: u8,                  // 0 or 2..=11
    pub entry_rise: [bool; MAX_BELT + 1], // 1..=11 used
    pub exit_fall: [bool; MAX_BELT + 1],  // 2..=11 used
}

impl Default for QueueInputs {
    fn default() -> Self {
        Self {
            python_task: 0,
            entry_rise: [false; MAX_BELT + 1],
            exit_fall: [false; MAX_BELT + 1],
        }
    }
}

impl SorterQueue {
    /// One "scan" of the queue logic.
    pub fn scan(&mut self, inputs: &QueueInputs) {
        self.python_task = inputs.python_task;

        if self.bug_fixed {
            // for now, fixed == buggy; we’ll diverge later
            self.section1_python_input_buggy();
        } else {
            self.section1_python_input_buggy();
        }
    }
}

impl SorterQueue {
    fn section1_python_input_buggy(&mut self) {
        // IF (#PythonTask >= 2 AND #PythonTask <= 11) THEN
        //     #StagedTask := #PythonTask;
        // END_IF;
        if (2..=11).contains(&self.python_task) {
            self.staged_task = self.python_task;
        }

        // #TriggerNow := (#PythonTask = 0) AND (NOT #LastTrigger);
        let trigger_now = self.python_task == 0 && !self.last_trigger;
        // #LastTrigger := (#PythonTask = 0);
        self.last_trigger = self.python_task == 0;

        // IF #TriggerNow AND (#StagedTask >= 2 AND #StagedTask <= 11) THEN
        if trigger_now && (2..=11).contains(&self.staged_task) {
            //     IF #Count < 10 THEN
            if self.count < 10 {
                let idx = self.write_index as usize;

                //         #Item[#WriteIndex].Active := TRUE;
                //         #Item[#WriteIndex].TaskNumber := #StagedTask;
                //         #Item[#WriteIndex].Position := 2;
                //         #Item[#WriteIndex].DropDone := FALSE;
                self.items[idx].active = true;
                self.items[idx].task_number = self.staged_task;
                self.items[idx].position = 2;
                self.items[idx].drop_done = false;

                //         #ConveyorItem[2] := #WriteIndex;
                self.conveyor_item[2] = self.write_index as i8;

                //         #StagedTask := 0;
                self.staged_task = 0;

                //         #WriteIndex := (#WriteIndex + 1) MOD 10;
                self.write_index = (self.write_index + 1) % MAX_ITEMS as u8;

                //         #Count := #Count + 1;
                self.count += 1;
            }
            //     END_IF;
        }
        // END_IF;
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

    #[test]
    fn section1_inserts_items_on_python_pulses() {
        let mut q = SorterQueue::new_buggy();

        // Simulate a few pulses of python_task = 3 then back to 0
        for _ in 0..5 {
            let mut inp = QueueInputs::default();
            inp.python_task = 3;
            q.scan(&inp); // stage

            inp.python_task = 0;
            q.scan(&inp); // trigger insert
        }

        assert!(q.count > 0);
        assert!(q.count <= MAX_ITEMS as u8);
        assert!(q.items.iter().any(|it| it.active));
        assert_eq!(q.conveyor_item[2], (q.count - 1) as i8);
    }
}
