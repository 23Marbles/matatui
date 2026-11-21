use super::*;

mod event_handling;
mod mutable;
mod reference;

macro_rules! container_kind {
    ($($func_name:ident([$count:literal]) -> $variant_name:ident {$($sequence_var_name:ident,)* };)*) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum ContainerKind {
            $(
                $variant_name {
                    $(
                        $sequence_var_name: SequenceId,
                    )*
                },
            )*
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum OwnedContainerKind {
            $(
                $variant_name {
                    $(
                        $sequence_var_name: OwnedSequence,
                    )*
                },
            )*
        }


        impl ContainerKind {
            $(
                pub fn $func_name(seq_ids: [SequenceId; $count]) -> Self {
                    let mut ids = seq_ids.into_iter();

                    Self::$variant_name {
                        $(
                            $sequence_var_name: ids.next()
                                .expect(
                                    stringify!(Supplied count should match number of variables in macro for $func_name)
                                ),
                        )*
                    }
                }
            )*
        }
    };
}

container_kind!(
    fraction([2]) -> Fraction {
        numerator, denominator,
    };
    sub_script([1]) -> SubScript {
        sequence,
    };
    super_script([1]) -> SuperScript {
        sequence,
    };
);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Container {
    pub(super) kind: ContainerKind,

    pub(super) parent_id: SequenceId,
}

impl Container {
    pub fn kind(&self) -> &ContainerKind {
        &self.kind
    }

    pub fn parent_id(&self) -> SequenceId {
        self.parent_id
    }

    pub(super) fn new_unchecked(parent_id: SequenceId, kind: ContainerKind) -> Self {
        Self { parent_id, kind }
    }
}

#[derive(Debug, Copy)]
pub struct ContainerRef<'a> {
    id: ContainerId,
    tree: &'a Tree,
    container: &'a Container,
}

pub struct ContainerMut<'a> {
    /// On construction this is guaranteed to exist
    pub(super) id: ContainerId,

    /// Mutating tree must not allow `Self::id` to become invalid
    pub(super) tree: &'a mut Tree,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OwnedContainer(pub OwnedContainerKind);
