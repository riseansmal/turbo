use anyhow::Result;
use turbo_tasks_fs::FileSystemPathVc;

use super::content::ChunkListContentVc;
use crate::{
    asset::{Asset, AssetContentVc, AssetVc},
    chunk::{ChunkGroupVc, ChunkReferenceVc, ChunksVc},
    reference::AssetReferencesVc,
    version::{VersionedContent, VersionedContentVc},
};

#[turbo_tasks::value(shared)]
pub(super) struct ChunkListAsset {
    server_root: FileSystemPathVc,
    chunk_group: ChunkGroupVc,
    path: FileSystemPathVc,
}

#[turbo_tasks::value_impl]
impl ChunkListAssetVc {
    #[turbo_tasks::function]
    pub fn new(
        server_root: FileSystemPathVc,
        chunk_group: ChunkGroupVc,
        path: FileSystemPathVc,
    ) -> Self {
        ChunkListAsset {
            server_root,
            chunk_group,
            path,
        }
        .cell()
    }

    #[turbo_tasks::function]
    async fn get_chunks(self) -> Result<ChunksVc> {
        Ok(self.await?.chunk_group.chunks())
    }

    #[turbo_tasks::function]
    async fn content(self) -> Result<ChunkListContentVc> {
        let this = &*self.await?;
        Ok(ChunkListContentVc::new(
            this.server_root,
            this.chunk_group.chunks(),
        ))
    }
}

#[turbo_tasks::value_impl]
impl Asset for ChunkListAsset {
    #[turbo_tasks::function]
    fn path(&self) -> FileSystemPathVc {
        self.path
    }

    #[turbo_tasks::function]
    async fn references(&self) -> Result<AssetReferencesVc> {
        let chunks = self.chunk_group.chunks().await?;

        let mut references = Vec::with_capacity(chunks.len());
        for chunk in chunks.iter() {
            references.push(ChunkReferenceVc::new(*chunk).into());
        }

        Ok(AssetReferencesVc::cell(references))
    }

    #[turbo_tasks::function]
    fn content(self_vc: ChunkListAssetVc) -> AssetContentVc {
        self_vc.content().content()
    }

    #[turbo_tasks::function]
    fn versioned_content(self_vc: ChunkListAssetVc) -> VersionedContentVc {
        self_vc.content().into()
    }
}
