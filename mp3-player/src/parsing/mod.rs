use mp4box::MP4Box;

use crate::errors;

pub mod container;
pub mod descriptor;
pub mod mp4box;
pub mod spec;
pub mod utils;

pub trait ParseBox: Sized {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<Self, errors::Error>;
}

pub trait CtorContainerBox: Sized {
    fn ctor(boxes: Vec<MP4Box>) -> Result<Self, errors::Error>;
}

pub trait ParseContainerBox<const TYP: u32>: CtorContainerBox {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<MP4Box, errors::Error>;

    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let mut boxes = Vec::new();

        if size == 0 {
            let (typ, box_size, hdr_size) = stream.read_box_common_header(size).await?;
            let child = Self::parse_child(stream, typ, box_size - hdr_size).await?;
            boxes.push(child);
        } else {
            while size > 0 {
                let (typ, box_size, hdr_size) = stream.read_box_common_header(size).await?;
                let child = Self::parse_child(stream, typ, box_size - hdr_size).await?;
                boxes.push(child);
                size -= box_size;
            }
        }

        Self::ctor(boxes)
    }
}
