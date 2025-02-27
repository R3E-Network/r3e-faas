// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::Duration;

use crate::source::*;

pub struct NeoTaskSource {
    sleep: Duration,
    uid: u64,
    count: u64,
    // TODO: Add Neo client from NeoRust SDK
}

impl NeoTaskSource {
    pub fn new(sleep: Duration, uid: u64) -> Self {
        Self {
            sleep,
            uid,
            count: 0,
        }
    }
}

#[async_trait::async_trait]
impl TaskSource for NeoTaskSource {
    async fn acquire_task(&mut self, _uid: u64, _fid_hint: u64) -> Result<Task, TaskError> {
        tokio::time::sleep(self.sleep).await;

        self.count += 1;
        let fid = 1 + (self.count & 1);
        
        // TODO: Replace with actual Neo block data from NeoRust SDK
        let neo_block = NeoBlock {
            header: Some(NeoBlockHeader {
                hash: format!("neo_block_hash_{}", self.count),
                version: 0,
                prev_block_hash: "previous_block_hash".to_string(),
                merkle_root: "merkle_root".to_string(),
                time: 0,
                nonce: 0,
                height: self.count as u32,
                primary: 0,
                next_consensus: "next_consensus".to_string(),
                witnesses: vec![],
            }),
            txs: vec![],
        };
        
        let event = event::Event::NeoBlock(neo_block);

        Ok(Task::new(self.uid, fid, event))
    }

    async fn acquire_fn(&mut self, _uid: u64, _fid: u64) -> Result<Func, FuncError> {
        let code = format!(
            r#"
            const delay = (n) => new Promise(r => setTimeout(r, n));

            export default async function(event) {{
                console.log("Neo event handler:", event);
                await delay(2100);
                console.log("Neo block height:", event.header?.height);
            }}"#
        );
        Ok(Func {
            version: 1,
            code: code.into(),
        })
    }
}
