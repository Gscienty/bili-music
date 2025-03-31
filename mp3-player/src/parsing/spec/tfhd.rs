use crate::{
    errors,
    parsing::{ParseBox, utils},
};

const FLAG_BASE_DATA_OFFSET: [u8; 3] = [0x00, 0x00, 0x01];
const FLAG_SAMPLE_DESC: [u8; 3] = [0x00, 0x00, 0x02];
const FLAG_SAMPLE_DUR: [u8; 3] = [0x00, 0x00, 0x08];
const FLAG_SAMPLE_SIZE: [u8; 3] = [0x00, 0x00, 0x10];
const FLAG_SAMPLE_FLAGS: [u8; 3] = [0x00, 0x00, 0x20];
const FLAG_DUR_EMPTY: [u8; 3] = [0x01, 0x00, 0x00];
const FLAG_DEFAULT_BASE_IS_MOOF: [u8; 3] = [0x02, 0x00, 0x00];

pub const TFHD: u32 = utils::box_type_u32(['t', 'f', 'h', 'd']);
#[derive(Debug)]
pub struct TrackFragmentHeaderBox {
    track_id: u32,
    base_data_offset: u64,
    default_sample_description_index: u32,
    default_sample_duration: u32,
    default_sample_size: u32,
    default_sample_flags: u32,
}

impl TrackFragmentHeaderBox {
    pub const fn get_track_id(&self) -> u32 {
        self.track_id
    }

    pub const fn get_default_sample_duration(&self) -> u32 {
        self.default_sample_duration
    }
}

impl ParseBox for TrackFragmentHeaderBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        _typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let (_, flag) = stream.read_box_version_flag_header().await?;

        let track_id = stream.read_u32().await?;
        size -= 8;

        let base_data_offset = if size > 0 && utils::check_flag(&flag, &FLAG_BASE_DATA_OFFSET) {
            let data = stream.read_u64().await?;
            size -= 8;

            data
        } else {
            0
        };

        let default_sample_description_index =
            if size > 0 && utils::check_flag(&flag, &FLAG_SAMPLE_DESC) {
                let data = stream.read_u32().await?;
                size -= 4;

                data
            } else {
                0
            };

        let default_sample_duration = if size > 0 && utils::check_flag(&flag, &FLAG_SAMPLE_DUR) {
            let data = stream.read_u32().await?;
            size -= 4;

            data
        } else {
            0
        };

        let default_sample_size = if size > 0 && utils::check_flag(&flag, &FLAG_SAMPLE_SIZE) {
            let data = stream.read_u32().await?;
            size -= 4;

            data
        } else {
            0
        };

        let default_sample_flags = if size > 0 && utils::check_flag(&flag, &FLAG_SAMPLE_FLAGS) {
            let data = stream.read_u32().await?;
            size -= 4;

            data
        } else {
            0
        };

        Ok(Self {
            track_id,
            base_data_offset,
            default_sample_description_index,
            default_sample_duration,
            default_sample_size,
            default_sample_flags,
        })
    }
}
