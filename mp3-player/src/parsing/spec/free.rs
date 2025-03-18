use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const FREE: u32 = utils::box_type_u32(['f', 'r', 'e', 'e']);
#[derive(Debug)]
pub struct FreeSpaceBox {}

impl ParseBox for FreeSpaceBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        while size > 0 {
            let _ = stream.read_u8().await?;

            size -= 1;
        }
        Ok(Self {})
    }
}
