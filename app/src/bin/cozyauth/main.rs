// © Copyright 2024 the cozyauth developers
// SPDX-License-Identifier: AGPL-3.0-or-later

use cozyauth::app;

#[tokio::main]
async fn main() {
    app::start_server().await
}
