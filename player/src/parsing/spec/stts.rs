use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const STTS: u32 = utils::box_type_u32(['s', 't', 't', 's']);
#[derive(Debug)]
pub struct TimeToSampleBox {
    sample_counts: Vec<u32>,
    sample_deltas: Vec<u32>,
}

impl ParseBox for TimeToSampleBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        let mut sample_counts = Vec::new();
        let mut sample_deltas = Vec::new();
        let entry_count = stream.read_u32().await?;
        if version == 0 {
            for _ in 0..entry_count {
                sample_counts.push(stream.read_u32().await?);
                let delta = stream.read_u32().await?;
                if (delta & 0x80000000) != 0 {
                    sample_deltas.push(1);
                } else {
                    sample_deltas.push(delta);
                }
            }
        }

        Ok(Self {
            sample_counts,
            sample_deltas,
        })
    }
}
