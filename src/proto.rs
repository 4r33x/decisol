use proto_rs::HasProto;
use proto_rs::proto_dump;

use crate::Decisol;
use crate::QuoteLamports;
use crate::QuoteLamportsKind;
use crate::QuoteLamportsKindProto;
use crate::SolanaLamports;
use crate::SolanaLamportsKind;
use crate::SolanaLamportsKindProto;

#[proto_dump(file = "protos/decisol.proto")]
#[derive(prost::Message, Clone, PartialEq)]
pub struct QuoteLamportsProto {
    #[prost(uint64, tag = 1)]
    pub amount: u64,
    #[prost(enumeration = "QuoteLamportsKindProto", tag = 2i32)]
    pub kind: i32,
}

impl HasProto for QuoteLamports {
    type Proto = QuoteLamportsProto;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            amount: self.amount(),
            kind: self.kind().to_proto() as i32,
        }
    }

    fn from_proto(proto: Self::Proto) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Self::new(proto.amount, QuoteLamportsKind::try_from(proto.kind)?))
    }
}

#[proto_dump(file = "protos/decisol.proto")]
#[derive(prost::Message, Clone, PartialEq)]
pub struct SolanaLamportsProto {
    #[prost(uint64, tag = 1)]
    pub amount: u64,
    #[prost(enumeration = "SolanaLamportsKindProto", tag = 2i32)]
    pub kind: i32,
}

impl HasProto for SolanaLamports {
    type Proto = SolanaLamportsProto;

    fn to_proto(&self) -> Self::Proto {
        Self::Proto {
            amount: self.amount(),
            kind: self.kind().to_proto() as i32,
        }
    }

    fn from_proto(proto: Self::Proto) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        Ok(Self::new(proto.amount, SolanaLamportsKind::try_from(proto.kind)?))
    }
}
