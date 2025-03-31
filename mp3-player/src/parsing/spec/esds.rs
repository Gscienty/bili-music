use std::collections::HashMap;

use futures::io::Cursor;
use tokio_util::bytes::Bytes;

use crate::{
    errors,
    parsing::{ParseBox, descriptor::MPEG4Descriptor, utils},
};

pub const ESDS: u32 = utils::box_type_u32(['e', 's', 'd', 's']);
#[derive(Debug, Clone)]
pub struct ElementaryStreamDescriptorBox {
    descriptors: HashMap<u8, MPEG4Descriptor>,
}

impl ParseBox for ElementaryStreamDescriptorBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let _ = stream.read_box_version_flag_header().await?;

        let mut descriptors = HashMap::new();

        size -= 4;
        while size > 0 {
            let tag = stream.read_u8().await?;
            let mut byte_read = stream.read_u8().await?;
            size -= 2;

            let mut descriptor_size = 0usize;
            while (byte_read & 0x80) != 0 {
                descriptor_size = (descriptor_size << 7) + (byte_read & 0x7f) as usize;
                byte_read = stream.read_u8().await?;
                size -= 1;
            }
            descriptor_size = (descriptor_size << 7) + (byte_read & 0x7f) as usize;

            let (descriptor, consumed_size) = MPEG4Descriptor::parse(stream, tag, size).await?;
            size -= consumed_size;

            descriptors.insert(tag, descriptor);
        }

        Ok(Self { descriptors })
    }
}
