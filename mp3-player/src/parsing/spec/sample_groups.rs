use crate::{errors, parsing::utils};

pub const ROLL: u32 = utils::box_type_u32(['r', 'o', 'l', 'l']);

#[derive(Debug)]
pub enum SampleGroup {
    Roll(SampleGroupRoll),
}

#[derive(Debug)]
pub struct SampleGroupRoll {
    roll_distance: u16,
}

impl SampleGroupRoll {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
    ) -> Result<Self, errors::Error> {
        let roll_distance = stream.read_u16().await?;

        Ok(Self { roll_distance })
    }
}

impl SampleGroup {
    pub(super) async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        description_length: usize,
    ) -> Result<Self, errors::Error> {
        let group = match typ.0 {
            ROLL => SampleGroup::Roll(SampleGroupRoll::parse(stream).await?),
            _ => {
                return Err(errors::Error::IOError(format!(
                    "sample group unknown type: {typ}"
                )));
            }
        };

        Ok(group)
    }
}
