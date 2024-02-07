use ggez::{
    audio::{SoundSource, Source},
    Context,
};

pub struct SoundHandler {
    pub current_audio: Option<Source>,
    pub sounds: Vec<String>,
}

impl Default for SoundHandler {
    fn default() -> Self {
        SoundHandler {
            sounds: vec![
                "/sound/BlueBoyAdventure.wav".to_string(),
                "/sound/coin.wav".to_string(),
                "/sound/powerup.wav".to_string(),
                "/sound/unlock.wav".to_string(),
                "/sound/fanfare.wav".to_string(),
                "/sound/hitmonster.wav".to_string(),
                "/sound/receivedamage.wav".to_string(),
            ],
            current_audio: None,
        }
    }
}

impl SoundHandler {
    fn set_file(&mut self, ctx: &mut Context, index: i32) {
        self.current_audio =
            Some(Source::new(ctx, self.sounds.get(index as usize).unwrap()).unwrap());
    }

    fn play(&mut self, ctx: &mut Context, index: i32) {
        let mut audio = Source::new(ctx, self.sounds.get(index as usize).unwrap()).unwrap();
        let _ = audio.play_detached(ctx);
    }

    fn loop_audio(&mut self, ctx: &mut Context) {
        if let Some(audio) = &mut self.current_audio {
            audio.set_repeat(true);
            let _ = audio.play(ctx);
        }
    }

    fn stop(&mut self, ctx: &mut Context) {
        if let Some(audio) = &mut self.current_audio {
            let _ = audio.stop(ctx);
        }
    }

    pub fn play_music(&mut self, ctx: &mut Context, index: i32) {
        self.set_file(ctx, index);
        self.loop_audio(ctx);
    }

    pub fn stop_music(&mut self, ctx: &mut Context) {
        self.stop(ctx);
    }

    pub fn play_sound_effect(&mut self, ctx: &mut Context, index: i32) {
        self.play(ctx, index);
    }
}
