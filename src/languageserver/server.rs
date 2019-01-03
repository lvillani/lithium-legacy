// Lithium
// Copyright (C) 2018 Lorenzo Villani
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::io;
use std::process;
use std::sync::{Arc, Mutex};

use languageserver_types::*;
use serde_json::json;

use super::super::ldn;
use super::super::ldn::{fmt, List, Parser};
use super::handler::LanguageServerHandler;
use super::stdio_transport::{read_request, write_response};

/// Runs the server's processing loop. Blocks the calling thread.
pub fn run() -> io::Result<()> {
    let handler = build_handler();

    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        let request = read_request(&mut stdin.lock())?;

        match handler.handle_request_sync(&request) {
            None => continue,
            Some(response) => write_response(&mut stdout.lock(), &response)?,
        }
    }
}

/// Builds the language server handle.
fn build_handler() -> LanguageServerHandler {
    let mut handler = LanguageServerHandler::default();
    let workspace: Arc<Mutex<HashMap<url::Url, List>>> = Arc::new(Mutex::new(HashMap::new()));

    // initialize
    handler.add_method::<request::Initialize, _>(|_params| {
        languageserver_types::InitializeResult {
            capabilities: languageserver_types::ServerCapabilities {
                document_formatting_provider: Some(true),

                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        change: Some(TextDocumentSyncKind::Full),
                        open_close: Some(true),
                        ..TextDocumentSyncOptions::default()
                    },
                )),

                ..ServerCapabilities::default()
            },
        }
    });

    // textDocument/formatting
    {
        let workspace = workspace.clone();

        handler.add_method::<request::Formatting, _>(move |params| {
            let workspace = workspace.lock().unwrap();

            match workspace.get(&params.text_document.uri) {
                None => None,
                Some(ast) => Some(vec![TextEdit::new(
                    Range::new(
                        Position::new(0, 0),
                        Position::new(u64::max_value(), u64::max_value()),
                    ),
                    fmt(ast),
                )]),
            }
        });
    }

    // textDocument/didOpen
    {
        let workspace = workspace.clone();

        handler.add_notification::<notification::DidOpenTextDocument, _>(move |params| {
            let mut workspace = workspace.lock().unwrap();

            match Parser::from(params.text_document.text.as_ref()).parse() {
                Err(err) => {
                    workspace.remove(&params.text_document.uri);
                    write_diagnostic(params.text_document.uri, Some(err))
                        .expect("sent diagnostic on file open");
                }
                Ok(ast) => {
                    workspace.insert(params.text_document.uri.clone(), ast);
                    write_diagnostic(params.text_document.uri, None)
                        .expect("clear diagnostic on file open");
                }
            }
        });
    }

    // textDocument/didChange
    {
        let workspace = workspace.clone();

        handler.add_notification::<notification::DidChangeTextDocument, _>(move |params| {
            let mut workspace = workspace.lock().unwrap();

            match Parser::from(params.content_changes[0].text.as_ref()).parse() {
                Err(err) => {
                    workspace.remove(&params.text_document.uri);
                    write_diagnostic(params.text_document.uri, Some(err))
                        .expect("sent diagnostic on file change");
                }
                Ok(ast) => {
                    workspace.insert(params.text_document.uri.clone(), ast);
                    write_diagnostic(params.text_document.uri, None)
                        .expect("clear diagnostic on file change");
                }
            }
        });
    }

    // textDocument/didClose
    {
        let workspace = workspace.clone();

        handler.add_notification::<notification::DidCloseTextDocument, _>(move |params| {
            let mut workspace = workspace.lock().unwrap();

            workspace.remove(&params.text_document.uri);
            write_diagnostic(params.text_document.uri, None)
                .expect("clear diagnostic on file close");
        });
    }

    // exit
    handler.add_notification::<notification::Exit, _>(|_params| process::exit(0));

    handler
}

/// Writes the given diagnostic to the given writer.
pub fn write_diagnostic(document_uri: url::Url, err: Option<ldn::Error>) -> io::Result<()> {
    // TODO(lvillani): I'm not sure this is the right way to construct a server-to-client
    // notification.
    let envelope = jsonrpc_core::Notification {
        jsonrpc: Some(jsonrpc_core::Version::V2),
        method: "textDocument/publishDiagnostics".into(),
        params: jsonrpc_core::Params::Map(
            json!(PublishDiagnosticsParams {
                uri: document_uri,
                diagnostics: match err {
                    None => vec![],
                    Some(err) => vec![err.into()],
                }
            })
            .as_object()
            .expect("valid lsp message")
            .clone(),
        ),
    };

    write_response(
        &mut io::stdout().lock(),
        &serde_json::to_string(&envelope).expect("serializable notification"),
    )
}

// FIXME(lvillani): I don't really like how repetitive all of this is. Can we make it shorter?
impl Into<Diagnostic> for ldn::Error {
    fn into(self) -> Diagnostic {
        match self {
            ldn::Error::IntegerLeadingZero(_, span) => Diagnostic {
                message: "Found leading zero while parsing integer constant".into(),
                range: span.into(),
                severity: Some(DiagnosticSeverity::Error),
                ..Diagnostic::default()
            },
            ldn::Error::IntegerParseError(_, span) => Diagnostic {
                message: "Invalid integer constant".into(),
                range: span.into(),
                severity: Some(DiagnosticSeverity::Error),
                ..Diagnostic::default()
            },
            ldn::Error::InvalidCharacter(_, pos) => Diagnostic {
                message: "Invalid character".into(),
                range: pos.into(),
                severity: Some(DiagnosticSeverity::Error),
                ..Diagnostic::default()
            },
            ldn::Error::SymbolParseError(_, span) => Diagnostic {
                message: "Symbol parse error".into(),
                range: span.into(),
                severity: Some(DiagnosticSeverity::Error),
                ..Diagnostic::default()
            },
            ldn::Error::UnbalancedParentheses(pos) => Diagnostic {
                message: "Unbalanced parentheses".into(),
                range: pos.into(),
                severity: Some(DiagnosticSeverity::Error),
                ..Diagnostic::default()
            },
            ldn::Error::Utf8Error(span) => Diagnostic {
                message: "UTF-8 decode error".into(),
                range: span.into(),
                severity: Some(DiagnosticSeverity::Error),
                ..Diagnostic::default()
            },
        }
    }
}

impl Into<Range> for ldn::Span {
    fn into(self) -> Range {
        Range::new(
            Position::new(self.start.line as u64, self.start.column as u64),
            Position::new(self.end.line as u64, self.end.column as u64),
        )
    }
}

impl Into<Range> for ldn::Position {
    fn into(self) -> Range {
        Range::new(
            Position::new(self.line as u64, self.column as u64),
            Position::new(self.line as u64, self.column as u64),
        )
    }
}
