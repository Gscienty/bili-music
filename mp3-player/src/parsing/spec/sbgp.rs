use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const SBGP: u32 = utils::box_type_u32(['s', 'b', 'g', 'p']);
#[derive(Debug)]
pub struct SampleToGroupBox {
    grouping_type: utils::BoxType,
    grouping_type_parameter: u32,
    entries: Vec<SampleToGroupEntry>,
}

#[derive(Debug)]
pub struct SampleToGroupEntry {
    sample_count: u32,
    group_description_index: u32,
}

impl ParseBox for SampleToGroupBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;
        let grouping_type = stream.read_box_type().await?;

        let grouping_type_parameter = if version == 1 {
            stream.read_u32().await?
        } else {
            0
        };

        let entry_count = stream.read_u32().await?;
        let mut entries = Vec::new();
        for _ in 0..entry_count {
            let sample_count = stream.read_u32().await?;
            let group_description_index = stream.read_u32().await?;

            entries.push(SampleToGroupEntry {
                sample_count,
                group_description_index,
            })
        }

        Ok(Self {
            grouping_type,
            grouping_type_parameter,
            entries,
        })
    }
}
