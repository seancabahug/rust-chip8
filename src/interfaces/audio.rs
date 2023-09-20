use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus},
    Sdl,
};

use crate::config::{BEEP_FREQUENCY_HZ, BEEP_VOLUME};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Audio {
    device: AudioDevice<SquareWave>,
}

impl Audio {
    pub fn init(sdl_context: &Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(BEEP_FREQUENCY_HZ),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: BEEP_VOLUME,
            })
            .unwrap();

        Audio { device }
    }

    pub fn play_beep(&self) {
        self.device.resume();
    }

    pub fn pause_beep(&self) {
        self.device.pause();
    }

    pub fn is_playing(&self) -> bool {
        self.device.status() == AudioStatus::Playing
    }
}
