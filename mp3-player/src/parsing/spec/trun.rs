use crate::{
    errors,
    parsing::{ParseBox, utils},
};

const FLAG_DATA_OFFSET: [u8; 3] = [0x00, 0x00, 0x01];
const FLAG_FIRST_FLAG: [u8; 3] = [0x00, 0x00, 0x04];
const FLAG_DURATION: [u8; 3] = [0x00, 0x01, 0x00];
const FLAG_SIZE: [u8; 3] = [0x00, 0x02, 0x00];
const FLAG_FLAGS: [u8; 3] = [0x00, 0x04, 0x00];
const FLAG_CTS_OFFSET: [u8; 3] = [0x00, 0x08, 0x00];

pub const TRUN: u32 = utils::box_type_u32(['t', 'r', 'u', 'n']);
#[derive(Debug)]
pub struct TrackRunBox {
    data_offset: u32,
    first_sample_flags: u32,
    sample_duration: Vec<u32>,
    sample_size: Vec<u32>,
    sample_flags: Vec<u32>,
    sample_composition_time_offset: Vec<u32>,
}

impl ParseBox for TrackRunBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        _typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let (_, flag) = stream.read_box_version_flag_header().await?;
        let sample_count = stream.read_u32().await? as usize;
        size -= 8;

        let data_offset = if size > 0 && utils::check_flag(&flag, &FLAG_DATA_OFFSET) {
            let value = stream.read_u32().await?;
            size -= 4;
            value
        } else {
            0
        };
        let first_sample_flags = if size > 0 && utils::check_flag(&flag, &FLAG_FIRST_FLAG) {
            let value = stream.read_u32().await?;
            size -= 4;
            value
        } else {
            0
        };

        let mut sample_duration = Vec::with_capacity(sample_count);
        let mut sample_size = Vec::with_capacity(sample_count);
        let mut sample_flags = Vec::with_capacity(sample_count);
        let mut sample_composition_time_offset = Vec::with_capacity(sample_count);

        if size > 0 {
            for _ in 0..sample_count {
                if utils::check_flag(&flag, &FLAG_DURATION) {
                    sample_duration.push(stream.read_u32().await?);
                }
                if utils::check_flag(&flag, &FLAG_SIZE) {
                    sample_size.push(stream.read_u32().await?);
                }
                if utils::check_flag(&flag, &FLAG_FLAGS) {
                    sample_flags.push(stream.read_u32().await?);
                }
                if utils::check_flag(&flag, &FLAG_CTS_OFFSET) {
                    sample_composition_time_offset.push(stream.read_u32().await?);
                }
            }
        }

        Ok(Self {
            data_offset,
            first_sample_flags,
            sample_duration,
            sample_size,
            sample_flags,
            sample_composition_time_offset,
        })
    }
}
