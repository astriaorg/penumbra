#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(
        oneof = "request::Value",
        tags = "1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17"
    )]
    pub value: ::core::option::Option<request::Value>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Echo(super::RequestEcho),
        #[prost(message, tag = "2")]
        Flush(super::RequestFlush),
        #[prost(message, tag = "3")]
        Info(super::RequestInfo),
        #[prost(message, tag = "5")]
        InitChain(super::RequestInitChain),
        #[prost(message, tag = "6")]
        Query(super::RequestQuery),
        #[prost(message, tag = "7")]
        BeginBlock(super::RequestBeginBlock),
        #[prost(message, tag = "8")]
        CheckTx(super::RequestCheckTx),
        #[prost(message, tag = "9")]
        DeliverTx(super::RequestDeliverTx),
        #[prost(message, tag = "10")]
        EndBlock(super::RequestEndBlock),
        #[prost(message, tag = "11")]
        Commit(super::RequestCommit),
        #[prost(message, tag = "12")]
        ListSnapshots(super::RequestListSnapshots),
        #[prost(message, tag = "13")]
        OfferSnapshot(super::RequestOfferSnapshot),
        #[prost(message, tag = "14")]
        LoadSnapshotChunk(super::RequestLoadSnapshotChunk),
        #[prost(message, tag = "15")]
        ApplySnapshotChunk(super::RequestApplySnapshotChunk),
        #[prost(message, tag = "16")]
        PrepareProposal(super::RequestPrepareProposal),
        #[prost(message, tag = "17")]
        ProcessProposal(super::RequestProcessProposal),
    }
}
impl ::prost::Name for Request {
    const NAME: &'static str = "Request";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEcho {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for RequestEcho {
    const NAME: &'static str = "RequestEcho";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestFlush {}
impl ::prost::Name for RequestFlush {
    const NAME: &'static str = "RequestFlush";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInfo {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_version: u64,
    #[prost(uint64, tag = "3")]
    pub p2p_version: u64,
    #[prost(string, tag = "4")]
    pub abci_version: ::prost::alloc::string::String,
}
impl ::prost::Name for RequestInfo {
    const NAME: &'static str = "RequestInfo";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInitChain {
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub consensus_params: ::core::option::Option<super::types::ConsensusParams>,
    #[prost(message, repeated, tag = "4")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(bytes = "vec", tag = "5")]
    pub app_state_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "6")]
    pub initial_height: i64,
}
impl ::prost::Name for RequestInitChain {
    const NAME: &'static str = "RequestInitChain";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQuery {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub height: i64,
    #[prost(bool, tag = "4")]
    pub prove: bool,
}
impl ::prost::Name for RequestQuery {
    const NAME: &'static str = "RequestQuery";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBeginBlock {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<super::types::Header>,
    #[prost(message, optional, tag = "3")]
    pub last_commit_info: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag = "4")]
    pub byzantine_validators: ::prost::alloc::vec::Vec<Misbehavior>,
}
impl ::prost::Name for RequestBeginBlock {
    const NAME: &'static str = "RequestBeginBlock";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCheckTx {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "CheckTxType", tag = "2")]
    pub r#type: i32,
}
impl ::prost::Name for RequestCheckTx {
    const NAME: &'static str = "RequestCheckTx";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDeliverTx {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for RequestDeliverTx {
    const NAME: &'static str = "RequestDeliverTx";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEndBlock {
    #[prost(int64, tag = "1")]
    pub height: i64,
}
impl ::prost::Name for RequestEndBlock {
    const NAME: &'static str = "RequestEndBlock";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCommit {}
impl ::prost::Name for RequestCommit {
    const NAME: &'static str = "RequestCommit";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// lists available snapshots
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestListSnapshots {}
impl ::prost::Name for RequestListSnapshots {
    const NAME: &'static str = "RequestListSnapshots";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// offers a snapshot to the application
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestOfferSnapshot {
    /// snapshot offered by peers
    #[prost(message, optional, tag = "1")]
    pub snapshot: ::core::option::Option<Snapshot>,
    /// light client-verified app hash for snapshot height
    #[prost(bytes = "vec", tag = "2")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for RequestOfferSnapshot {
    const NAME: &'static str = "RequestOfferSnapshot";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// loads a snapshot chunk
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLoadSnapshotChunk {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub format: u32,
    #[prost(uint32, tag = "3")]
    pub chunk: u32,
}
impl ::prost::Name for RequestLoadSnapshotChunk {
    const NAME: &'static str = "RequestLoadSnapshotChunk";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// Applies a snapshot chunk
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestApplySnapshotChunk {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
}
impl ::prost::Name for RequestApplySnapshotChunk {
    const NAME: &'static str = "RequestApplySnapshotChunk";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPrepareProposal {
    /// the modified transactions cannot exceed this size.
    #[prost(int64, tag = "1")]
    pub max_tx_bytes: i64,
    /// txs is an array of transactions that will be included in a block,
    /// sent to the app for possible modifications.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "3")]
    pub local_last_commit: ::core::option::Option<ExtendedCommitInfo>,
    #[prost(message, repeated, tag = "4")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    #[prost(int64, tag = "5")]
    pub height: i64,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the validator proposing the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for RequestPrepareProposal {
    const NAME: &'static str = "RequestPrepareProposal";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestProcessProposal {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "2")]
    pub proposed_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag = "3")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    /// hash is the merkle root hash of the fields of the proposed block.
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "5")]
    pub height: i64,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the original proposer of the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for RequestProcessProposal {
    const NAME: &'static str = "RequestProcessProposal";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(
        oneof = "response::Value",
        tags = "1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18"
    )]
    pub value: ::core::option::Option<response::Value>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Exception(super::ResponseException),
        #[prost(message, tag = "2")]
        Echo(super::ResponseEcho),
        #[prost(message, tag = "3")]
        Flush(super::ResponseFlush),
        #[prost(message, tag = "4")]
        Info(super::ResponseInfo),
        #[prost(message, tag = "6")]
        InitChain(super::ResponseInitChain),
        #[prost(message, tag = "7")]
        Query(super::ResponseQuery),
        #[prost(message, tag = "8")]
        BeginBlock(super::ResponseBeginBlock),
        #[prost(message, tag = "9")]
        CheckTx(super::ResponseCheckTx),
        #[prost(message, tag = "10")]
        DeliverTx(super::ResponseDeliverTx),
        #[prost(message, tag = "11")]
        EndBlock(super::ResponseEndBlock),
        #[prost(message, tag = "12")]
        Commit(super::ResponseCommit),
        #[prost(message, tag = "13")]
        ListSnapshots(super::ResponseListSnapshots),
        #[prost(message, tag = "14")]
        OfferSnapshot(super::ResponseOfferSnapshot),
        #[prost(message, tag = "15")]
        LoadSnapshotChunk(super::ResponseLoadSnapshotChunk),
        #[prost(message, tag = "16")]
        ApplySnapshotChunk(super::ResponseApplySnapshotChunk),
        #[prost(message, tag = "17")]
        PrepareProposal(super::ResponsePrepareProposal),
        #[prost(message, tag = "18")]
        ProcessProposal(super::ResponseProcessProposal),
    }
}
impl ::prost::Name for Response {
    const NAME: &'static str = "Response";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// nondeterministic
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseException {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
impl ::prost::Name for ResponseException {
    const NAME: &'static str = "ResponseException";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEcho {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for ResponseEcho {
    const NAME: &'static str = "ResponseEcho";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFlush {}
impl ::prost::Name for ResponseFlush {
    const NAME: &'static str = "ResponseFlush";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseInfo {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub app_version: u64,
    #[prost(int64, tag = "4")]
    pub last_block_height: i64,
    #[prost(bytes = "vec", tag = "5")]
    pub last_block_app_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ResponseInfo {
    const NAME: &'static str = "ResponseInfo";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseInitChain {
    #[prost(message, optional, tag = "1")]
    pub consensus_params: ::core::option::Option<super::types::ConsensusParams>,
    #[prost(message, repeated, tag = "2")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(bytes = "vec", tag = "3")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ResponseInitChain {
    const NAME: &'static str = "ResponseInitChain";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQuery {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// bytes data = 2; // use "value" instead.
    ///
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub index: i64,
    #[prost(bytes = "vec", tag = "6")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "8")]
    pub proof_ops: ::core::option::Option<super::crypto::ProofOps>,
    #[prost(int64, tag = "9")]
    pub height: i64,
    #[prost(string, tag = "10")]
    pub codespace: ::prost::alloc::string::String,
}
impl ::prost::Name for ResponseQuery {
    const NAME: &'static str = "ResponseQuery";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseBeginBlock {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
impl ::prost::Name for ResponseBeginBlock {
    const NAME: &'static str = "ResponseBeginBlock";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCheckTx {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub gas_wanted: i64,
    #[prost(int64, tag = "6")]
    pub gas_used: i64,
    #[prost(message, repeated, tag = "7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag = "8")]
    pub codespace: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "10")]
    pub priority: i64,
    /// mempool_error is set by CometBFT.
    /// ABCI applictions creating a ResponseCheckTX should not set mempool_error.
    #[prost(string, tag = "11")]
    pub mempool_error: ::prost::alloc::string::String,
}
impl ::prost::Name for ResponseCheckTx {
    const NAME: &'static str = "ResponseCheckTx";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDeliverTx {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub gas_wanted: i64,
    #[prost(int64, tag = "6")]
    pub gas_used: i64,
    /// nondeterministic
    #[prost(message, repeated, tag = "7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag = "8")]
    pub codespace: ::prost::alloc::string::String,
}
impl ::prost::Name for ResponseDeliverTx {
    const NAME: &'static str = "ResponseDeliverTx";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEndBlock {
    #[prost(message, repeated, tag = "1")]
    pub validator_updates: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(message, optional, tag = "2")]
    pub consensus_param_updates: ::core::option::Option<super::types::ConsensusParams>,
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
impl ::prost::Name for ResponseEndBlock {
    const NAME: &'static str = "ResponseEndBlock";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCommit {
    /// reserve 1
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "3")]
    pub retain_height: i64,
}
impl ::prost::Name for ResponseCommit {
    const NAME: &'static str = "ResponseCommit";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseListSnapshots {
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::prost::alloc::vec::Vec<Snapshot>,
}
impl ::prost::Name for ResponseListSnapshots {
    const NAME: &'static str = "ResponseListSnapshots";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseOfferSnapshot {
    #[prost(enumeration = "response_offer_snapshot::Result", tag = "1")]
    pub result: i32,
}
/// Nested message and enum types in `ResponseOfferSnapshot`.
pub mod response_offer_snapshot {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result, abort all snapshot restoration
        Unknown = 0,
        /// Snapshot accepted, apply chunks
        Accept = 1,
        /// Abort all snapshot restoration
        Abort = 2,
        /// Reject this specific snapshot, try others
        Reject = 3,
        /// Reject all snapshots of this format, try others
        RejectFormat = 4,
        /// Reject all snapshots from the sender(s), try others
        RejectSender = 5,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unknown => "UNKNOWN",
                Result::Accept => "ACCEPT",
                Result::Abort => "ABORT",
                Result::Reject => "REJECT",
                Result::RejectFormat => "REJECT_FORMAT",
                Result::RejectSender => "REJECT_SENDER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ACCEPT" => Some(Self::Accept),
                "ABORT" => Some(Self::Abort),
                "REJECT" => Some(Self::Reject),
                "REJECT_FORMAT" => Some(Self::RejectFormat),
                "REJECT_SENDER" => Some(Self::RejectSender),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for ResponseOfferSnapshot {
    const NAME: &'static str = "ResponseOfferSnapshot";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseLoadSnapshotChunk {
    #[prost(bytes = "vec", tag = "1")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ResponseLoadSnapshotChunk {
    const NAME: &'static str = "ResponseLoadSnapshotChunk";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseApplySnapshotChunk {
    #[prost(enumeration = "response_apply_snapshot_chunk::Result", tag = "1")]
    pub result: i32,
    /// Chunks to refetch and reapply
    #[prost(uint32, repeated, tag = "2")]
    pub refetch_chunks: ::prost::alloc::vec::Vec<u32>,
    /// Chunk senders to reject and ban
    #[prost(string, repeated, tag = "3")]
    pub reject_senders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ResponseApplySnapshotChunk`.
pub mod response_apply_snapshot_chunk {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result, abort all snapshot restoration
        Unknown = 0,
        /// Chunk successfully accepted
        Accept = 1,
        /// Abort all snapshot restoration
        Abort = 2,
        /// Retry chunk (combine with refetch and reject)
        Retry = 3,
        /// Retry snapshot (combine with refetch and reject)
        RetrySnapshot = 4,
        /// Reject this snapshot, try others
        RejectSnapshot = 5,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unknown => "UNKNOWN",
                Result::Accept => "ACCEPT",
                Result::Abort => "ABORT",
                Result::Retry => "RETRY",
                Result::RetrySnapshot => "RETRY_SNAPSHOT",
                Result::RejectSnapshot => "REJECT_SNAPSHOT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ACCEPT" => Some(Self::Accept),
                "ABORT" => Some(Self::Abort),
                "RETRY" => Some(Self::Retry),
                "RETRY_SNAPSHOT" => Some(Self::RetrySnapshot),
                "REJECT_SNAPSHOT" => Some(Self::RejectSnapshot),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for ResponseApplySnapshotChunk {
    const NAME: &'static str = "ResponseApplySnapshotChunk";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePrepareProposal {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for ResponsePrepareProposal {
    const NAME: &'static str = "ResponsePrepareProposal";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseProcessProposal {
    #[prost(enumeration = "response_process_proposal::ProposalStatus", tag = "1")]
    pub status: i32,
}
/// Nested message and enum types in `ResponseProcessProposal`.
pub mod response_process_proposal {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ProposalStatus {
        Unknown = 0,
        Accept = 1,
        Reject = 2,
    }
    impl ProposalStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProposalStatus::Unknown => "UNKNOWN",
                ProposalStatus::Accept => "ACCEPT",
                ProposalStatus::Reject => "REJECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "ACCEPT" => Some(Self::Accept),
                "REJECT" => Some(Self::Reject),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for ResponseProcessProposal {
    const NAME: &'static str = "ResponseProcessProposal";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitInfo {
    #[prost(int32, tag = "1")]
    pub round: i32,
    #[prost(message, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<VoteInfo>,
}
impl ::prost::Name for CommitInfo {
    const NAME: &'static str = "CommitInfo";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedCommitInfo {
    /// The round at which the block proposer decided in the previous height.
    #[prost(int32, tag = "1")]
    pub round: i32,
    /// List of validators' addresses in the last validator set with their voting
    /// information, including vote extensions.
    #[prost(message, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<ExtendedVoteInfo>,
}
impl ::prost::Name for ExtendedCommitInfo {
    const NAME: &'static str = "ExtendedCommitInfo";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// Event allows application developers to attach additional information to
/// ResponseBeginBlock, ResponseEndBlock, ResponseCheckTx and ResponseDeliverTx.
/// Later, transactions may be queried using these events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<EventAttribute>,
}
impl ::prost::Name for Event {
    const NAME: &'static str = "Event";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// EventAttribute is a single key-value pair, associated with an event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(bool, tag = "3")]
    pub index: bool,
}
impl ::prost::Name for EventAttribute {
    const NAME: &'static str = "EventAttribute";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// TxResult contains results of executing the transaction.
///
/// One usage is indexing transaction results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub result: ::core::option::Option<ResponseDeliverTx>,
}
impl ::prost::Name for TxResult {
    const NAME: &'static str = "TxResult";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// Validator
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    /// The first 20 bytes of SHA256(public key)
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// PubKey pub_key = 2 \[(gogoproto.nullable)=false\];
    ///
    /// The voting power
    #[prost(int64, tag = "3")]
    pub power: i64,
}
impl ::prost::Name for Validator {
    const NAME: &'static str = "Validator";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// ValidatorUpdate
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorUpdate {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<super::crypto::PublicKey>,
    #[prost(int64, tag = "2")]
    pub power: i64,
}
impl ::prost::Name for ValidatorUpdate {
    const NAME: &'static str = "ValidatorUpdate";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
/// VoteInfo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteInfo {
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
    #[prost(bool, tag = "2")]
    pub signed_last_block: bool,
}
impl ::prost::Name for VoteInfo {
    const NAME: &'static str = "VoteInfo";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedVoteInfo {
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
    #[prost(bool, tag = "2")]
    pub signed_last_block: bool,
    /// Reserved for future use
    #[prost(bytes = "vec", tag = "3")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ExtendedVoteInfo {
    const NAME: &'static str = "ExtendedVoteInfo";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehavior {
    #[prost(enumeration = "MisbehaviorType", tag = "1")]
    pub r#type: i32,
    /// The offending validator
    #[prost(message, optional, tag = "2")]
    pub validator: ::core::option::Option<Validator>,
    /// The height when the offense occurred
    #[prost(int64, tag = "3")]
    pub height: i64,
    /// The corresponding time where the offense occurred
    #[prost(message, optional, tag = "4")]
    pub time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Total voting power of the validator set in case the ABCI application does
    /// not store historical validators.
    /// <https://github.com/tendermint/tendermint/issues/4581>
    #[prost(int64, tag = "5")]
    pub total_voting_power: i64,
}
impl ::prost::Name for Misbehavior {
    const NAME: &'static str = "Misbehavior";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// The height at which the snapshot was taken
    #[prost(uint64, tag = "1")]
    pub height: u64,
    /// The application-specific snapshot format
    #[prost(uint32, tag = "2")]
    pub format: u32,
    /// Number of chunks in the snapshot
    #[prost(uint32, tag = "3")]
    pub chunks: u32,
    /// Arbitrary snapshot hash, equal only if identical
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Arbitrary application metadata
    #[prost(bytes = "vec", tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Snapshot {
    const NAME: &'static str = "Snapshot";
    const PACKAGE: &'static str = "tendermint.abci";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("tendermint.abci.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CheckTxType {
    New = 0,
    Recheck = 1,
}
impl CheckTxType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CheckTxType::New => "NEW",
            CheckTxType::Recheck => "RECHECK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NEW" => Some(Self::New),
            "RECHECK" => Some(Self::Recheck),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MisbehaviorType {
    Unknown = 0,
    DuplicateVote = 1,
    LightClientAttack = 2,
}
impl MisbehaviorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MisbehaviorType::Unknown => "UNKNOWN",
            MisbehaviorType::DuplicateVote => "DUPLICATE_VOTE",
            MisbehaviorType::LightClientAttack => "LIGHT_CLIENT_ATTACK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "DUPLICATE_VOTE" => Some(Self::DuplicateVote),
            "LIGHT_CLIENT_ATTACK" => Some(Self::LightClientAttack),
            _ => None,
        }
    }
}
/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod abci_application_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AbciApplicationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AbciApplicationClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AbciApplicationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AbciApplicationClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AbciApplicationClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn echo(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestEcho>,
        ) -> std::result::Result<tonic::Response<super::ResponseEcho>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/Echo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "Echo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn flush(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestFlush>,
        ) -> std::result::Result<tonic::Response<super::ResponseFlush>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/Flush",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "Flush"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn info(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestInfo>,
        ) -> std::result::Result<tonic::Response<super::ResponseInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/Info",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "Info"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn deliver_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestDeliverTx>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseDeliverTx>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/DeliverTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "DeliverTx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestCheckTx>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseCheckTx>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/CheckTx",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "CheckTx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestQuery>,
        ) -> std::result::Result<tonic::Response<super::ResponseQuery>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/Query",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "Query"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn commit(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestCommit>,
        ) -> std::result::Result<tonic::Response<super::ResponseCommit>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/Commit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "Commit"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn init_chain(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestInitChain>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseInitChain>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/InitChain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "InitChain"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn begin_block(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestBeginBlock>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseBeginBlock>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/BeginBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tendermint.abci.ABCIApplication", "BeginBlock"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn end_block(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestEndBlock>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseEndBlock>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/EndBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tendermint.abci.ABCIApplication", "EndBlock"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestListSnapshots>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseListSnapshots>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/ListSnapshots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tendermint.abci.ABCIApplication", "ListSnapshots"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn offer_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestOfferSnapshot>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseOfferSnapshot>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/OfferSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tendermint.abci.ABCIApplication", "OfferSnapshot"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn load_snapshot_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestLoadSnapshotChunk>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseLoadSnapshotChunk>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/LoadSnapshotChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tendermint.abci.ABCIApplication",
                        "LoadSnapshotChunk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn apply_snapshot_chunk(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestApplySnapshotChunk>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseApplySnapshotChunk>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/ApplySnapshotChunk",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tendermint.abci.ABCIApplication",
                        "ApplySnapshotChunk",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn prepare_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestPrepareProposal>,
        ) -> std::result::Result<
            tonic::Response<super::ResponsePrepareProposal>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/PrepareProposal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tendermint.abci.ABCIApplication", "PrepareProposal"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn process_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestProcessProposal>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseProcessProposal>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tendermint.abci.ABCIApplication/ProcessProposal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tendermint.abci.ABCIApplication", "ProcessProposal"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod abci_application_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AbciApplicationServer.
    #[async_trait]
    pub trait AbciApplication: Send + Sync + 'static {
        async fn echo(
            &self,
            request: tonic::Request<super::RequestEcho>,
        ) -> std::result::Result<tonic::Response<super::ResponseEcho>, tonic::Status>;
        async fn flush(
            &self,
            request: tonic::Request<super::RequestFlush>,
        ) -> std::result::Result<tonic::Response<super::ResponseFlush>, tonic::Status>;
        async fn info(
            &self,
            request: tonic::Request<super::RequestInfo>,
        ) -> std::result::Result<tonic::Response<super::ResponseInfo>, tonic::Status>;
        async fn deliver_tx(
            &self,
            request: tonic::Request<super::RequestDeliverTx>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseDeliverTx>,
            tonic::Status,
        >;
        async fn check_tx(
            &self,
            request: tonic::Request<super::RequestCheckTx>,
        ) -> std::result::Result<tonic::Response<super::ResponseCheckTx>, tonic::Status>;
        async fn query(
            &self,
            request: tonic::Request<super::RequestQuery>,
        ) -> std::result::Result<tonic::Response<super::ResponseQuery>, tonic::Status>;
        async fn commit(
            &self,
            request: tonic::Request<super::RequestCommit>,
        ) -> std::result::Result<tonic::Response<super::ResponseCommit>, tonic::Status>;
        async fn init_chain(
            &self,
            request: tonic::Request<super::RequestInitChain>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseInitChain>,
            tonic::Status,
        >;
        async fn begin_block(
            &self,
            request: tonic::Request<super::RequestBeginBlock>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseBeginBlock>,
            tonic::Status,
        >;
        async fn end_block(
            &self,
            request: tonic::Request<super::RequestEndBlock>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseEndBlock>,
            tonic::Status,
        >;
        async fn list_snapshots(
            &self,
            request: tonic::Request<super::RequestListSnapshots>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseListSnapshots>,
            tonic::Status,
        >;
        async fn offer_snapshot(
            &self,
            request: tonic::Request<super::RequestOfferSnapshot>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseOfferSnapshot>,
            tonic::Status,
        >;
        async fn load_snapshot_chunk(
            &self,
            request: tonic::Request<super::RequestLoadSnapshotChunk>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseLoadSnapshotChunk>,
            tonic::Status,
        >;
        async fn apply_snapshot_chunk(
            &self,
            request: tonic::Request<super::RequestApplySnapshotChunk>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseApplySnapshotChunk>,
            tonic::Status,
        >;
        async fn prepare_proposal(
            &self,
            request: tonic::Request<super::RequestPrepareProposal>,
        ) -> std::result::Result<
            tonic::Response<super::ResponsePrepareProposal>,
            tonic::Status,
        >;
        async fn process_proposal(
            &self,
            request: tonic::Request<super::RequestProcessProposal>,
        ) -> std::result::Result<
            tonic::Response<super::ResponseProcessProposal>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AbciApplicationServer<T: AbciApplication> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AbciApplication> AbciApplicationServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AbciApplicationServer<T>
    where
        T: AbciApplication,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/tendermint.abci.ABCIApplication/Echo" => {
                    #[allow(non_camel_case_types)]
                    struct EchoSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestEcho> for EchoSvc<T> {
                        type Response = super::ResponseEcho;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestEcho>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::echo(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/Flush" => {
                    #[allow(non_camel_case_types)]
                    struct FlushSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestFlush> for FlushSvc<T> {
                        type Response = super::ResponseFlush;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestFlush>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::flush(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FlushSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/Info" => {
                    #[allow(non_camel_case_types)]
                    struct InfoSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestInfo> for InfoSvc<T> {
                        type Response = super::ResponseInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestInfo>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/DeliverTx" => {
                    #[allow(non_camel_case_types)]
                    struct DeliverTxSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestDeliverTx>
                    for DeliverTxSvc<T> {
                        type Response = super::ResponseDeliverTx;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestDeliverTx>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::deliver_tx(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeliverTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/CheckTx" => {
                    #[allow(non_camel_case_types)]
                    struct CheckTxSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestCheckTx>
                    for CheckTxSvc<T> {
                        type Response = super::ResponseCheckTx;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestCheckTx>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::check_tx(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/Query" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestQuery> for QuerySvc<T> {
                        type Response = super::ResponseQuery;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestQuery>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::query(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/Commit" => {
                    #[allow(non_camel_case_types)]
                    struct CommitSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestCommit>
                    for CommitSvc<T> {
                        type Response = super::ResponseCommit;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestCommit>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::commit(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CommitSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/InitChain" => {
                    #[allow(non_camel_case_types)]
                    struct InitChainSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestInitChain>
                    for InitChainSvc<T> {
                        type Response = super::ResponseInitChain;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestInitChain>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::init_chain(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitChainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/BeginBlock" => {
                    #[allow(non_camel_case_types)]
                    struct BeginBlockSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestBeginBlock>
                    for BeginBlockSvc<T> {
                        type Response = super::ResponseBeginBlock;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestBeginBlock>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::begin_block(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BeginBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/EndBlock" => {
                    #[allow(non_camel_case_types)]
                    struct EndBlockSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestEndBlock>
                    for EndBlockSvc<T> {
                        type Response = super::ResponseEndBlock;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestEndBlock>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::end_block(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EndBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/ListSnapshots" => {
                    #[allow(non_camel_case_types)]
                    struct ListSnapshotsSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestListSnapshots>
                    for ListSnapshotsSvc<T> {
                        type Response = super::ResponseListSnapshots;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestListSnapshots>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::list_snapshots(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSnapshotsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/OfferSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct OfferSnapshotSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestOfferSnapshot>
                    for OfferSnapshotSvc<T> {
                        type Response = super::ResponseOfferSnapshot;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestOfferSnapshot>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::offer_snapshot(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OfferSnapshotSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/LoadSnapshotChunk" => {
                    #[allow(non_camel_case_types)]
                    struct LoadSnapshotChunkSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestLoadSnapshotChunk>
                    for LoadSnapshotChunkSvc<T> {
                        type Response = super::ResponseLoadSnapshotChunk;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestLoadSnapshotChunk>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::load_snapshot_chunk(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LoadSnapshotChunkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/ApplySnapshotChunk" => {
                    #[allow(non_camel_case_types)]
                    struct ApplySnapshotChunkSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestApplySnapshotChunk>
                    for ApplySnapshotChunkSvc<T> {
                        type Response = super::ResponseApplySnapshotChunk;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestApplySnapshotChunk>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::apply_snapshot_chunk(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ApplySnapshotChunkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/PrepareProposal" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareProposalSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestPrepareProposal>
                    for PrepareProposalSvc<T> {
                        type Response = super::ResponsePrepareProposal;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestPrepareProposal>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::prepare_proposal(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tendermint.abci.ABCIApplication/ProcessProposal" => {
                    #[allow(non_camel_case_types)]
                    struct ProcessProposalSvc<T: AbciApplication>(pub Arc<T>);
                    impl<
                        T: AbciApplication,
                    > tonic::server::UnaryService<super::RequestProcessProposal>
                    for ProcessProposalSvc<T> {
                        type Response = super::ResponseProcessProposal;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestProcessProposal>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AbciApplication>::process_proposal(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProcessProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AbciApplication> Clone for AbciApplicationServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: AbciApplication> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AbciApplication> tonic::server::NamedService for AbciApplicationServer<T> {
        const NAME: &'static str = "tendermint.abci.ABCIApplication";
    }
}
