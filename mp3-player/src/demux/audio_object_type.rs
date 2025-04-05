use crate::errors;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AudioObjectType {
    AacMain = 1,                                       // AAC Main Profile
    AacLowComplexity = 2,                              // AAC Low Complexity
    AacScalableSampleRate = 3,                         // AAC Scalable Sample Rate
    AacLongTermPrediction = 4,                         // AAC Long Term Predictor
    SpectralBandReplication = 5,                       // Spectral band Replication
    AACScalable = 6,                                   // AAC Scalable
    TwinVQ = 7,                                        // Twin VQ
    CodeExcitedLinearPrediction = 8,                   // CELP
    HarmonicVectorExcitationCoding = 9,                // HVXC
    TextToSpeechtInterface = 12,                       // TTSI
    MainSynthetic = 13,                                // Main Synthetic
    WavetableSynthesis = 14,                           // Wavetable Synthesis
    GeneralMIDI = 15,                                  // General MIDI
    AlgorithmicSynthesis = 16,                         // Algorithmic Synthesis
    ErrorResilientAacLowComplexity = 17,               // ER AAC LC
    ErrorResilientAacLongTermPrediction = 19,          // ER AAC LTP
    ErrorResilientAacScalable = 20,                    // ER AAC Scalable
    ErrorResilientAacTwinVQ = 21,                      // ER AAC TwinVQ
    ErrorResilientAacBitSlicedArithmeticCoding = 22,   // ER Bit Sliced Arithmetic Coding
    ErrorResilientAacLowDelay = 23,                    // ER AAC Low Delay
    ErrorResilientCodeExcitedLinearPrediction = 24,    // ER CELP
    ErrorResilientHarmonicVectorExcitationCoding = 25, // ER HVXC
    ErrorResilientHarmonicIndividualLinesNoise = 26,   // ER HILN
    ErrorResilientParametric = 27,                     // ER Parametric
    SinuSoidalCoding = 28,                             // SSC
    ParametricStereo = 29,                             // PS
    MpegSurround = 30,                                 // MPEG Surround
    MpegLayer1 = 32,                                   // MPEG Layer 1
    MpegLayer2 = 33,                                   // MPEG Layer 2
    MpegLayer3 = 34,                                   // MPEG Layer 3
    DirectStreamTransfer = 35,                         // DST Direct Stream Transfer
    AudioLosslessCoding = 36,                          // ALS Audio Lossless Coding
    ScalableLosslessCoding = 37,                       // SLC Scalable Lossless Coding
    ScalableLosslessCodingNoneCore = 38,               // SLC non-core
    ErrorResilientAacEnhancedLowDelay = 39,            // ER AAC ELD
    SymbolicMusicRepresentationSimple = 40,            // SMR Simple
    SymbolicMusicRepresentationMain = 41,              // SMR Main
    UnifiedSpeechAudioCoding = 42,                     // USAC
    SpatialAudioObjectCoding = 43,                     // SAOC
    LowDelayMpegSurround = 44,                         // LD MPEG Surround
    SpatialAudioObjectCodingDialogueEnhancement = 45,  // SAOC-DE
    AudioSync = 46,                                    // Audio Sync
}

impl TryFrom<u8> for AudioObjectType {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<AudioObjectType, Self::Error> {
        match value {
            1 => Ok(AudioObjectType::AacMain),
            2 => Ok(AudioObjectType::AacLowComplexity),
            3 => Ok(AudioObjectType::AacScalableSampleRate),
            4 => Ok(AudioObjectType::AacLongTermPrediction),
            5 => Ok(AudioObjectType::SpectralBandReplication),
            6 => Ok(AudioObjectType::AACScalable),
            7 => Ok(AudioObjectType::TwinVQ),
            8 => Ok(AudioObjectType::CodeExcitedLinearPrediction),
            9 => Ok(AudioObjectType::HarmonicVectorExcitationCoding),
            12 => Ok(AudioObjectType::TextToSpeechtInterface),
            13 => Ok(AudioObjectType::MainSynthetic),
            14 => Ok(AudioObjectType::WavetableSynthesis),
            15 => Ok(AudioObjectType::GeneralMIDI),
            16 => Ok(AudioObjectType::AlgorithmicSynthesis),
            17 => Ok(AudioObjectType::ErrorResilientAacLowComplexity),
            19 => Ok(AudioObjectType::ErrorResilientAacLongTermPrediction),
            20 => Ok(AudioObjectType::ErrorResilientAacScalable),
            21 => Ok(AudioObjectType::ErrorResilientAacTwinVQ),
            22 => Ok(AudioObjectType::ErrorResilientAacBitSlicedArithmeticCoding),
            23 => Ok(AudioObjectType::ErrorResilientAacLowDelay),
            24 => Ok(AudioObjectType::ErrorResilientCodeExcitedLinearPrediction),
            25 => Ok(AudioObjectType::ErrorResilientHarmonicVectorExcitationCoding),
            26 => Ok(AudioObjectType::ErrorResilientHarmonicIndividualLinesNoise),
            27 => Ok(AudioObjectType::ErrorResilientParametric),
            28 => Ok(AudioObjectType::SinuSoidalCoding),
            29 => Ok(AudioObjectType::ParametricStereo),
            30 => Ok(AudioObjectType::MpegSurround),
            32 => Ok(AudioObjectType::MpegLayer1),
            33 => Ok(AudioObjectType::MpegLayer2),
            34 => Ok(AudioObjectType::MpegLayer3),
            35 => Ok(AudioObjectType::DirectStreamTransfer),
            36 => Ok(AudioObjectType::AudioLosslessCoding),
            37 => Ok(AudioObjectType::ScalableLosslessCoding),
            38 => Ok(AudioObjectType::ScalableLosslessCodingNoneCore),
            39 => Ok(AudioObjectType::ErrorResilientAacEnhancedLowDelay),
            40 => Ok(AudioObjectType::SymbolicMusicRepresentationSimple),
            41 => Ok(AudioObjectType::SymbolicMusicRepresentationMain),
            42 => Ok(AudioObjectType::UnifiedSpeechAudioCoding),
            43 => Ok(AudioObjectType::SpatialAudioObjectCoding),
            44 => Ok(AudioObjectType::LowDelayMpegSurround),
            45 => Ok(AudioObjectType::SpatialAudioObjectCodingDialogueEnhancement),
            46 => Ok(AudioObjectType::AudioSync),
            _ => Err(errors::Error::InternalError(
                "invalid audio object type".to_string(),
            )),
        }
    }
}
