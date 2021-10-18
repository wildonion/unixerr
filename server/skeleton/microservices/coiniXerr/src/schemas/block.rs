


use argon2::{self, Config, ThreadMode, Variant, Version};
use serde::{Serialize, Deserialize};
use uuid::Uuid;




#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chain{
    pub branch_id: Uuid,
    pub branch_name: String,
    pub blocks: Vec<Block>,
}

impl Default for Chain{
    pub fn default() -> Self{
        Chain{
            branch_id: Uuid::new_v4(),
            branch_name: "main".to_string(),
            blocks: vec![Block::default()],
        }
    }
}

impl Chain{

    pub fn new(branch_id: Uuid, branch_id: String, blocks: Vec<Block>) -> Self{
        Chain{
            branch_id,
            branch_name,
            blocks,
        }
    }

    pub fn add(&mut self, block: Block) -> Self{
        self.blocks.push(block);
        Chain{
            branch_id: self.branch_id,
            branch_name: self.branch_name,
            blocks: self.blocks,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block{
    pub id: Uuid,
    pub is_genesis: bool
    pub prev_hash: String,
    pub hash: String,
    pub merkle_root: String, //-- hash of all transactions in the form of a binary tree-like structure called merkle tree such that each hash is linked to its parent following a parent-child tree-like relation
    pub nonce: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>, //-- can't implement the Copy trait for Vec thus can't bound it to the Block structure 
    pub is_mined: bool,
}

impl Default for Block{
    fn default() -> Self{
        Block{
            id: Uuid::new_v4(),
            is_genesis: true
            prev_hash: "hash of pervious block".to_string(), // TODO -
            hash: "hash of current block".to_string(), // TODO -
            nonce: 0,
            timestamp: chrono::Local::now().naive_local().timestamp(),
            transactions: vec![Transaction::default()],
            is_mined: true,
        }
    }
}













// NOTE - all fields of a union share common storage and writes to one field of a union can overwrite its other fields, and size of a union is determined by the size of its largest field
// NOTE - there is no way for the compiler to guarantee that you always read the correct type (that is, the most recently written type) from the union
// NOTE - enums use some extra memory to keep track of the enum variant, with unions we keep track of the current active field ourself
unsafe impl Send for TransactionMem {} //-- due to unsafeness manner of C based raw pointers we implement the Send trait for TransactionMem union in order to be shareable between tokio threads
union TransactionMem{
    pub data: *mut self::Transaction, //-- defining the data as a raw mutable pointer to a mutable Transaction object, changing the data will change the Transaction object and vice versa
    pub buffer: *const u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction{
    pub id: Uuid,
    pub amount: i32,
    pub from_address: String,
    pub to_address: String,
    pub issued: i64,
    pub signed: Option<i64>,
    pub hash: String, // TODO -
}

impl Default for Transaction{
    fn default() -> Self{
        Transaction{
            id: Uuid::new_v4(),
            amount: 100,
            from_address: "genesis wallet address here".to_string(), // TODO - 
            to_address: "a lucky user wallet address here".to_string(), // TODO - 
            issued: chrono::Local::now().naive_local().timestamp(),
            signed: Some(chrono::Local::now().naive_local().timestamp()),
            hash: "hash of the current transaction".to_string(), // TODO -
        }
    }
}

impl Transaction{
    pub fn new(buffer: &[u8]) -> Result<&mut Self, Box<dyn std::error::Error>>{
        unsafe{
            let transaction = TransactionMem{buffer: buffer.as_ptr() as *const u8}; //-- filling the buffer field will also fill the data cause thay have a same memory storage
            let deserialized_transaction = &mut *transaction.data; //-- since the data inside the union is a pointer to a mutable Transaction object we have to return a dereferenced of the data which is a mutable object of Transaction
            Ok(deserialized_transaction)
        }
    }
}

