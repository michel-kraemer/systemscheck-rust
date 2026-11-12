use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Operation {
    machine: u64,
    tool: u64,
    duration: u64,
}

#[derive(PartialEq, Eq)]
struct Event {
    /// The event's timestamp
    t: u64,

    /// The event's kind
    kind: EventKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EventKind {
    /// Start the whole state machine
    Start,

    /// Start an operation
    StartWork { job_id: usize, op: Operation },

    /// Stop an operation
    StopWork { job_id: usize, op: Operation },

    /// A part has returned to the staging area
    PartReturnedToStaging { job_id: usize },

    /// A tool has returned to the staging area
    ToolReturnedToStaging { tool: u64 },
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.t.cmp(&self.t)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let blocks = input.trim().split("\n\n").collect::<Vec<_>>();
    let mut jobs = Vec::new();
    for b in blocks {
        let mut operations = VecDeque::new();
        for l in b.lines().skip(1) {
            let mut parts = l.split_ascii_whitespace().skip(1);
            let machine = parts.next().unwrap()[1..].parse::<u64>().unwrap();
            let tool = parts.next().unwrap()[1..].parse::<u64>().unwrap();
            let duration = parts.next().unwrap()[1..].parse::<u64>().unwrap();
            operations.push_back(Operation {
                machine,
                tool,
                duration,
            });
        }
        jobs.push(operations);
    }

    // part 1
    let mut total = 0;
    for job in &jobs {
        let mut pos = (0u64, 0u64);
        for op in job {
            let machine_x = op.machine % 5;
            let machine_y = op.machine / 5;
            total +=
                (pos.0.abs_diff(machine_x) + pos.1.abs_diff(machine_y)).max(machine_x + machine_y);
            pos = (machine_x, machine_y);
            total += op.duration;
        }
        total += pos.0 + pos.1;
    }
    println!("{total}");

    // part 2 ...

    // which machines, tools, parts are currently in use
    let mut machines_blocked: HashSet<u64> = HashSet::new();
    let mut tools_blocked: HashSet<u64> = HashSet::new();
    let mut parts_blocked: HashSet<usize> = HashSet::new();

    // where the parts are currently
    let mut part_positions: HashMap<usize, (u64, u64)> = HashMap::new();

    // an event queue sorted by time ascending
    let mut queue = BinaryHeap::new();
    queue.push(Event {
        t: 0,
        kind: EventKind::Start,
    });

    let mut max_t = 0;
    while let Some(e) = queue.pop() {
        max_t = e.t;

        match e.kind {
            EventKind::Start => {
                // The whole state machine starts. Just fall through to the
                // scheduler.
            }

            EventKind::StartWork { job_id, op } => {
                // An operation has started. Schedule a stop event and record
                // the part's current position.
                queue.push(Event {
                    t: e.t + op.duration,
                    kind: EventKind::StopWork { job_id, op },
                });
                part_positions.insert(job_id, (op.machine % 5, op.machine / 5));
            }

            EventKind::StopWork { job_id, op } => {
                // An operation has finished. Immediately release the machine
                // and the part and send the tool back to the staging area.
                machines_blocked.remove(&op.machine);
                parts_blocked.remove(&job_id);
                queue.push(Event {
                    t: e.t + op.machine % 5 + op.machine / 5,
                    kind: EventKind::ToolReturnedToStaging { tool: op.tool },
                });
            }

            EventKind::PartReturnedToStaging { job_id } => {
                // A part has returned to the staging area. It can be used
                // again.
                parts_blocked.remove(&job_id);
            }

            EventKind::ToolReturnedToStaging { tool } => {
                // A tool has returned to the staging area. It can be used
                // again.
                tools_blocked.remove(&tool);
            }
        }

        // before we schedule another event, we must process all event from the
        // current timestamp
        if let Some(next) = queue.peek()
            && next.t == e.t
        {
            continue;
        }

        for (i, j) in jobs.iter_mut().enumerate() {
            if parts_blocked.contains(&i) {
                // The part is still in use. We cannot schedule another
                // operation from this job.
                continue;
            }

            if let Some(op) = j.front()
                && !machines_blocked.contains(&op.machine)
                && !tools_blocked.contains(&op.tool)
            {
                // The part is free and the required machine and tool are free.
                // Schedule a new operation.
                machines_blocked.insert(op.machine);
                tools_blocked.insert(op.tool);
                parts_blocked.insert(i);

                let machine_x = op.machine % 5;
                let machine_y = op.machine / 5;

                let part_position = part_positions.get(&i).unwrap_or(&(0, 0));
                let next_t = e.t
                    + (part_position.0.abs_diff(machine_x) + part_position.1.abs_diff(machine_y))
                        .max(machine_x + machine_y);

                queue.push(Event {
                    t: next_t,
                    kind: EventKind::StartWork { job_id: i, op: *op },
                });

                // remove the operation from the job's queue
                j.pop_front();
            } else if let Some(pp) = part_positions.remove(&i) {
                // The part is free but the required machine or tool are not
                // free. Since the tile is still somewhere on the grid, we need
                // to send it back to the staging area before we can continue
                // with it. While it is being transported back, it's blocked.
                parts_blocked.insert(i);
                queue.push(Event {
                    t: e.t + pp.0 + pp.1,
                    kind: EventKind::PartReturnedToStaging { job_id: i },
                });
            }
        }
    }

    println!("{max_t}");
}
