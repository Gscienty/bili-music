use crate::{
    errors,
    parsing::{ParseBox, mp4box, utils},
};

pub const TREP: u32 = utils::box_type_u32(['t', 'r', 'e', 'p']);
#[derive(Debug)]
pub struct TrackExtensionPropertiesBox {
    track_id: u32,
    boxes: Vec<mp4box::MP4Box>,
}

impl TrackExtensionPropertiesBox {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown trep box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseBox for TrackExtensionPropertiesBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let _ = stream.read_box_version_flag_header().await?;
        let track_id = stream.read_u32().await?;
        size -= 8;
        let mut boxes = Vec::new();

        while size > 0 {
            let (typ, box_size, hdr_size) = stream.read_box_common_header(size).await?;
            let child = Self::parse_child(stream, typ, size).await?;
            boxes.push(child);

            size -= box_size;
        }

        Ok(Self { track_id, boxes })
    }
}
