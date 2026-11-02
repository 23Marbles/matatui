use std::{
    collections::HashMap,
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
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

pub struct Context<'a> {
    pub ui: &'a mut egui::Ui,

    pub app: &'a mut DebugApp,

    pub frame: &'a mut eframe::Frame,

    pub edit_list: &'a mut Vec<TreeEdit>,
}

#[derive(Debug, Default)]
pub struct DebugApp {
    pub editor: CursoredTree,

    save_path: Option<PathBuf>,

    pub atom_buffers: HashMap<(SequenceId, usize), String>,

    visible_sections: VisibleSections,
}

impl DebugApp {
    pub fn new() -> Self {
        Self {
            visible_sections: VisibleSections::all(),
            ..Default::default()
        }
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
            editor,
            atom_buffers: HashMap::new(),
            save_path: None,
            visible_sections: VisibleSections::all(),
        })
    }
}

impl eframe::App for DebugApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let visible_sections = self.visible_sections;

        let mut edit_list = Vec::new();

        ui.vertical(|ui| {
            let mut context = Context {
                app: self,
                ui,
                frame,
                edit_list: &mut edit_list,
            };

            if visible_sections.contains(VisibleSections::TREE) {
                draw_tree(&mut context);
            };

            if visible_sections.contains(VisibleSections::CURSOR) {
                context.ui.horizontal(|ui| {
                    if ui.small_button("<").clicked() {
                        context.edit_list.push(TreeEdit::Left);
                    }

                    if ui.small_button(">").clicked() {
                        context.edit_list.push(TreeEdit::Right);
                    }

                    let cursor = context.app.editor.cursor();

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

        if let Some(ref path) = self.save_path {
            let contents = match ron::ser::to_string_pretty(&self.editor, PrettyConfig::new()) {
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
