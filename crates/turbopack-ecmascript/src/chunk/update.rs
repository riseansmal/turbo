use anyhow::Result;
use indexmap::IndexMap;
use turbopack_core::{chunk::ModuleIdReadRef, code_builder::CodeReadRef};

use super::{content::EcmascriptChunkContentVc, version::EcmascriptChunkVersionVc};

#[turbo_tasks::value]
pub(super) enum EcmascriptChunkUpdate {
    None,
    Partial(EcmascriptChunkPartialUpdate),
}

#[turbo_tasks::value]
pub(super) struct EcmascriptChunkPartialUpdate {
    pub added: IndexMap<ModuleIdReadRef, (u64, CodeReadRef)>,
    pub deleted: IndexMap<ModuleIdReadRef, u64>,
    pub modified: IndexMap<ModuleIdReadRef, CodeReadRef>,
}

#[turbo_tasks::function]
pub(super) async fn update_ecmascript_chunk(
    content: EcmascriptChunkContentVc,
    from_version: EcmascriptChunkVersionVc,
) -> Result<EcmascriptChunkUpdateVc> {
    let to = content.version().await?;
    let from = from_version.await?;

    // When to and from point to the same value we can skip comparing them.
    // This will happen since `TraitRef<VersionVc>::cell` will not clone the value,
    // but only make the cell point to the same immutable value (Arc).
    if from.ptr_eq(&to) {
        return Ok(EcmascriptChunkUpdate::None.cell());
    }

    let content = content.await?;

    // TODO(alexkirsz) This should probably be stored as a HashMap already.
    let mut module_factories: IndexMap<_, _> = content
        .module_factories
        .iter()
        .map(|entry| (entry.id.clone(), entry))
        .collect();
    let mut added = IndexMap::new();
    let mut modified = IndexMap::new();
    let mut deleted = IndexMap::new();

    for (id, from_hash) in &from.module_factories_hashes {
        let id = &*id;
        if let Some(entry) = module_factories.remove(id) {
            if entry.hash != *from_hash {
                modified.insert(id.clone(), entry.code.clone());
            }
        } else {
            deleted.insert(id.clone(), *from_hash);
        }
    }

    // Remaining entries are added
    for (id, entry) in module_factories {
        if !from.module_factories_hashes.contains_key(&id) {
            added.insert(id, (entry.hash, entry.code.clone()));
        }
    }

    let update = if added.is_empty() && modified.is_empty() && deleted.is_empty() {
        EcmascriptChunkUpdate::None
    } else {
        EcmascriptChunkUpdate::Partial(EcmascriptChunkPartialUpdate {
            added,
            modified,
            deleted,
        })
    };

    Ok(update.cell())
}
