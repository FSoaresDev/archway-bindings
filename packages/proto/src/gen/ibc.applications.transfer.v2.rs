// @generated
/// FungibleTokenPacketData defines a struct for the packet payload
/// See FungibleTokenPacketData spec:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FungibleTokenPacketData {
    /// the token denomination to be transferred
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// the token amount to be transferred
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// the sender address
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
    /// optional memo
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
}
impl ::prost::Name for FungibleTokenPacketData {
    const NAME: &'static str = "FungibleTokenPacketData";
    const PACKAGE: &'static str = "ibc.applications.transfer.v2";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.applications.transfer.v2.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `ibc.applications.transfer.v2` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf4, 0x07, 0x0a, 0x29, 0x69, 0x62, 0x63, 0x2f, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x2f, 0x76,
    0x32, 0x2f, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1c,
    0x69, 0x62, 0x63, 0x2e, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x2e, 0x76, 0x32, 0x22, 0x8f, 0x01, 0x0a,
    0x17, 0x46, 0x75, 0x6e, 0x67, 0x69, 0x62, 0x6c, 0x65, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x50, 0x61,
    0x63, 0x6b, 0x65, 0x74, 0x44, 0x61, 0x74, 0x61, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6e, 0x6f,
    0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x12, 0x16,
    0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
    0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x12, 0x1a,
    0x0a, 0x08, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x08, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x65,
    0x6d, 0x6f, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6d, 0x65, 0x6d, 0x6f, 0x42, 0x39,
    0x5a, 0x37, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73,
    0x6d, 0x6f, 0x73, 0x2f, 0x69, 0x62, 0x63, 0x2d, 0x67, 0x6f, 0x2f, 0x76, 0x37, 0x2f, 0x6d, 0x6f,
    0x64, 0x75, 0x6c, 0x65, 0x73, 0x2f, 0x61, 0x70, 0x70, 0x73, 0x2f, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x66, 0x65, 0x72, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0xd3, 0x05, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04,
    0x00, 0x4e, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x04, 0x00, 0x4e, 0x0a, 0xd5, 0x01,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x14, 0x01, 0x1a, 0xc8, 0x01, 0x20, 0x46, 0x75,
    0x6e, 0x67, 0x69, 0x62, 0x6c, 0x65, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x50, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x44, 0x61, 0x74, 0x61, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20,
    0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70,
    0x61, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x0a, 0x20, 0x53,
    0x65, 0x65, 0x20, 0x46, 0x75, 0x6e, 0x67, 0x69, 0x62, 0x6c, 0x65, 0x54, 0x6f, 0x6b, 0x65, 0x6e,
    0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x44, 0x61, 0x74, 0x61, 0x20, 0x73, 0x70, 0x65, 0x63, 0x3a,
    0x0a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
    0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x69, 0x62, 0x63, 0x2f,
    0x74, 0x72, 0x65, 0x65, 0x2f, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2f, 0x73, 0x70, 0x65, 0x63,
    0x2f, 0x61, 0x70, 0x70, 0x2f, 0x69, 0x63, 0x73, 0x2d, 0x30, 0x32, 0x30, 0x2d, 0x66, 0x75, 0x6e,
    0x67, 0x69, 0x62, 0x6c, 0x65, 0x2d, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2d, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x66, 0x65, 0x72, 0x23, 0x64, 0x61, 0x74, 0x61, 0x2d, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74,
    0x75, 0x72, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08,
    0x1f, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x13, 0x1a, 0x2a,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x64, 0x65, 0x6e, 0x6f, 0x6d,
    0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0b, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0b, 0x11, 0x12, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02,
    0x14, 0x1a, 0x24, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x61, 0x6d,
    0x6f, 0x75, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0d, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x12,
    0x13, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02, 0x14, 0x1a, 0x14,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x09, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x12, 0x13, 0x0a, 0x3d,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x11, 0x02, 0x16, 0x1a, 0x30, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x73, 0x74, 0x69,
    0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x11, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x11, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x11, 0x14, 0x15, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x13, 0x02, 0x12, 0x1a, 0x0f, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x6d, 0x65, 0x6d, 0x6f, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x13, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x13, 0x09,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x13, 0x10, 0x11, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
