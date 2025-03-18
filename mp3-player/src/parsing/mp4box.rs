use super::container;

use super::container::{DINF, EDTS, MDIA, MINF, MOOV, MVEX, STBL, TRAK};
use super::spec::free::FreeSpaceBox;
use super::spec::ftyp::FileTypeBox;
use super::spec::mvhd::MovieHeaderBox;

#[allow(unused)]
#[derive(Debug)]
pub enum MP4Box {
    CompressedMovie(container::ContainerBox<MOOV>),
    Track(container::ContainerBox<TRAK>),
    Edit(container::ContainerBox<EDTS>),
    Media(container::ContainerBox<MDIA>),
    MediaInformation(container::ContainerBox<MINF>),
    DataInformation(container::ContainerBox<DINF>),
    SampleTable(container::ContainerBox<STBL>),
    MovieExtends(container::ContainerBox<MVEX>),

    FileType(FileTypeBox),
    FreeSpace(FreeSpaceBox),
    MovieHeader(MovieHeaderBox),
}
