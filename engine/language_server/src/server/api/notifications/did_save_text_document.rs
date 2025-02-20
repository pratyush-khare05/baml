use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::server::api::ResultExt;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub struct DidSaveTextDocument;

impl super::NotificationHandler for DidSaveTextDocument {
    type NotificationType = notif::DidSaveTextDocument;
}

impl super::SyncNotificationHandler for DidSaveTextDocument {
    fn run(
        session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        _params: types::DidSaveTextDocumentParams,
    ) -> Result<()> {
       session.reload().internal_error()?; 
       tracing::info!("About to run generator");
       session.default_project_db_mut().run_generators_without_debounce(|_| {}, |_| {});
       Ok(())
    }
}
