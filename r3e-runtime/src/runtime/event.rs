// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved



#[derive(Debug)]
pub struct EventListener {
    /// like: bitcoin, ethereum etc.
    pub topic: String,

    /// like: TxExecution, BlockCreation, etc.
    pub subtopics: Vec<String>,
}