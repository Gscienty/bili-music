use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const HDLR: u32 = utils::box_type_u32(['h', 'd', 'l', 'r']);
#[derive(Debug)]
pub struct HandlerBox {
    handler: utils::BoxType,
    name: Vec<u8>,
}

impl ParseBox for HandlerBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        mut size: usize,
    ) -> Result<Self, errors::Error> {
        let (version, _) = stream.read_box_version_flag_header().await?;

        if version == 0 {
            let _ = stream.read_u32().await?;
            let handler = stream.read_box_type().await?;
            for _ in 0..3 {
                let _ = stream.read_u32().await?;
            }
            size -= 24;
            let mut name = Vec::with_capacity(size);
            for _ in 0..size {
                name.push(stream.read_u8().await?);
            }

            Ok(Self { handler, name })
        } else {
            Ok(Self {
                handler: utils::BoxType(0),
                name: Vec::new(),
            })
        }
    }
}
