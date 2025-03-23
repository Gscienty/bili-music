use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const MDAT: u32 = utils::box_type_u32(['m', 'd', 'a', 't']);
#[derive(Debug)]
pub struct MediaDataBox {
    data: Vec<u8>,
}

impl ParseBox for MediaDataBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<Self, errors::Error> {
        let mut data = vec![0u8; size];
        stream.read_exact(&mut data).await?;

        Ok(Self { data })
    }
}
