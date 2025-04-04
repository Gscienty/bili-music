use crate::{
    errors,
    parsing::{ParseBox, utils},
};

pub const FTYP: u32 = utils::box_type_u32(['f', 't', 'y', 'p']);
#[derive(Debug)]
pub struct FileTypeBox {
    pub(crate) major_brand: utils::BoxType,
    pub(crate) minor_version: u32,
    pub(crate) compatible_brands: Vec<utils::BoxType>,
}

impl ParseBox for FileTypeBox {
    async fn parse(
        stream: &mut utils::BoxStream<impl tokio::io::AsyncReadExt + Unpin>,
        typ: utils::BoxType,
        size: usize,
    ) -> Result<Self, errors::Error> {
        let major_brand = stream.read_box_type().await?;
        let minor_version = stream.read_u32().await?;

        let mut toparse = size - 8;
        let mut compatible_brands = Vec::with_capacity(toparse / 4);
        while toparse >= 4 {
            compatible_brands.push(stream.read_box_type().await?);
            toparse -= 4;
        }

        Ok(Self {
            major_brand,
            minor_version,
            compatible_brands,
        })
    }
}
