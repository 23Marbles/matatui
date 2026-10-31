## Cursor
```rust
struct Cursor {
    sequence_id: SequenceId,
    child_index: Option<(usize, Option<usize>)>
}
```
## Logical
Logical left/right movement is determined by the pointed to sequence. The cursor points to after the index. A child index of `None` points to the start of the sequence. The inner `Option` can only be `Some` when the child index points to a `Text` child.
## Spatial #todo

## Clicking #todo 
Allows the cursor to be changed by clicking anywhere. Uses the root sequence.