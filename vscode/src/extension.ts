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

import * as fs from "fs";
import * as process from "process";
import * as path from "path";

import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from "vscode-languageclient";

export function activate(context: vscode.ExtensionContext) {
  let devServerPath = path.join(
    process.cwd(),
    "target",
    "debug",
    "ldn-languageserver"
  );

  let languageServerCommand;
  if (fs.existsSync(devServerPath)) {
    languageServerCommand = devServerPath;
  } else {
    languageServerCommand = "ldn-languageserver"; // Assume it's in $PATH
  }

  let languageServerOptions: ServerOptions = {
    command: languageServerCommand,
    transport: TransportKind.stdio
  };

  let languageClientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "ldn" }]
  };

  let client = new LanguageClient(
    "ldn",
    languageServerOptions,
    languageClientOptions
  );

  context.subscriptions.push(client.start());
}

export function deactivate() {}
