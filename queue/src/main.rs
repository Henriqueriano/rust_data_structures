mod queue;
use queue::{Queue, QueueTrait};
fn main() {
    let mut q: Queue = Queue::new();
    q.show_data();
}
