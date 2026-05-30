//! database repositories (DAO layer)

pub mod account;
pub mod folder;
pub mod email;

pub use account::AccountRepository;
pub use folder::FolderRepository;
pub use email::EmailRepository;

