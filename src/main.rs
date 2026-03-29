use std::io::{self, Write};

use belt_sorter_control::{QueueInputs, SorterQueue, MAX_BELT};

fn main() {
    let mut queue = SorterQueue::new_buggy();
    let mut tick: u64 = 0;

    println!("belt-sorter-control interactive scan demo");
    println!("Press Enter to execute one scan, 'q' + Enter to quit.\n");

    loop {
        print!("command [Enter/q]: ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }
        let cmd = buf.trim();
        if cmd.eq_ignore_ascii_case("q") {
            break;
        }

        tick += 1;

        // Build inputs for this scan
        let mut inputs = QueueInputs::default();

        // Example: pulse every 3 ticks: 3, 0, 0, then repeat
        if tick % 3 == 1 {
            inputs.python_task = 3;
        } else {
            inputs.python_task = 0;
        }

        // (movement / exit events will be added later)

        queue.scan(&inputs);

        // Print a compact snapshot
        println!("scan {tick}: count = {}", queue.count);

        print!("  belts:");
        for b in 1..=MAX_BELT {
            let idx = queue.conveyor_item[b];
            if idx >= 0 {
                print!(" {}:{}", b, idx);
            }
        }
        println!();

        let active = queue
            .items
            .iter()
            .enumerate()
            .filter(|(_, it)| it.active)
            .map(|(i, it)| (i, it.task_number, it.position))
            .collect::<Vec<_>>();
        println!("  active items: {:?}", active);
        println!();
    }

    println!("Exiting.");
}
