use belt_sorter_control::{QueueInputs, SorterQueue, MAX_BELT};
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    println!("Use fixed logic? [y/N]: ");
    io::stdin().read_line(&mut input).unwrap();
    let use_fixed = input.trim().eq_ignore_ascii_case("y");

    let mut queue = if use_fixed {
        SorterQueue::new_fixed()
    } else {
        SorterQueue::new_buggy()
    };

    let mut tick: u64 = 0;
    println!("\nPress Enter for next scan, 'q' to quit.\n");

    loop {
        print!("command [Enter/q]: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        if buf.trim().eq_ignore_ascii_case("q") {
            break;
        }

        tick += 1;
        let mut inputs = QueueInputs::default();

        // same pulse + movement logic as before
        if tick % 3 == 1 {
            inputs.python_task = 3;
        } else {
            inputs.python_task = 0;
        }

        queue.scan(&inputs);

        let active = queue.items.iter().filter(|it| it.active).count() as u8;

        println!("scan {tick}: count = {}/active = {}", queue.count, active);
        print!("  belts:");
        for b in 1..=MAX_BELT {
            let idx = queue.conveyor_item[b];
            if idx >= 0 {
                print!(" {}:{}", b, idx);
            }
        }
        println!();
    }
}
