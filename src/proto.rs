use proto_rs::DecodeError;
use proto_rs::ProtoShadowDecode;
use proto_rs::ProtoShadowEncode;
use proto_rs::proto_message;

use crate::Decisol;
use crate::QuoteLamports;
use crate::QuoteLamportsKind;
use crate::SolanaLamports;
use crate::SolanaLamportsKind;

#[proto_message(proto_path = "protos/decisol.proto", sun = QuoteLamports)]
pub struct QuoteLamportsProto {
    pub amount: u64,
    pub kind: QuoteLamportsKind,
}

impl<'a> ProtoShadowEncode<'a, QuoteLamports> for QuoteLamportsProto {
    fn from_sun(value: &'a QuoteLamports) -> Self {
        Self {
            amount: value.amount(),
            kind: value.kind(),
        }
    }
}
impl ProtoShadowDecode<QuoteLamports> for QuoteLamportsProto {
    fn to_sun(self) -> Result<QuoteLamports, DecodeError> {
        Ok(self.kind.value(self.amount))
    }
}

#[proto_message(proto_path = "protos/decisol.proto", sun = SolanaLamports)]
pub struct SolanaLamportsProto {
    pub amount: u64,
    pub kind: SolanaLamportsKind,
}

impl<'a> ProtoShadowEncode<'a, SolanaLamports> for SolanaLamportsProto {
    fn from_sun(value: &'a SolanaLamports) -> Self {
        Self {
            amount: value.amount(),
            kind: value.kind(),
        }
    }
}
impl ProtoShadowDecode<SolanaLamports> for SolanaLamportsProto {
    fn to_sun(self) -> Result<SolanaLamports, DecodeError> {
        Ok(self.kind.value(self.amount))
    }
}
