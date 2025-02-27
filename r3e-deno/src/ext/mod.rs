// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

pub mod encoding;
pub mod neo;
pub mod oracle;
pub mod tee;
pub mod neo_services;

use deno_core::extension;

use crate::js_op;
use neo::{op_neo_create_key_pair, op_neo_create_rpc_client, op_neo_create_transaction, op_neo_invoke_script};
use oracle::{op_oracle_submit_request, op_oracle_get_request_status, op_oracle_get_response, op_oracle_cancel_request, op_oracle_get_price, op_oracle_get_random};
use tee::{op_tee_execute, op_tee_generate_attestation, op_tee_verify_attestation, op_neo_tee_execute};
use neo_services::{
    op_neo_gas_bank_create_account, op_neo_gas_bank_get_account, op_neo_gas_bank_deposit, 
    op_neo_gas_bank_withdraw, op_neo_gas_bank_pay_gas, op_neo_gas_bank_get_gas_price,
    op_neo_meta_tx_submit, op_neo_meta_tx_get_status, op_neo_meta_tx_get_transaction, op_neo_meta_tx_get_next_nonce,
    op_neo_abstract_account_create, op_neo_abstract_account_get, op_neo_abstract_account_execute_operation,
    op_neo_abstract_account_get_operation_status, op_neo_abstract_account_get_next_nonce
};

extension!(
    r3e,
    ops = [
        op_defer,
        op_neo_create_rpc_client,
        op_neo_create_key_pair,
        op_neo_create_transaction,
        op_neo_invoke_script,
        op_oracle_submit_request,
        op_oracle_get_request_status,
        op_oracle_get_response,
        op_oracle_cancel_request,
        op_oracle_get_price,
        op_oracle_get_random,
        op_tee_execute,
        op_tee_generate_attestation,
        op_tee_verify_attestation,
        op_neo_tee_execute,
        op_neo_gas_bank_create_account,
        op_neo_gas_bank_get_account,
        op_neo_gas_bank_deposit,
        op_neo_gas_bank_withdraw,
        op_neo_gas_bank_pay_gas,
        op_neo_gas_bank_get_gas_price,
        op_neo_meta_tx_submit,
        op_neo_meta_tx_get_status,
        op_neo_meta_tx_get_transaction,
        op_neo_meta_tx_get_next_nonce,
        op_neo_abstract_account_create,
        op_neo_abstract_account_get,
        op_neo_abstract_account_execute_operation,
        op_neo_abstract_account_get_operation_status,
        op_neo_abstract_account_get_next_nonce,
    ],
    esm_entry_point = "ext:r3e/r3e.js",
    esm = [dir "src/js", "r3e.js", "encoding.js", "infra.js", "time.js", "neo.js", "oracle.js", "tee.js", "neo_services.js"]
);

// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
#[allow(clippy::unused_async)]
#[js_op(async(lazy), fast)]
pub async fn op_defer() {}
