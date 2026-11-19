# Symbolic Representation Goals
Should be able to store mathematical values without losing precision. Should also be able to store unknown values (Variables).
# Initial Ideas
Have a map which uses multiple `Slotmap`s to store all of the stored variables, with each variable type getting its own `Slotmap`. Have a visual id (a `String`) and a `VariableId`. These ids will have a transformer that can derive the other one from one of them. The `VariableId` will map to a data id in the form of an enum that can store any id type for the `Slotmap`s or an unknown label to show the variable has no value.
# Types

| Name Of Type       | Purpose                          |
| ------------------ | -------------------------------- |
| `VariableId`       | Stable variable identity         |
| `VisualId`         | Human-readable variable identity |
| `VariableRegistry` | Maps identities bijectively      |
| `VariableValue`    | Represents assignment state      |
| `DataId`           | Type-erased data reference       |
| `DataStore`        | Owns actual values               |

```rust
struct VariableMap {
    registry: VariableRegistry,
    data: DataStore,
}

struct VariableRegistry {
    variables: SlotMap<VariableId, VariableRecord>,
    visual_to_id: HashMap<String, VariableId>,
}

struct VariableRecord {
    visual_id: String,
    value: VariableValue,
}

enum VariableValue {
    Defined(DataId),
    Undefined,
}

enum DataId {
    Scalar(ScalarId),
    Vector(VectorId),
    Matrix(MatrixId),
    Function(FunctionId),
}
```