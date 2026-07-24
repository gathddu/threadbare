//! e-mail synchronization logic

use crate::state::DaemonState;
use anyhow::Result;
use tracing::{info, warn, error};
use async_imap::Session;
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};
use futures::TryStreamExt;

type ImapSession = Session<Compat<TlsStream<TcpStream>>>;

/// e-mail synchronization manager
pub struct SyncManager {
    state: DaemonState,
}

impl SyncManager {
    /// create a new sync manager
    pub fn new(state: DaemonState) -> Self {
        Self { state }
    }

    /// start the sync loop
    pub async fn start_sync_loop(&self) -> Result<()> {
        let sync_interval = self.state.config().daemon.sync_interval;

        loop {
            info!("Starting sync cycle");

            if let Err(e) = self.sync_all_accounts().await {
                error!("Sync error: {}", e);
            }

            // wait for the next sync interval
            tokio::time::sleep(tokio::time::Duration::from_secs(sync_interval)).await;
        }
    }

    /// sync all accounts
    async fn sync_all_accounts(&self) -> Result<()> {
        let accounts = self.state.config().accounts.clone();

        if accounts.is_empty() {
            info!("No accounts configured, skipping sync");
            return Ok(());
        }

        for account in &accounts {
            info!("Syncing account: {}", account.email);
            if let Err(e) = self.sync_account(account).await {
                error!("Failed to sync {}: {}", account.email, e);
            }
        }

        Ok(())
    }

    /// sync a single account
    async fn sync_account(&self, account: &threadbare_core::config::AccountConfig) -> Result<()> {
        let mut session = self.connect_imap(account).await?;

        // fetch folder list
        let mailboxes: Vec<_> = session.list(Some(""), Some("*")).await?
            .try_collect().await?;
        let folder_names: Vec<String> = mailboxes
            .iter()
            .map(|m| m.name().to_string())
            .collect();

        info!("Found {} folders for {}", folder_names.len(), account.email);

        // sync inbox first
        if let Err(e) = self.sync_folder(&mut session, "INBOX").await {
            error!("Failed to sync INBOX: {}", e);
        }

        // sync other folders
        for folder in &folder_names {
            if folder == "INBOX" {
                continue;
            }
            if let Err(e) = self.sync_folder(&mut session, folder).await {
                warn!("Failed to sync folder {}: {}", folder, e);
            }
        }

        // logout
        session.logout().await?;
        info!("Finished syncing {}", account.email);

        Ok(())
    }

    /// connect to IMAP server
    async fn connect_imap(
        &self,
        account: &threadbare_core::config::AccountConfig,
    ) -> Result<ImapSession> {
        let addr = format!("{}:{}", account.imap_host, account.imap_port);
        info!("Connecting to IMAP: {}", addr);

        let tcp_stream = TcpStream::connect(&addr).await?;

        let tls_connector = tokio_native_tls::TlsConnector::from(
            native_tls::TlsConnector::new()?
        );
        let tls_stream = tls_connector.connect(&account.imap_host, tcp_stream).await?;

        // wrap with compat layer to bridge tokio to futures-io
        let compat_stream = tls_stream.compat();

        let mut client = async_imap::Client::new(compat_stream);

        // read server greeting
        let _greeting = client.read_response().await?
            .ok_or_else(|| anyhow::anyhow!("No server greeting received"))?;

        let session = client
            .login(&account.username, &account.password)
            .await
            .map_err(|e| anyhow::anyhow!("IMAP login failed: {}", e.0))?;

        info!("Connected to {} as {}", account.imap_host, account.username);
        Ok(session)
    }

    /// sync a single folder
    async fn sync_folder(
        &self,
        session: &mut ImapSession,
        folder: &str,
    ) -> Result<()> {
        let mailbox = session.select(folder).await?;
        let msg_count = mailbox.exists;
        info!("Folder '{}': {} messages", folder, msg_count);

        if msg_count == 0 {
            return Ok(());
        }

        // fetch last 50 messages or all if less
        let start = if msg_count > 50 { msg_count - 49 } else { 1 };
        let range = format!("{}:*", start);

        let messages: Vec<_> = session
            .fetch(&range, "(UID FLAGS ENVELOPE BODY.PEEK[])")
            .await?
            .try_collect().await?;

        info!("Fetched {} messages from '{}'", messages.len(), folder);

        for msg in &messages {
            if let Some(body) = msg.body() {
                // parse the e-mail
                match mail_parser::MessageParser::default().parse(body) {
                    Some(parsed) => {
                        let from = parsed
                            .from()
                            .and_then(|f| f.first())
                            .map(|a| {
                                a.name()
                                    .unwrap_or(a.address().unwrap_or("unknown"))
                                    .to_string()
                            })
                            .unwrap_or_else(|| "unknown".to_string());

                        let subject = parsed
                            .subject()
                            .unwrap_or("(no subject)")
                            .to_string();

                        info!("  From: {} | Subject: {}", from, subject);

                        // TODO: store in database
                    }
                    None => {
                        warn!("Failed to parse message");
                    }
                }
            }
        }

        Ok(())
    }
}

