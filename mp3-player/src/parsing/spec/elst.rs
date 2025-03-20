use crate::{
    errors,
    parsing::{ParseBox, mp4box, utils},
};

pub const ELST: u32 = utils::box_type_u32(['e', 'l', 's', 't']);
#[derive(Debug)]
pub struct EditListBox {
    entries: Vec<EditListEntry>,
}

#[derive(Debug)]
pub struct EditListEntry {
    segment_duration: u64,
    media_time: u64,
    media_rate_integer: u16,
    media_rate_fraction: u16,
}

impl ParseBox for EditListBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        _size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        let mut entries = Vec::new();
        let entry_count = stream.read_u32().await?;
        for _ in 0..entry_count {
            let (segment_duration, media_time) = if version == 1 {
                let segment_duration = stream.read_u64().await?;
                let media_time = stream.read_u64().await?;

                (segment_duration, media_time)
            } else {
                let segment_duration = stream.read_u32().await? as u64;
                let media_time = stream.read_u32().await? as u64;

                (segment_duration, media_time)
            };
            let media_rate_integer = stream.read_u16().await?;
            let media_rate_fraction = stream.read_u16().await?;

            entries.push(EditListEntry {
                segment_duration,
                media_time,
                media_rate_integer,
                media_rate_fraction,
            });
        }

        Ok(Self { entries })
    }
}
