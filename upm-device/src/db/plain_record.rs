use heapless::{String, Vec};
use minicbor::{Decode, Encode};
use nameof::name_of;
use rekv::{Entity, Id};

#[derive(Encode, Decode)]
pub struct PlainRecord {
    #[n(0)]
    pub id: Id<PlainRecord>,

    #[n(1)]
    #[cbor(with = "minicbor_adapters")]
    pub name: String<64>,

    #[n(2)]
    #[cbor(with = "minicbor_adapters")]
    pub data: Vec<u8, 16384>, // 16 Kb
}

impl Entity for PlainRecord {
    type CBOR_MAX_LEN = typenum::U32768; // 32 Kb

    const RAW_TABLE_ID: u8 = 1;

    const DEBUG_NAME: &str = name_of!(type PlainRecord);

    fn id(&self) -> Id<Self> {
        self.id
    }
}
