use super::container;

use super::container::{DINF, EDTS, ILST, MDIA, MINF, MOOF, MOOV, MVEX, STBL, TRAF, TRAK, UDTA};
use super::spec::apple_data::UTF8AppleDataBox;
use super::spec::audio_sample::AudioSampleBox;
use super::spec::dref::DataReferenceBox;
use super::spec::elst::EditListBox;
use super::spec::esds::ElementaryStreamDescriptorBox;
use super::spec::free::FreeSpaceBox;
use super::spec::ftyp::FileTypeBox;
use super::spec::hdlr::HandlerBox;
use super::spec::mdat::MediaDataBox;
use super::spec::mdhd::MediaHeaderBox;
use super::spec::mehd::MovieExtendsHeaderBox;
use super::spec::meta::MetaBox;
use super::spec::mfhd::MovieFragmentHeaderBox;
use super::spec::mvhd::MovieHeaderBox;
use super::spec::sbgp::SampleToGroupBox;
use super::spec::sgpd::SampleGroupDescriptionBox;
use super::spec::sidx::CompressedSegmentIndexBox;
use super::spec::smhd::SoundMediaHeaderBox;
use super::spec::stco::ChunkOffsetBox;
use super::spec::stsc::SampleToChunkBox;
use super::spec::stsd::SampleDescriptionBox;
use super::spec::stsz::SampleSizeBox;
use super::spec::stts::TimeToSampleBox;
use super::spec::tfdt::TrackFragmentBaseMediaDecodeTimeBox;
use super::spec::tfhd::TrackFragmentHeaderBox;
use super::spec::tkhd::TrackHeaderBox;
use super::spec::trep::TrackExtensionPropertiesBox;
use super::spec::trex::TrackExtendsBox;
use super::spec::trun::TrackRunBox;
use super::spec::url::DataEntryUrlBox;

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
    UserData(container::ContainerBox<UDTA>),
    AppleListItem(container::ContainerBox<ILST>),
    CompressedMovieFragment(container::ContainerBox<MOOF>),
    TrackFragment(container::ContainerBox<TRAF>),

    FileType(FileTypeBox),
    FreeSpace(FreeSpaceBox),
    MovieHeader(MovieHeaderBox),
    MovieExtendsHeader(MovieExtendsHeaderBox),
    TrackExtends(TrackExtendsBox),
    TrackExtensionProperties(TrackExtensionPropertiesBox),
    TrackHeader(TrackHeaderBox),
    EditList(EditListBox),
    MediaHeader(MediaHeaderBox),
    Handler(HandlerBox),
    SoundMediaHeader(SoundMediaHeaderBox),
    DataReference(DataReferenceBox),
    DataEntryUrl(DataEntryUrlBox),
    SampleDescription(SampleDescriptionBox),
    TimeToSample(TimeToSampleBox),
    SampleToChunk(SampleToChunkBox),
    SampleSize(SampleSizeBox),
    ChunkOffset(ChunkOffsetBox),
    SampleGroupDescription(SampleGroupDescriptionBox),
    Meta(MetaBox),
    UTF8AppleData(UTF8AppleDataBox),
    CompressedSegmentIndex(CompressedSegmentIndexBox),
    MovieFragmentHeader(MovieFragmentHeaderBox),
    TrackFragmentHeader(TrackFragmentHeaderBox),
    TrackFragmentBaseMediaDecodeTime(TrackFragmentBaseMediaDecodeTimeBox),
    SampleToGroup(SampleToGroupBox),
    TrackRun(TrackRunBox),
    MediaData(MediaDataBox),

    AudioSample(AudioSampleBox),
    ElementaryStreamDescriptor(ElementaryStreamDescriptorBox),
}
