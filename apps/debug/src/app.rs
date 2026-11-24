use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};

use bitflags::bitflags;
use matatree::{editor::CursoredTree, tree::SequenceId};
use ron::ser::PrettyConfig;

use crate::{command::TreeEdit, tree::draw_tree};

bitflags! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct VisibleSections: u8 {
        const TREE = 1 << 0;
        const CURSOR = 1 << 1;
    }
}

pub enum ContextEdit<'a> {
    Editable {
        edit_list: &'a mut Vec<TreeEdit>,
        atom_buffers: &'a mut HashMap<(SequenceId, usize), String>,
    },
    Immutable,
}

pub struct Context<'a> {
    pub ui: &'a mut egui::Ui,

    pub frame: &'a mut eframe::Frame,

    pub tree: &'a CursoredTree,

    pub edit: ContextEdit<'a>,
}

impl ContextEdit<'_> {
    pub fn push_edit(&mut self, edit: TreeEdit) -> bool {
        match self {
            ContextEdit::Editable { edit_list, .. } => {
                edit_list.push(edit);
                true
            }
            ContextEdit::Immutable => false,
        }
    }

    pub fn is_mut(&self) -> bool {
        matches!(self, ContextEdit::Editable { .. })
    }
}

#[derive(Debug)]
pub enum AppKind {
    ImmutableMirror {
        cache: CursoredTree,
        tree: Arc<parking_lot::RwLock<CursoredTree>>,
    },
    MutableMirror {
        cache: CursoredTree,
        tree: Arc<parking_lot::RwLock<CursoredTree>>,
        atom_buffers: HashMap<(SequenceId, usize), String>,
    },
    ImmutableOwned {
        editor: CursoredTree,
    },
    Owned {
        editor: CursoredTree,
        atom_buffers: HashMap<(SequenceId, usize), String>,
    },
}

impl Default for AppKind {
    fn default() -> Self {
        Self::Owned {
            atom_buffers: HashMap::new(),
            editor: CursoredTree::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct DebugApp {
    visible_sections: VisibleSections,
    save_path: Option<PathBuf>,

    pub kind: AppKind,
}

impl DebugApp {
    pub fn new() -> Self {
        Self {
            visible_sections: VisibleSections::all(),
            ..Default::default()
        }
    }

    pub fn try_update_cache(&mut self) {
        match self.kind {
            AppKind::ImmutableMirror {
                ref mut cache,
                ref tree,
            }
            | AppKind::MutableMirror {
                ref mut cache,
                ref tree,
                ..
            } => {
                log::debug!("Trying to update cache");
                *cache = match (*tree).try_read() {
                    Some(t) => t.to_owned(),
                    None => {
                        log::debug!("Failed to get lock");
                        return;
                    }
                };
                log::debug!("Updated cache");
            }
            _ => return,
        };
    }

    pub fn update_cache(&mut self) {
        match self.kind {
            AppKind::ImmutableMirror {
                ref mut cache,
                ref tree,
            }
            | AppKind::MutableMirror {
                ref mut cache,
                ref tree,
                ..
            } => {
                log::debug!("Updating cache");
                *cache = (*tree).read().to_owned();
                log::debug!("Updated cache");
            }
            _ => return,
        };
    }

    pub fn with_save_path(&mut self, save_path: Option<PathBuf>) {
        self.save_path = save_path
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut file = fs::OpenOptions::new().read(true).open(path)?;

        let mut buf = String::with_capacity(file.metadata()?.len() as usize);

        file.read_to_string(&mut buf)?;

        let editor = ron::from_str(&buf)?;

        Ok(Self {
            save_path: None,
            visible_sections: VisibleSections::all(),
            kind: AppKind::Owned {
                editor,
                atom_buffers: HashMap::new(),
            },
        })
    }

    pub fn get_tree_ref(&self) -> &CursoredTree {
        match &self.kind {
            AppKind::ImmutableMirror { cache: tree, .. }
            | AppKind::MutableMirror { cache: tree, .. }
            | AppKind::ImmutableOwned { editor: tree }
            | AppKind::Owned { editor: tree, .. } => tree,
        }
    }
}

impl eframe::App for DebugApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let visible_sections = self.visible_sections;

        let mut edit_list = Vec::new();

        ui.vertical(|ui| {
            let cursor = self.get_tree_ref().cursor();

            let (edit, tree) = match self.kind {
                AppKind::ImmutableMirror { ref cache, .. } => (ContextEdit::Immutable, cache),
                AppKind::ImmutableOwned { ref editor } => (ContextEdit::Immutable, editor),
                AppKind::MutableMirror {
                    ref mut atom_buffers,
                    ref cache,
                    ..
                } => (
                    ContextEdit::Editable {
                        edit_list: &mut edit_list,
                        atom_buffers,
                    },
                    cache,
                ),
                AppKind::Owned {
                    ref mut atom_buffers,
                    ref editor,
                    ..
                } => (
                    ContextEdit::Editable {
                        edit_list: &mut edit_list,
                        atom_buffers,
                    },
                    editor,
                ),
            };

            let mut context = Context {
                ui,
                frame,
                edit,
                tree,
            };

            if visible_sections.contains(VisibleSections::TREE) {
                draw_tree(&mut context);
            };

            if visible_sections.contains(VisibleSections::CURSOR) {
                context.ui.horizontal(|ui| {
                    if ui.small_button("<").clicked() {
                        context.edit.push_edit(TreeEdit::Left);
                    }

                    if ui.small_button(">").clicked() {
                        context.edit.push_edit(TreeEdit::Right);
                    }

                    ui.label(format!("Id: {:?}", cursor.sequence_id()));
                    ui.label(format!("{:?}", cursor.child_index()));
                });
            };
        });

        for edit in edit_list {
            self.apply_edit(edit);
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        log::info!("Save called");

        self.update_cache();

        if let Some(ref path) = self.save_path {
            let contents =
                match ron::ser::to_string_pretty(self.get_tree_ref(), PrettyConfig::new()) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("Serialize error when saving: {e}");
                        return;
                    }
                };

            if let Err(e) = fs::write(path, contents) {
                log::error!("File write error when saving: {e}")
            }
        }
    }
}
