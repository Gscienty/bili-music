use std::collections::HashMap;

use futures::io::Cursor;
use tokio_util::bytes::Bytes;

use crate::{
    errors,
    parsing::{ParseBox, utils},
};

use super::descriptor;

pub const ESDS: u32 = utils::box_type_u32(['e', 's', 'd', 's']);
#[derive(Debug, Clone)]
pub struct ElementaryStreamDescriptorBox {
    descriptor: descriptor::ESDescriptor,
}

impl ParseBox for ElementaryStreamDescriptorBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let _ = stream.read_box_version_flag_header().await?;
        size -= 4;

        let (tag, desc_size, hdr_size) = descriptor::parse_descriptor_header(stream).await?;
        if !matches!(tag, 0x03) {
            return Err(errors::Error::IOError("ESDescriptor not found".to_string()));
        }
        let (descriptor, desc_size) = descriptor::ESDescriptor::parse(stream, desc_size).await?;
        size -= hdr_size + desc_size;

        stream.skip_bytes(size).await?;

        Ok(Self { descriptor })
    }
}
