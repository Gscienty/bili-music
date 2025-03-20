use crate::errors;

use super::{
    CtorContainerBox, ParseBox, ParseContainerBox, mp4box,
    spec::{
        dref::{DREF, DataReferenceBox},
        elst::{ELST, EditListBox},
        free::{FREE, FreeSpaceBox},
        ftyp::{FTYP, FileTypeBox},
        hdlr::{HDLR, HandlerBox},
        mdhd::{MDHD, MediaHeaderBox},
        mehd::{MEHD, MovieExtendsHeaderBox},
        meta::{META, MetaBox},
        mvhd::{MVHD, MovieHeaderBox},
        sgpd::SGPD,
        smhd::{SMHD, SoundMediaHeaderBox},
        stco::{ChunkOffsetBox, STCO},
        stsc::{STSC, SampleToChunkBox},
        stsd::{STSD, SampleDescriptionBox},
        stsz::{STSZ, SampleSizeBox},
        stts::{STTS, TimeToSampleBox},
        tkhd::{TKHD, TrackHeaderBox},
        trep::{TREP, TrackExtensionPropertiesBox},
        trex::{TREX, TrackExtendsBox},
    },
    utils,
};

pub const MOOV: u32 = utils::box_type_u32(['m', 'o', 'o', 'v']);
pub const TRAK: u32 = utils::box_type_u32(['t', 'r', 'a', 'k']);
pub const EDTS: u32 = utils::box_type_u32(['e', 'd', 't', 's']);
pub const MDIA: u32 = utils::box_type_u32(['m', 'd', 'i', 'a']);
pub const MINF: u32 = utils::box_type_u32(['m', 'i', 'n', 'f']);
pub const DINF: u32 = utils::box_type_u32(['d', 'i', 'n', 'f']);
pub const STBL: u32 = utils::box_type_u32(['s', 't', 'b', 'l']);
pub const MVEX: u32 = utils::box_type_u32(['m', 'v', 'e', 'x']);
pub const UDTA: u32 = utils::box_type_u32(['u', 'd', 't', 'a']);

#[derive(Debug)]
pub struct ContainerBox<const TYP: u32> {}

impl<const TYP: u32> CtorContainerBox for ContainerBox<TYP> {
    fn ctor(_boxes: Vec<mp4box::MP4Box>) -> Result<Self, errors::Error> {
        Ok(Self {})
    }
}

impl ParseContainerBox<0> for ContainerBox<0> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            FTYP => mp4box::MP4Box::FileType(FileTypeBox::parse(stream, typ, size).await?),
            FREE => mp4box::MP4Box::FreeSpace(FreeSpaceBox::parse(stream, typ, size).await?),
            MOOV => {
                mp4box::MP4Box::CompressedMovie(ContainerBox::<MOOV>::parse(stream, size).await?)
            }
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown root box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<MOOV> for ContainerBox<MOOV> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            TRAK => mp4box::MP4Box::Track(ContainerBox::<TRAK>::parse(stream, size).await?),
            MVEX => mp4box::MP4Box::MovieExtends(ContainerBox::<MVEX>::parse(stream, size).await?),
            MVHD => mp4box::MP4Box::MovieHeader(MovieHeaderBox::parse(stream, typ, size).await?),
            UDTA => mp4box::MP4Box::UserData(ContainerBox::<UDTA>::parse(stream, size).await?),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown moov box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<TRAK> for ContainerBox<TRAK> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            EDTS => mp4box::MP4Box::Edit(ContainerBox::<EDTS>::parse(stream, size).await?),
            MDIA => mp4box::MP4Box::Media(ContainerBox::<MDIA>::parse(stream, size).await?),
            TKHD => mp4box::MP4Box::TrackHeader(TrackHeaderBox::parse(stream, typ, size).await?),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown trak box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<EDTS> for ContainerBox<EDTS> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            ELST => mp4box::MP4Box::EditList(EditListBox::parse(stream, typ, size).await?),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown edts box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<MDIA> for ContainerBox<MDIA> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            MINF => {
                mp4box::MP4Box::MediaInformation(ContainerBox::<MINF>::parse(stream, size).await?)
            }
            MDHD => mp4box::MP4Box::MediaHeader(MediaHeaderBox::parse(stream, typ, size).await?),
            HDLR => mp4box::MP4Box::Handler(HandlerBox::parse(stream, typ, size).await?),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown mdia box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<MINF> for ContainerBox<MINF> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            DINF => {
                mp4box::MP4Box::DataInformation(ContainerBox::<DINF>::parse(stream, size).await?)
            }
            STBL => mp4box::MP4Box::SampleTable(ContainerBox::<STBL>::parse(stream, size).await?),
            SMHD => mp4box::MP4Box::SoundMediaHeader(
                SoundMediaHeaderBox::parse(stream, typ, size).await?,
            ),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown minf box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<DINF> for ContainerBox<DINF> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            DREF => {
                mp4box::MP4Box::DataReference(DataReferenceBox::parse(stream, typ, size).await?)
            }
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown dinf box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<STBL> for ContainerBox<STBL> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            STSD => mp4box::MP4Box::SampleDescription(
                SampleDescriptionBox::parse(stream, typ, size).await?,
            ),
            STTS => mp4box::MP4Box::TimeToSample(TimeToSampleBox::parse(stream, typ, size).await?),
            STSC => {
                mp4box::MP4Box::SampleToChunk(SampleToChunkBox::parse(stream, typ, size).await?)
            }
            STSZ => mp4box::MP4Box::SampleSize(SampleSizeBox::parse(stream, typ, size).await?),
            STCO => mp4box::MP4Box::ChunkOffset(ChunkOffsetBox::parse(stream, typ, size).await?),
            SGPD => mp4box::MP4Box::SampleGroupDescription(
                super::spec::sgpd::SampleGroupDescriptionBox::parse(stream, typ, size).await?,
            ),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown stbl box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<MVEX> for ContainerBox<MVEX> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            MEHD => mp4box::MP4Box::MovieExtendsHeader(
                MovieExtendsHeaderBox::parse(stream, typ, size).await?,
            ),
            TREX => mp4box::MP4Box::TrackExtends(TrackExtendsBox::parse(stream, typ, size).await?),
            TREP => mp4box::MP4Box::TrackExtensionProperties(
                TrackExtensionPropertiesBox::parse(stream, typ, size).await?,
            ),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown mvex box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}

impl ParseContainerBox<UDTA> for ContainerBox<UDTA> {
    async fn parse_child(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<mp4box::MP4Box, errors::Error> {
        let child = match typ.0 {
            META => mp4box::MP4Box::Meta(MetaBox::parse(stream, typ, size).await?),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "unknown udta box type: {typ}"
                )));
            }
        };

        Ok(child)
    }
}
