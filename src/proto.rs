use proto_rs::DecodeError;
use proto_rs::ProtoShadow;
use proto_rs::proto_message;

use crate::Decisol;
use crate::QuoteLamports;
use crate::QuoteLamportsKind;
use crate::SolanaLamports;
use crate::SolanaLamportsKind;

#[proto_message(proto_path = "protos/decisol.proto", sun = QuoteLamports)]
pub struct QuoteLamportssProto {
    pub amount: u64,
    pub kind: QuoteLamportsKind,
}

impl ProtoShadow<QuoteLamports> for QuoteLamportssProto {
    type Sun<'a> = &'a QuoteLamports;
    type OwnedSun = QuoteLamports;
    type View<'a> = Self;

    fn to_sun(self) -> Result<Self::OwnedSun, DecodeError> {
        Ok(self.kind.value(self.amount))
    }

    fn from_sun(value: Self::Sun<'_>) -> Self::View<'_> {
        Self {
            amount: value.amount(),
            kind: value.kind(),
        }
    }
}

#[proto_message(proto_path = "protos/decisol.proto", sun = SolanaLamports)]
pub struct SolanaLamportsProto {
    pub amount: u64,
    pub kind: SolanaLamportsKind,
}

impl ProtoShadow<SolanaLamports> for SolanaLamportsProto {
    type Sun<'a> = &'a SolanaLamports;
    type OwnedSun = SolanaLamports;
    type View<'a> = Self;

    fn to_sun(self) -> Result<Self::OwnedSun, DecodeError> {
        Ok(self.kind.value(self.amount))
    }

    fn from_sun(value: Self::Sun<'_>) -> Self::View<'_> {
        Self {
            amount: value.amount(),
            kind: value.kind(),
        }
    }
}
