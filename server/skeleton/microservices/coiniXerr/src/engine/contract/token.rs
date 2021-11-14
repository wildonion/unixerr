




// NOTE - declarative macros are written using macro_rules!
// NOTE - procedural macros are custom derive: #[derive(CustomDerive)], attribute-like: #[CustomAttribute], and function-like: custom!(...)
// TODO - substrate ft and nft smart contract on local chain 
// TODO - different kind of arguments passing structure with arbitrary numbers of them using macros
// https://medium.com/block-journal/introducing-substrate-smart-contracts-with-ink-d486289e2b59 
// https://stackoverflow.com/questions/60345904/defining-a-macro-that-passes-params-to-a-function
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://blog.logrocket.com/procedural-macros-in-rust/
// http://gradebot.org/doc/ipur/trait.html
// https://cheats.rs/#behind-the-scenes





/* 

                        -----------------------------------------------------------------------------------
                            MINTING TOKEN BASED SMART CONTRACT FOR DIGITAL ASSETS USING coiniXerr COIN
                        -----------------------------------------------------------------------------------


        ==========--------------==========--------------==========--------------==========--------------==========--------------
        | CRC20 and CRC21 (coiniXerr request for comment) traits are used to issue CRC20 and CRC21 FT and NFT based
        | smart contracts respectively to invest on tokenized asstes like musics, notes and even coiniXerr gold coins 
        | which can be stored, sent, sold and bought using coiniXerr wallet address to put them on CVM(coiniXerr Virtual Machine)
        | blockchain like regular transactions on top of the uniXerr network.
        ==========--------------==========--------------==========--------------==========--------------==========--------------

*/




pub trait CRC21{ //-- smart contract for none fungible tokens or digital assets in which a token links ownership to unique physical or digital items

    type AssetLink; //-- stored in IPFS or CVM(coiniXerr Virtual Machine) blockchain
    type TokenName;
    type TokenID;
    type MetaData; //-- metadata schema in json format about the place of stored NFT

    fn mint(&mut self); //-- minting NFT is a transaction and means assigning a token or an asset value to a wallet address which can be issued by smart contracts
    fn transfer_from(&mut self); //-- transfer token from a sender to a recipient
    fn balance_of(&mut self); //-- provides the number of tokens held by a given address
    fn approve(&mut self);  //-- a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information
    fn allowance(&mut self); //-- provides the number of tokens allowed to be transferred from a given address by another given address
    fn trade(&mut self); //-- do something after successfull token transfer

}


pub trait CRC20{ //-- smart contract for fungible tokens or digital assets in which a token or the asset is a mapping between wallet addresses and their balances

    type TokenID;
    type TokenName;
    type TotalSupply; //-- total value or price of the asset - sum of all addresses' balance must be equal to this; eg : an asset with total supply of 40 can be divided into 4 addresses 10 value for each means thos addresses must invest 10 coins to own this asset 
    type Decimal;

    fn mint(&mut self); //-- minting FT is a transaction and means assigning a token or an asset value to a wallet address which can be issued by smart contracts
    fn transfer_from(&mut self); //-- transfer token from a sender to a recipient
    fn balance_of(&mut self); //-- provides the number of tokens held by a given address
    fn approve(&mut self);  //-- a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information
    fn allowance(&mut self); //-- provides the number of tokens allowed to be transferred from a given address by another given address
    fn trade(&mut self); //-- do something after successfull token transfer

}

