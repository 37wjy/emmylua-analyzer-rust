mod decl;
mod decl_tree;
mod scope;

pub use decl::LuaDeclExtra;
pub use decl::{LocalAttribute, LuaDecl, LuaDeclId};
pub use decl_tree::{LuaDeclOrMemberId, LuaDeclarationTree};
pub use scope::{LuaScope, LuaScopeId, LuaScopeKind, ScopeOrDeclId};
use smol_str::SmolStr;
use std::collections::HashMap;

use crate::FileId;

use super::{traits::LuaIndex, LuaMemberKey};

#[derive(Debug)]
pub struct LuaDeclIndex {
    decl_trees: HashMap<FileId, LuaDeclarationTree>,
    global_decl: HashMap<LuaMemberKey, Vec<LuaDeclId>>,
}

impl LuaDeclIndex {
    pub fn new() -> Self {
        Self {
            decl_trees: HashMap::new(),
            global_decl: HashMap::new(),
        }
    }

    pub fn add_global_decl(&mut self, name: &str, decl_id: LuaDeclId) {
        let key = SmolStr::new(name);
        self.global_decl
            .entry(LuaMemberKey::Name(key))
            .or_insert_with(Vec::new)
            .push(decl_id);
    }

    pub fn add_decl_tree(&mut self, tree: LuaDeclarationTree) {
        self.decl_trees.insert(tree.file_id(), tree);
    }

    pub fn get_decl_tree(&self, file_id: &FileId) -> Option<&LuaDeclarationTree> {
        self.decl_trees.get(file_id)
    }

    pub fn get_decl(&self, decl_id: &LuaDeclId) -> Option<&LuaDecl> {
        let tree = self.decl_trees.get(&decl_id.file_id)?;
        tree.get_decl(decl_id)
    }

    pub fn get_decl_mut(&mut self, decl_id: &LuaDeclId) -> Option<&mut LuaDecl> {
        let tree = self.decl_trees.get_mut(&decl_id.file_id)?;
        tree.get_decl_mut(*decl_id)
    }

    pub fn get_global_decls(&self) -> Vec<LuaDeclId> {
        let mut decls = Vec::new();
        for (_, v) in &self.global_decl {
            decls.extend(v);
        }

        decls
    }

    pub fn get_global_decls_by_name(&self, name: &str) -> Vec<LuaDeclId> {
        let key = SmolStr::new(name);
        self.global_decl
            .get(&LuaMemberKey::Name(key))
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_global_decl_id(&self, key: &LuaMemberKey) -> Option<LuaDeclId> {
        let decls = self.global_decl.get(key)?;
        if decls.len() == 1 {
            return Some(decls[0]);
        }

        let mut valid_decl_id = None;
        for decl_id in decls {
            let decl = self.get_decl(decl_id)?;
            let ty = decl.get_type();
            if let Some(ty) = ty {
                if ty.is_def() || ty.is_ref() {
                    return Some(*decl_id);
                }

                if valid_decl_id.is_none() {
                    valid_decl_id = Some(*decl_id);
                } else if ty.is_table() {
                    valid_decl_id = Some(*decl_id);
                }
            }
        }

        valid_decl_id
    }
}

impl LuaIndex for LuaDeclIndex {
    fn remove(&mut self, file_id: FileId) {
        self.decl_trees.remove(&file_id);
        self.global_decl.retain(|_, v| {
            v.retain(|decl_id| decl_id.file_id != file_id);
            !v.is_empty()
        });
    }

    fn clear(&mut self) {
        self.decl_trees.clear();
        self.global_decl.clear();
    }
}
