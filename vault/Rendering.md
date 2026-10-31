## Render Engine

The context is passed in with things like defined functions and variables, font, colours, etc...

```rust
trait Renderer<E: LayoutEngine> {
    type Rect = Rect<E::Unit>;
    type Canvas; // e.g. &mut ratatui::Buffer
    
    fn render_layout(MeasurementResult<E>, Self::Rect, Self::Canvas, Context<'_>);
    
    fn render_sequence(SequenceRef, &Cursor, Self::Rect, Self::Canvas, Context<'_>) {
        // default impl
    }
    fn render_container(ContainerRef, &Cursor, Self::Rect, Self::Canvas, Context<'_>) {
        // default impl
    }
}
```
