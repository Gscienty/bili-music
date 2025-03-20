use crate::{
    errors,
    parsing::{ParseBox, utils},
};

use super::sample_groups::SampleGroup;

pub const SGPD: u32 = utils::box_type_u32(['s', 'g', 'p', 'd']);
#[derive(Debug)]
pub struct SampleGroupDescriptionBox {
    entries: Vec<SampleGroup>,
}

impl ParseBox for SampleGroupDescriptionBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        let grouping_type = stream.read_box_type().await?;
        let default_length = if version == 1 {
            stream.read_u32().await?
        } else {
            0
        };
        let default_group_description_index = if version >= 2 {
            stream.read_u32().await?
        } else {
            0
        };

        let mut entries = Vec::new();
        let entry_count = stream.read_u32().await?;
        for _ in 0..entry_count {
            let description_length = if version == 1 {
                if default_length == 0 {
                    stream.read_u32().await?
                } else {
                    default_length
                }
            } else {
                default_length
            } as usize;
            entries.push(SampleGroup::parse(stream, grouping_type, description_length).await?);
        }

        Ok(Self { entries })
    }
}
