use crate::errors;

use super::{
    ParseBox, ParseContainerBox,
    container::{ContainerBox, MOOF, MOOV},
    mp4box,
    spec::{
        free::{FREE, FreeSpaceBox},
        ftyp::{FTYP, FileTypeBox},
        mdat::{MDAT, MediaDataBox},
        sidx::{CompressedSegmentIndexBox, SIDX},
    },
    utils,
};

pub async fn parse_box(
    stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
) -> Result<mp4box::MP4Box, errors::Error> {
    let (typ, size, hdr_size) = stream.read_box_common_header(0).await?;

    let size = size - hdr_size;
    let child = match typ.0 {
        FTYP => mp4box::MP4Box::FileType(FileTypeBox::parse(stream, typ, size).await?),
        FREE => mp4box::MP4Box::FreeSpace(FreeSpaceBox::parse(stream, typ, size).await?),
        MOOV => mp4box::MP4Box::CompressedMovie(ContainerBox::<MOOV>::parse(stream, size).await?),
        SIDX => mp4box::MP4Box::CompressedSegmentIndex(
            CompressedSegmentIndexBox::parse(stream, typ, size).await?,
        ),
        MOOF => mp4box::MP4Box::CompressedMovieFragment(
            ContainerBox::<MOOF>::parse(stream, size).await?,
        ),
        MDAT => mp4box::MP4Box::MediaData(MediaDataBox::parse(stream, typ, size).await?),
        _ => {
            return Err(errors::Error::IOError(format!(
                "unknown root box type: {typ}"
            )));
        }
    };

    Ok(child)
}
