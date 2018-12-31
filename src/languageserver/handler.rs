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

use serde_json::json;

/// A type-safe wrapper over `jsonrpc_core::IoHandler` to implement a language server.
#[derive(Default)]
pub struct LanguageServerHandler {
    io_handler: jsonrpc_core::IoHandler,
}

impl LanguageServerHandler {
    /// Re-exposes the same method from `jsonrpc_core::IoHandler`.
    pub fn handle_request_sync(&self, request: &str) -> Option<String> {
        self.io_handler.handle_request_sync(request)
    }

    /// Registers a method handler. This acts as a type-safe wrapper over the method from
    /// `jsonrpc_core::IoHandler` with the same name.
    pub fn add_method<R, F>(&mut self, callable: F)
    where
        R: languageserver_types::request::Request,
        R::Result: serde::ser::Serialize,
        F: Fn(R::Params) -> R::Result + Send + Sync + 'static,
        for<'de> R::Params: serde::de::Deserialize<'de>,
    {
        self.io_handler
            .add_method(R::METHOD, move |params: jsonrpc_core::Params| {
                let params = params
                    .parse::<R::Params>()
                    .expect("valid request parameters");

                Ok(json!(callable(params)))
            })
    }

    /// Registers a notification handler. This acts as a type-safe wrapper over the method from
    /// `jsonrpc_core::IoHandler` with the same name.
    pub fn add_notification<N, F>(&mut self, callable: F)
    where
        N: languageserver_types::notification::Notification,
        F: Fn(N::Params) + Send + Sync + 'static,
        for<'de> N::Params: serde::de::Deserialize<'de>,
    {
        self.io_handler
            .add_notification(N::METHOD, move |params: jsonrpc_core::Params| {
                let params = params
                    .parse::<N::Params>()
                    .expect("valid notification parameters");

                callable(params)
            })
    }
}
