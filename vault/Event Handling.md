## Editor
```rust
struct Editor {
    cursor: Cursor,
    tree: Tree,
}
```
The editor first attempts getting the parent container of the cursor's `SequenceId`. If there is a parent then `ContainerMut::handle_event` is called on it. If there is no parent or if `ContainerMut::handle_event` returns `Some(<event>)` then then the cursor's `SequenceId` has  `SequenceMut::handle_event` called.
## Container
```rust
struct ContainerMut(ContainerId, &mut Tree);

fn handle_event(ContainerMut, &mut Cursor, Event) -> Option<Event>;
```
The container can choose whether to handle the event or not. It can also choose whether the event should be handled by the sequence pointed at by returning `Some(<event>)`.
## Sequence
```rust
struct SequenceMut(SequenceId, &mut Tree);

fn handle_event(SequenceMut, &mut Cursor, Event);
```
The sequence must handle the `Event`.