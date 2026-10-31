## Engine 

```rust
trait FontLayout<U> {
    fn measure_str(font_name: impl AsRef<str>, &str) -> Measurement<U>;
}

trait LayoutEngine {
    type Unit;
    type FontLayout: FontLayout<Self::Unit>;
    
    fn layout_sequence(SequenceRef, &Cursor) -> MeasurementResult<Self>;
    fn layout_container(ContainerRef, &Cursor) -> MeasurementResult;
}
```

All measurements are taken from the top left and are positive integers.

```rust
struct Measurement<U> {
    ascent: U,
    descent: U,
    width: U,
}

struct PositionedNodeRef<E: LayoutEngine> {
    measurement: Measurement<E::Unit>,
    x: E::Unit,
    y: E::Unit,
    kind: NodeKindRef<E>,
}

enum NodeKindRef<E: LayoutEngine> {
    Sequence(Vec<PositionedNodeRef<E>>),
    Text(&str),
    Operator(char),
    ... // Container node kinds
}
```

There are owned versions of all of these for displaying not currently being edited expressions without storing all associated data such as the tree.

```rust
struct MeasurementResult<E: LayoutEngine> {
    measurement: Measurement<E::Unit>,
    kind: NodeKindRef<E>,
    cursor_pos: Option<(E::Unit, E::Unit, E::Unit)> // (x, y, height)
}
```