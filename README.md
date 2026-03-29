# belt-sorter-control

A Rust model of queue logic for a multi-belt sorting system, built to reproduce a long-run control bug and verify a fix with deterministic tests. The project uses `cargo test` to compare a buggy implementation against a corrected one under the same simulated load .

## Overview

This repository focuses on the part of a sorter that tracks items, assigns destinations, advances items between belts, and removes them once dropped. The goal is not to emulate a full PLC runtime, but to create a behaviorally equivalent state-machine model that makes the bug easy to reproduce, explain, and test .

The model is intentionally small and deterministic. That makes it useful both as a debugging tool and as a foundation for later embedded targets, where the same core logic can be run on a microcontroller with minimal changes .

## Problem

The original control issue appeared only after extended operation: sorting gradually stopped even though the conveyors continued running. In the buggy queue implementation, a new item could overwrite the slot representing the first tracked belt before the previous item had advanced, creating orphaned active items and allowing the queue count to drift away from reality .

Once the tracked count saturated, further insertions were blocked even though the system no longer held that many real items. At that point, downstream logic no longer received valid destination assignments, so no belt ever entered its drop/reverse path .

## Fix

The corrected implementation applies two changes:

1. Guard insertion so a new item is not placed onto belt 2 while belt 2 is still occupied.
2. Recompute `count` from active item state each scan instead of maintaining it manually across multiple code paths.

Together, these changes remove the overwrite mechanism and eliminate count drift. The test harness exists to show that the buggy version can fail under load while the fixed version remains stable under the same stimulus .

## Project structure

- `src/lib.rs` — queue model, state types, and scan logic.
- `tests/` — integration tests that drive the queue with simulated task pulses and belt events.
- Optional future direction: re-use the same core logic in an Embedded Rust target after validating behavior on the PC side first .

## Running tests

```bash
cargo test
