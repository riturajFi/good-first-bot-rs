mod repo_entity;
pub mod sqlite;
#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use mockall::automock;
pub use repo_entity::RepoEntity;
use teloxide::types::ChatId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Data integrity error: Stored repository '{0}' is invalid: {1}")]
    DataIntegrityError(String, #[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

pub type StorageResult<T> = Result<T, StorageError>;

#[automock]
#[async_trait]
pub trait RepoStorage: Send + Sync {
    /// Add a repository to the storage
    /// Returns `true` if the repository was added, `false` if it was already
    /// present.
    async fn add_repository(&self, chat_id: ChatId, repository: RepoEntity) -> StorageResult<bool>;

    /// Remove a repository from the storage.
    async fn remove_repository(
        &self,
        chat_id: ChatId,
        repo_name_with_owner: &str,
    ) -> StorageResult<bool>;

    /// Get all repositories for a user.
    async fn get_repos_per_user(&self, chat_id: ChatId) -> StorageResult<Vec<RepoEntity>>;

    /// Get all repositories from the storage.
    async fn get_all_repos(&self) -> StorageResult<HashMap<ChatId, HashSet<RepoEntity>>>;

    /// Get the last poll time for a repository.
    async fn get_last_poll_time(
        &self,
        chat_id: ChatId,
        repository: &RepoEntity,
    ) -> StorageResult<Option<i64>>;

    /// Set the last poll time for a repository.
    async fn set_last_poll_time(
        &self,
        chat_id: ChatId,
        repository: &RepoEntity,
    ) -> StorageResult<()>;

    /// Get tracked labels by for user and repository.
    async fn get_tracked_labels(
        &self,
        chat_id: ChatId,
        repository: &RepoEntity,
    ) -> StorageResult<HashSet<String>>;

    /// Add or remove a label from the user tracked repository labels.
    async fn toggle_label(
        &self,
        chat_id: ChatId,
        repository: &RepoEntity,
        label_name: &str,
    ) -> StorageResult<bool>;

    /// Get the number of repositories per user.
    async fn count_repos_per_user(&self, chat_id: ChatId) -> StorageResult<usize>;
}
