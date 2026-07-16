//! Request types shared by sound-effect and text-to-speech generation.
//!
//! These types add the Seed Audio request variants while retaining conversion
//! support for the original ElevenLabs-backed request structs.

use serde::{Deserialize, Serialize};

use crate::error::RunwayError;

use super::generation::{SoundEffectModel, SoundEffectRequest, TextToSpeechRequest};

/// The Seed Audio model discriminator.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SeedAudioModel {
    /// The `seed_audio` API value.
    #[serde(rename = "seed_audio")]
    SeedAudio,
}

/// Output container supported by Seed Audio.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SeedAudioOutputFormat {
    /// WAV output.
    Wav,
    /// MP3 output.
    Mp3,
    /// Ogg with the Opus codec.
    #[serde(rename = "ogg_opus")]
    OggOpus,
}

/// Output sample rate supported by Seed Audio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SeedAudioSampleRate {
    /// 8,000 Hz.
    Hz8000,
    /// 16,000 Hz.
    Hz16000,
    /// 24,000 Hz.
    Hz24000,
    /// 32,000 Hz.
    Hz32000,
    /// 44,100 Hz.
    Hz44100,
    /// 48,000 Hz.
    Hz48000,
}

impl SeedAudioSampleRate {
    /// Every sample rate supported by the pinned official API schema.
    pub const ALL: &'static [Self] = &[
        Self::Hz8000,
        Self::Hz16000,
        Self::Hz24000,
        Self::Hz32000,
        Self::Hz44100,
        Self::Hz48000,
    ];

    /// Return the exact numeric sample rate sent to the API.
    pub const fn as_hz(self) -> u32 {
        match self {
            Self::Hz8000 => 8_000,
            Self::Hz16000 => 16_000,
            Self::Hz24000 => 24_000,
            Self::Hz32000 => 32_000,
            Self::Hz44100 => 44_100,
            Self::Hz48000 => 48_000,
        }
    }
}

impl Serialize for SeedAudioSampleRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.as_hz())
    }
}

impl<'de> Deserialize<'de> for SeedAudioSampleRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match u32::deserialize(deserializer)? {
            8_000 => Ok(Self::Hz8000),
            16_000 => Ok(Self::Hz16000),
            24_000 => Ok(Self::Hz24000),
            32_000 => Ok(Self::Hz32000),
            44_100 => Ok(Self::Hz44100),
            48_000 => Ok(Self::Hz48000),
            value => Err(serde::de::Error::custom(format!(
                "unsupported Seed Audio sample rate: {value}"
            ))),
        }
    }
}

/// A Seed Audio sound-effect request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeedAudioSoundEffectRequest {
    /// Exact model discriminator.
    pub model: SeedAudioModel,
    /// Non-empty scene, dialogue, music, or sound-effect prompt.
    pub prompt_text: String,
    /// Relative loudness; negative is quieter and positive is louder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness_rate: Option<f64>,
    /// Optional output container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<SeedAudioOutputFormat>,
    /// Pitch shift in semitones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_rate: Option<f64>,
    /// Up to three audio URIs referred to as `@Audio1` through `@Audio3`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_audios: Option<Vec<String>>,
    /// Optional output sample rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<SeedAudioSampleRate>,
    /// Relative speech speed; negative is slower and positive is faster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_rate: Option<f64>,
}

impl SeedAudioSoundEffectRequest {
    /// Create a Seed Audio sound-effect request.
    pub fn new(prompt_text: impl Into<String>) -> Self {
        Self {
            model: SeedAudioModel::SeedAudio,
            prompt_text: prompt_text.into(),
            loudness_rate: None,
            output_format: None,
            pitch_rate: None,
            reference_audios: None,
            sample_rate: None,
            speech_rate: None,
        }
    }

    /// Set the relative output loudness.
    pub fn loudness_rate(mut self, loudness_rate: f64) -> Self {
        self.loudness_rate = Some(loudness_rate);
        self
    }

    /// Set the output container.
    pub fn output_format(mut self, output_format: SeedAudioOutputFormat) -> Self {
        self.output_format = Some(output_format);
        self
    }

    /// Set the pitch shift in semitones.
    pub fn pitch_rate(mut self, pitch_rate: f64) -> Self {
        self.pitch_rate = Some(pitch_rate);
        self
    }

    /// Set up to three reference audio URIs.
    pub fn reference_audios(mut self, reference_audios: Vec<String>) -> Self {
        self.reference_audios = Some(reference_audios);
        self
    }

