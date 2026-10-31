## Overview 
The tree data structure is the basis for the editor.
```rust
pub struct Tree {
    containers: SlotMap<ContainerId, Container>,
    sequences: SlotMap<SequenceId, Sequence>,
    root: SequenceId,
}
```