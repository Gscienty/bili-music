use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const SIDX: u32 = utils::box_type_u32(['s', 'i', 'd', 'x']);
#[derive(Debug)]
pub struct CompressedSegmentIndexBox {
    reference_id: u32,
    timescale: u32,
    earliest_presentation_time: u64,
    first_offset: u64,

    references: Vec<Reference>,
}

#[derive(Debug)]
pub struct Reference {
    reference_type: u8,
    references_size: u32,
    subsegment_duration: u32,
    starts_with_sap: u8,
    sap_type: u8,
    sap_delta_time: u32,
}

impl ParseBox for CompressedSegmentIndexBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        let reference_id = stream.read_u32().await?;
        let timescale = stream.read_u32().await?;
        let (earliest_presentation_time, first_offset) = if version == 0 {
            let earliest_presentation_time = stream.read_u32().await? as u64;
            let first_offset = stream.read_u32().await? as u64;

            (earliest_presentation_time, first_offset)
        } else {
            let earliest_presentation_time = stream.read_u64().await?;
            let first_offset = stream.read_u64().await?;

            (earliest_presentation_time, first_offset)
        };
        let _ = stream.read_u16().await?;

        let mut references = Vec::new();
        let reference_count = stream.read_u16().await?;
        for _ in 0..reference_count {
            let tmp = stream.read_u32().await?;
            let reference_type = ((tmp >> 31) & 0x01) as u8;
            let references_size = tmp & 0x7fff_ffff;
            let subsegment_duration = stream.read_u32().await?;
            let tmp = stream.read_u32().await?;
            let starts_with_sap = ((tmp >> 31) & 0x01) as u8;
            let sap_type = ((tmp >> 28) & 0x07) as u8;
            let sap_delta_time = tmp & 0x00ff_ffff;

            references.push(Reference {
                reference_type,
                references_size,
                subsegment_duration,
                starts_with_sap,
                sap_type,
                sap_delta_time,
            })
        }

        Ok(Self {
            reference_id,
            timescale,
            earliest_presentation_time,
            first_offset,
            references,
        })
    }
}

impl CompressedSegmentIndexBox {
    pub fn get_reference(
        &self,
        offset: usize,
        mut index: usize,
    ) -> Option<((usize, usize), &Reference)> {
        let mut begin = offset + self.first_offset as usize;
        let mut end = offset + self.first_offset as usize;

        if index >= self.references.len() {
            return None;
        }

        for reference in self.references.iter() {
            begin = end + 1;
            end = begin + reference.references_size as usize - 1;

            if index == 0 {
                return Some(((begin, end), reference));
            }
            index -= 1;
        }

        None
    }

    pub fn count(&self) -> usize {
        self.references.len()
    }
}
