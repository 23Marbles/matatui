use slotmap::new_key_type;

new_key_type! {
    pub struct SymbolId;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolicIdent {
    visual_display: String,
    stable_id: SymbolId,
}
