// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod encoding;

use deno_core::extension;

use crate::js_op;

extension!(
    r3e,
    ops = [op_defer],
    esm_entry_point = "ext:r3e/r3e.js",
    esm = [dir "src/js", "r3e.js", "encoding.js", "infra.js", "time.js"]
);

// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
#[allow(clippy::unused_async)]
#[js_op(async(lazy), fast)]
pub async fn op_defer() {}
