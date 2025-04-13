use crate::{
    errors,
    parsing::{
        ParseBox, ParseContainerBox,
        container::{ContainerBox, ILST},
        mp4box, utils,
    },
};

use super::hdlr::{HDLR, HandlerBox};

pub const META: u32 = utils::box_type_u32(['m', 'e', 't', 'a']);
#[derive(Debug)]
pub struct MetaBox {
    boxes: Vec<mp4box::MP4Box>,
}

impl MetaBox {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            HDLR => mp4box::MP4Box::Handler(HandlerBox::parse(stream, typ, size).await?),
            ILST => mp4box::MP4Box::AppleListItem(ContainerBox::<ILST>::parse(stream, size).await?),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown meta box type: {typ} {size}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseBox for MetaBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let _ = stream.read_box_version_flag_header().await?;
        size -= 4;

        let mut boxes = Vec::new();
        while size > 0 {
            let (typ, box_size, hdr_size) = stream.read_box_common_header(size).await?;
            let child = Self::parse_child(stream, typ, box_size - hdr_size).await?;
            boxes.push(child);
            size -= box_size;
        }
        Ok(Self { boxes })
    }
}
