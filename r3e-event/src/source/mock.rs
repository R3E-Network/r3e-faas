// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

use std::time::Duration;

use crate::source::*;

pub struct MockTaskSource {
    sleep: Duration,
    uid: u64,
    count: u64,
}

impl MockTaskSource {
    pub fn new(sleep: Duration, uid: u64) -> Self {
        Self {
            sleep,
            uid,
            count: 0,
        }
    }
}

#[async_trait::async_trait]
impl TaskSource for MockTaskSource {
    async fn acquire_task(&mut self, _uid: u64, _fid_hint: u64) -> Result<Task, TaskError> {
        tokio::time::sleep(self.sleep).await;

        self.count += 1;
        let fid = 1 + (self.count & 1);
        let event = event::Event::Mock(MockEvent {
            message: format!("MockEvent {},{}", self.count, fid).into(),
        });

        Ok(Task::new(self.uid, fid, event))
    }

    async fn acquire_fn(&mut self, _uid: u64, _fid: u64) -> Result<Func, FuncError> {
        let code = format!(
            r#"
            const delay = (n) => new Promise(r => setTimeout(r, n));

            export default async function(event) {{
                console.log("hello event-1:", event);
                await delay(2100);
                console.log("hello event-2:", event);
            }}"#
        );
        Ok(Func {
            version: 1,
            code: code.into(),
        })
    }
}
