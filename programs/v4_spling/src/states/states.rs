use crate::*;

#[account]
pub struct SenderLog {
    pub active: bool,       // 1 byte
    pub status: u8,         // 1 byte
    pub ts: i64,            // 8 byte
    pub counter: u16,       // 2 byte
} 

#[account]
pub struct Operation {
    pub status: u8,         // 1 byte
    pub index: u8,          // 1 byte
    pub title: String,
    pub description: String,
}

#[account]
pub struct Schema {
    pub status: u8,         // 1 byte
    pub index: u8,          // 1 byte
    pub title: String,
    pub description: String,
}

#[account]
pub struct Payer {
    pub bump: u8,           // 1 byte
}
