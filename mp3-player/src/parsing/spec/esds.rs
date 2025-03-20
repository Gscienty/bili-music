use futures::io::Cursor;
use tokio_util::bytes::Bytes;

use crate::{
    errors,
    parsing::{ParseBox, descriptor::MPEG4Descriptor, utils},
};

pub const ESDS: u32 = utils::box_type_u32(['e', 's', 'd', 's']);
#[derive(Debug)]
pub struct ElementaryStreamDescriptorBox {
    descriptor: MPEG4Descriptor,
}

impl ParseBox for ElementaryStreamDescriptorBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<Self, errors::Error> {
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(stream.read_u8().await?);
        }

        let mut description_stream = utils::BoxStream(tokio_util::io::StreamReader::new(
            tokio_stream::once(tokio::io::Result::Ok(Bytes::from_iter(data.into_iter()))),
        ));

        let descriptor = MPEG4Descriptor::parse(&mut description_stream).await?;

        Ok(Self { descriptor })
    }
}