    /// Set the output sample rate.
    pub fn sample_rate(mut self, sample_rate: SeedAudioSampleRate) -> Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    /// Set the relative speech speed.
    pub fn speech_rate(mut self, speech_rate: f64) -> Self {
        self.speech_rate = Some(speech_rate);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_nonempty(&self.prompt_text, "promptText")?;
        validate_finite(self.loudness_rate, "loudnessRate")?;
        validate_finite(self.pitch_rate, "pitchRate")?;
        validate_finite(self.speech_rate, "speechRate")?;
        if let Some(reference_audios) = &self.reference_audios {
            if reference_audios.len() > 3 {
                return validation("seed_audio supports up to 3 referenceAudios");
            }
            for uri in reference_audios {
                validate_nonempty(uri, "referenceAudios[]")?;
            }
        }
        Ok(())
    }
}

/// Exact Eleven Text-to-Sound V2 wire request.
///
/// This compatibility wrapper corrects the legacy `loop_output` field to the
/// official JSON key `loop` when a legacy [`SoundEffectRequest`] is sent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElevenTextToSoundV2Request {
    /// Exact model discriminator.
    pub model: SoundEffectModel,
    /// Description of the sound to generate.
    pub prompt_text: String,
    /// Optional duration from 0.5 through 30 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Whether the generated sound should loop seamlessly.
    #[serde(rename = "loop", skip_serializing_if = "Option::is_none")]
    pub loop_output: Option<bool>,
}

impl ElevenTextToSoundV2Request {
    /// Create an Eleven Text-to-Sound V2 request.
    pub fn new(prompt_text: impl Into<String>) -> Self {
        Self {
            model: SoundEffectModel::ElevenTextToSoundV2,
            prompt_text: prompt_text.into(),
            duration: None,
            loop_output: None,
        }
    }

    /// Set the desired duration in seconds.
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set whether the output should loop seamlessly.
    pub fn loop_output(mut self, loop_output: bool) -> Self {
        self.loop_output = Some(loop_output);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        if self.model != SoundEffectModel::ElevenTextToSoundV2 {
            return validation("ElevenTextToSoundV2Request must use model eleven_text_to_sound_v2");
        }
        if let Some(duration) = self.duration {
            if !duration.is_finite() || !(0.5..=30.0).contains(&duration) {
                return validation("duration must be between 0.5 and 30 seconds");
            }
        }
        Ok(())
    }
}

impl From<SoundEffectRequest> for ElevenTextToSoundV2Request {
    fn from(value: SoundEffectRequest) -> Self {
        Self {
            model: value.model,
            prompt_text: value.prompt_text,
            duration: value.duration,
            loop_output: value.loop_output,
        }
    }
}

/// Any request accepted by the current sound-effect endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SoundEffectCreateRequest {
    /// Seed Audio text-to-audio generation.
    SeedAudio(SeedAudioSoundEffectRequest),
    /// Eleven Text-to-Sound V2 generation.
    ElevenTextToSoundV2(ElevenTextToSoundV2Request),
}

impl SoundEffectCreateRequest {
    /// Validate the selected model-specific request.
    pub fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::SeedAudio(request) => request.validate(),
            Self::ElevenTextToSoundV2(request) => request.validate(),
        }
    }
}

impl From<SeedAudioSoundEffectRequest> for SoundEffectCreateRequest {
    fn from(value: SeedAudioSoundEffectRequest) -> Self {
        Self::SeedAudio(value)
    }
}

impl From<ElevenTextToSoundV2Request> for SoundEffectCreateRequest {
    fn from(value: ElevenTextToSoundV2Request) -> Self {
        Self::ElevenTextToSoundV2(value)
    }
}

impl From<SoundEffectRequest> for SoundEffectCreateRequest {
    fn from(value: SoundEffectRequest) -> Self {
        Self::ElevenTextToSoundV2(value.into())
    }
}

/// Discriminator for a Seed Audio reference voice.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SeedAudioVoiceType {
    /// Clone speech from a reference audio clip.
    #[serde(rename = "reference-audio")]
    ReferenceAudio,
}

/// A reference voice used by Seed Audio text-to-speech.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SeedAudioReferenceVoice {
    /// HTTPS URL or supported Runway URI for the reference clip.
    pub audio_uri: String,
    /// Exact reference-voice discriminator.
    #[serde(rename = "type")]
    pub voice_type: SeedAudioVoiceType,
}

impl SeedAudioReferenceVoice {
    /// Create a reference voice from an audio URI.
    pub fn new(audio_uri: impl Into<String>) -> Self {
        Self {
            audio_uri: audio_uri.into(),
            voice_type: SeedAudioVoiceType::ReferenceAudio,
        }
    }

