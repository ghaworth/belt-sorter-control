# belt-sorter-control

A Rust model of queue logic for a multi-belt conveyor sorter. It reproduces a long-run control bug and verifies a fix using `cargo test` and a small CLI “scan stepper”.

## What this project shows

- A queue implementation that tracks:
  - up to 10 items,
  - their destination belt,
  - their current belt,
  - and which belt holds which item index.
- A **buggy** version where the queue length (`count`) is maintained manually across multiple code paths.
- A **fixed** version where `count` is derived from the actual active items each scan, with an extra guard on inserting onto belt 2 while it’s still occupied.

Under a simple simulated load:

- The buggy logic can enter a state where `count == 10` but only 2 items are actually active, and it stays stuck there.
- The fixed logic, driven by the same pattern, keeps `count` equal to the number of active items and avoids that lock-up.

This mirrors a real-world symptom: after extended operation, the sorter stopped sorting even though the conveyors still ran.

## How it works

Core pieces:

- `SorterQueue` — state of the queue and belt mapping.
- `QueueInputs` — per-scan inputs (`python_task`, `entry_rise`, `exit_fall`).
- Buggy path:
  - manual `count += 1` on insert,
  - manual `count -= 1` on various removal paths.
- Fixed path:
  - no manual `count` updates,
  - `count` recomputed from `Item.active` each scan,
  - plus an insertion guard on belt 2 to avoid overwriting an existing item.

There is also a tiny movement rule (belt 2 → belt 3) to create contention, enough to drive the buggy implementation into the failure state.

## Tests

Key tests in `src/lib.rs`:

- Basic construction and insert/remove behaviour.
- `buggy_can_lock_up_while_fixed_stays_consistent`:
  - Drives both variants with the same synthetic “task pulse” pattern.
  - Asserts that the buggy version can reach `count == 10` with fewer than 10 active items.
  - Asserts that the fixed version always has `count == active_items`.

Run all tests with:

```bash
cargo test
