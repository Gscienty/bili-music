use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const TOO: u32 = utils::box_type_u32(['©', 't', 'o', 'o']);
pub const DESC: u32 = utils::box_type_u32(['d', 'e', 's', 'c']);
#[derive(Debug)]
pub struct UTF8AppleDataBox {
    typ: utils::BoxType,
    data: Vec<u8>,
}

impl ParseBox for UTF8AppleDataBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<Self, errors::Error> {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(stream.read_u8().await?);
        }

        Ok(Self { typ, data })
    }
}