    /// Validate this voice before transport.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_nonempty(&self.audio_uri, "voice.audioUri")
    }
}

/// A Seed Audio text-to-speech request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeedAudioTextToSpeechRequest {
    /// Exact model discriminator.
    pub model: SeedAudioModel,
    /// Non-empty text to speak.
    pub prompt_text: String,
    /// Relative loudness; negative is quieter and positive is louder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness_rate: Option<f64>,
    /// Optional output container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<SeedAudioOutputFormat>,
    /// Pitch shift in semitones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_rate: Option<f64>,
    /// Optional output sample rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<SeedAudioSampleRate>,
    /// Relative speech speed; negative is slower and positive is faster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_rate: Option<f64>,
    /// Optional voice cloned from one reference clip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<SeedAudioReferenceVoice>,
}

impl SeedAudioTextToSpeechRequest {
    /// Create a Seed Audio text-to-speech request.
    pub fn new(prompt_text: impl Into<String>) -> Self {
        Self {
            model: SeedAudioModel::SeedAudio,
            prompt_text: prompt_text.into(),
            loudness_rate: None,
            output_format: None,
            pitch_rate: None,
            sample_rate: None,
            speech_rate: None,
            voice: None,
        }
    }

    /// Set the relative output loudness.
    pub fn loudness_rate(mut self, loudness_rate: f64) -> Self {
        self.loudness_rate = Some(loudness_rate);
        self
    }

    /// Set the output container.
    pub fn output_format(mut self, output_format: SeedAudioOutputFormat) -> Self {
        self.output_format = Some(output_format);
        self
    }

    /// Set the pitch shift in semitones.
    pub fn pitch_rate(mut self, pitch_rate: f64) -> Self {
        self.pitch_rate = Some(pitch_rate);
        self
    }

    /// Set the output sample rate.
    pub fn sample_rate(mut self, sample_rate: SeedAudioSampleRate) -> Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    /// Set the relative speech speed.
    pub fn speech_rate(mut self, speech_rate: f64) -> Self {
        self.speech_rate = Some(speech_rate);
        self
    }

    /// Clone the output voice from a reference clip.
    pub fn voice(mut self, voice: SeedAudioReferenceVoice) -> Self {
        self.voice = Some(voice);
        self
    }

    /// Validate documented client-checkable constraints.
    pub fn validate(&self) -> Result<(), RunwayError> {
        validate_nonempty(&self.prompt_text, "promptText")?;
        validate_finite(self.loudness_rate, "loudnessRate")?;
        validate_finite(self.pitch_rate, "pitchRate")?;
        validate_finite(self.speech_rate, "speechRate")?;
        if let Some(voice) = &self.voice {
            voice.validate()?;
        }
        Ok(())
    }
}

/// Any request accepted by the current text-to-speech endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum TextToSpeechCreateRequest {
    /// Seed Audio text-to-speech generation.
    SeedAudio(SeedAudioTextToSpeechRequest),
    /// Existing Eleven Multilingual V2 generation.
    ElevenMultilingualV2(TextToSpeechRequest),
}

impl TextToSpeechCreateRequest {
    /// Validate the selected model-specific request.
    pub fn validate(&self) -> Result<(), RunwayError> {
        match self {
            Self::SeedAudio(request) => request.validate(),
            Self::ElevenMultilingualV2(request) => request.validate(),
        }
    }
}

impl From<SeedAudioTextToSpeechRequest> for TextToSpeechCreateRequest {
    fn from(value: SeedAudioTextToSpeechRequest) -> Self {
        Self::SeedAudio(value)
    }
}

impl From<TextToSpeechRequest> for TextToSpeechCreateRequest {
    fn from(value: TextToSpeechRequest) -> Self {
        Self::ElevenMultilingualV2(value)
    }
}

fn validate_nonempty(value: &str, field_name: &str) -> Result<(), RunwayError> {
    if value.trim().is_empty() {
        validation(format!("{field_name} cannot be empty"))
    } else {
        Ok(())
    }
}

fn validate_finite(value: Option<f64>, field_name: &str) -> Result<(), RunwayError> {
    if value.is_some_and(|value| !value.is_finite()) {
        validation(format!("{field_name} must be a finite number"))
    } else {
        Ok(())
    }
}

fn validation<T>(message: impl Into<String>) -> Result<T, RunwayError> {
    Err(RunwayError::Validation {
        message: message.into(),
    })
}
