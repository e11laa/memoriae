use nih_plug::prelude::*;
use std::sync::Arc;

/// A gentle tone-shaper that blends a smoothed tilt filter with a clean dry path.
/// The design keeps CPU usage low and avoids external dependencies so it is easy
/// to build on Windows without linker errors.
pub struct QuietTexture {
    params: Arc<QuietTextureParams>,
    tilt_state: f32,
}

#[derive(Params)]
pub struct QuietTextureParams {
    /// Overall output level in decibels.
    #[id = "gain"]
    pub gain: FloatParam,

    /// Tilt filter intensity. Positive values add shimmer while negative values
    /// gently darken the sound.
    #[id = "tilt"]
    pub tilt_amount: FloatParam,

    /// Dry/wet crossfade between the original signal and the processed one.
    #[id = "mix"]
    pub mix: FloatParam,
}

impl Default for QuietTexture {
    fn default() -> Self {
        Self {
            params: Arc::new(QuietTextureParams::default()),
            tilt_state: 0.0,
        }
    }
}

impl Default for QuietTextureParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Output",
                0.0,
                FloatRange::Linear {
                    min: -24.0,
                    max: 24.0,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(30.0))
            .with_unit(" dB"),
            tilt_amount: FloatParam::new(
                "Tilt",
                0.0,
                FloatRange::Linear {
                    min: -1.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(20.0)),
            mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_unit(" %"),
        }
    }
}

impl Plugin for QuietTexture {
    const NAME: &'static str = "Quiet Texture";
    const VENDOR: &'static str = "Memoriae Labs";
    const URL: &'static str = "https://example.com/memoriae";
    const EMAIL: &'static str = "support@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const DEFAULT_AUDIO_IO: AudioIOLayout = AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    };

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    const STABILIZER: Stabilizer = Stabilizer::new(1.0, 0.01);

    type BackgroundTask = ();
    type SysExMessage = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _context: &mut impl InitContext<Self>,
    ) -> Result<(), nih_plug::prelude::PluginError> {
        self.tilt_state = 0.0;
        Ok(())
    }

    fn reset(&mut self) {
        self.tilt_state = 0.0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let gain = self.params.gain.smoothed.next();
        let mix = self.params.mix.smoothed.next();

        // Convert decibels to linear gain once per block.
        let linear_gain = util::db_to_gain_fast(gain);

        for mut channel_samples in buffer.iter_samples() {
            for (channel, sample) in channel_samples.iter_mut().enumerate() {
                // One-pole tilt that biases high frequencies on channel 1 and
                // low frequencies on channel 0 to keep the stereo image stable.
                let tilt_target = self.params.tilt_amount.smoothed.next();
                self.tilt_state = self.tilt_state * 0.965 + tilt_target * 0.035;

                let shaped = if channel % 2 == 0 {
                    *sample - self.tilt_state * *sample * 0.35
                } else {
                    *sample + self.tilt_state * *sample * 0.35
                };

                let processed = shaped * linear_gain;
                *sample = processed * mix + *sample * (1.0 - mix);
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for QuietTexture {
    const CLAP_ID: &'static str = "com.memoriae.quiet_texture";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A gentle tilt and gain tool for quick tone shaping.");
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Effect,
        ClapFeature::Stereo,
        ClapFeature::Mastering,
    ];
}

impl Vst3Plugin for QuietTexture {
    const VST3_CLASS_ID: [u8; 16] = *b"QuietTextureMem";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Mastering];
}

nih_export_clap!(QuietTexture);
nih_export_vst3!(QuietTexture);
