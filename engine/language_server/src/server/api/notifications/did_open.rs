use std::collections::HashMap;
use std::path::{Path, PathBuf};
use baml_runtime::InternalRuntimeInterface;
use log::info;
use lsp_server::ErrorCode;
use lsp_types::notification::DidOpenTextDocument;
use lsp_types::{DiagnosticSeverity, DidOpenTextDocumentParams, PublishDiagnosticsParams, Url};

// use crate::baml_diagnostics::baml_to_lsp_diagnostics;
use crate::baml_project::watch::ChangeEvent;
use crate::baml_project::Project;
use crate::baml_text_size::TextSize;
use crate::server::api::ResultExt;
use crate::server::api::diagnostics::session_lsp_diagnostics;
use crate::server::api::traits::{NotificationHandler, SyncNotificationHandler};
use crate::server::client::{Notifier, Requester};
use crate::server::{Result, api::Error};
use crate::session::Session;
// use crate::system::{url_to_any_system_path, AnySystemPath};
use crate::{DocumentKey, TextDocument};

pub(crate) struct DidOpenTextDocumentHandler;

impl NotificationHandler for DidOpenTextDocumentHandler {
    type NotificationType = DidOpenTextDocument;
}

impl SyncNotificationHandler for DidOpenTextDocumentHandler {
    fn run(
        session: &mut Session,
        notifier: Notifier,
        _requester: &mut Requester,
        params: DidOpenTextDocumentParams,
    ) -> Result<()> {
        info!("did_open params: {:?}", params);
        tracing::info!("DidOpenTextDocumentHandler");
        // let Ok(path) = url_to_any_system_path(&params.text_document.uri) else {
        //     return Ok(());
        // };

        // let document = TextDocument::new(params.text_document.text, params.text_document.version);

        let url = params.text_document.uri;
        // session.open_text_document(url.clone(), document);
        session.reload().internal_error()?;

        let diagnostics = session_lsp_diagnostics(session, &url);

        // TODO: Only send this when clients do not support pull diagnostics?
        notifier.notify::<lsp_types::notification::PublishDiagnostics>( PublishDiagnosticsParams {
            uri: url,
            version: Some(params.text_document.version),
            diagnostics,
        }).expect("TODO");

        // match path {
        //     AnySystemPath::System(path) => {
        //         let db = match session.project_db_for_path_mut(path.as_std_path()) {
        //             Some(db) => db,
        //             None => session.default_project_db_mut(),
        //         };
        //         db.apply_changes(vec![ChangeEvent::Opened(path)], None);
        //     }
        //     AnySystemPath::SystemVirtual(virtual_path) => {
        //         let db = session.default_project_db_mut();
        //         db.files().virtual_file(db, &virtual_path);
        //     }
        // }

        // TODO(dhruvmanila): Publish diagnostics if the client doesn't support pull diagnostics

        Ok(())
    }
}

 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_to_url() {
        let path_buf = PathBuf::from("file:///test.baml");
        let url = Url::from_file_path(&path_buf).unwrap();
        assert_eq!(url.as_str(), "file:///test.baml");
    }

    #[test]
    fn parse_path_with_file_prefix() {
        let url = Url::parse("file:///test.baml").unwrap();
        assert_eq!(url.as_str(), "file:///test.baml");
    }

}