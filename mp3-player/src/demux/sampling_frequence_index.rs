use crate::errors;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SamplingFrequenceIndex {
    Freq96000 = 0x0,
    Freq88200 = 0x1,
    Freq64000 = 0x2,
    Freq48000 = 0x3,
    Freq44100 = 0x4,
    Freq32000 = 0x5,
    Freq24000 = 0x6,
    Freq22050 = 0x7,
    Freq16000 = 0x8,
    Freq12000 = 0x9,
    Freq11025 = 0xa,
    Freq8000 = 0xb,
    Freq7350 = 0xc,
}

impl TryFrom<u32> for SamplingFrequenceIndex {
    type Error = errors::Error;
    fn try_from(value: u32) -> Result<SamplingFrequenceIndex, Self::Error> {
        match value {
            0x0 => Ok(SamplingFrequenceIndex::Freq96000),
            0x1 => Ok(SamplingFrequenceIndex::Freq88200),
            0x2 => Ok(SamplingFrequenceIndex::Freq64000),
            0x3 => Ok(SamplingFrequenceIndex::Freq48000),
            0x4 => Ok(SamplingFrequenceIndex::Freq44100),
            0x5 => Ok(SamplingFrequenceIndex::Freq32000),
            0x6 => Ok(SamplingFrequenceIndex::Freq24000),
            0x7 => Ok(SamplingFrequenceIndex::Freq22050),
            0x8 => Ok(SamplingFrequenceIndex::Freq16000),
            0x9 => Ok(SamplingFrequenceIndex::Freq12000),
            0xa => Ok(SamplingFrequenceIndex::Freq11025),
            0xb => Ok(SamplingFrequenceIndex::Freq8000),
            0xc => Ok(SamplingFrequenceIndex::Freq7350),
            _ => Err(errors::Error::InternalError(
                "invalid sampling frequency index".to_string(),
            )),
        }
    }
}

impl SamplingFrequenceIndex {
    pub const fn sample_rate(&self) -> u32 {
        match *self {
            Self::Freq96000 => 96000,
            Self::Freq88200 => 88200,
            Self::Freq64000 => 64000,
            Self::Freq48000 => 48000,
            Self::Freq44100 => 44100,
            Self::Freq32000 => 32000,
            Self::Freq24000 => 24000,
            Self::Freq22050 => 22050,
            Self::Freq16000 => 16000,
            Self::Freq12000 => 12000,
            Self::Freq11025 => 11025,
            Self::Freq8000 => 8000,
            Self::Freq7350 => 7350,
        }
    }
}
