use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const HDLR: u32 = utils::box_type_u32(['h', 'd', 'l', 'r']);

pub const VIDE: u32 = utils::box_type_u32(['v', 'i', 'd', 'e']);
pub const SOUN: u32 = utils::box_type_u32(['s', 'o', 'u', 'n']);
#[derive(Debug)]
pub struct HandlerBox {
    handler: utils::BoxType,
    name: String,
}

impl HandlerBox {
    pub const fn get_handler_type(&self) -> utils::BoxType {
        self.handler
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
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

            let name = std::str::from_utf8(&name)
                .map_err(|err| errors::Error::IOError(err.to_string()))?
                .to_string();

            Ok(Self { handler, name })
        } else {
            Ok(Self {
                handler: utils::BoxType(0),
                name: String::new(),
            })
        }
    }
}
