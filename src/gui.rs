use iced::executor;
use iced::widget::*;
use iced::{Application, Command, Element, Length, Settings, Theme};

use crate::*;

#[derive(Debug, Clone)]
enum AudioMessage {
    SetFrequency(f32),
    SetAmplitude(f32),
}

struct AudioSettings(Arc<Mutex<AudioParams>>);

impl Application for AudioSettings {
    type Message = AudioMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Arc<Mutex<AudioParams>>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self(flags), Command::none())
    }

    fn title(&self) -> String {
        "Audiotool".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let mut audio_params = self.0.lock().unwrap();
        match message {
            AudioMessage::SetAmplitude(a) => audio_params.amplitude = a,
            AudioMessage::SetFrequency(f) => audio_params.frequency = f,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let contents = row![
            vertical_slider(0.0f32..=24_000.0, 1000.0, AudioMessage::SetAmplitude),
            vertical_slider(0.0f32..=24_000.0, 1000.0, AudioMessage::SetFrequency),
        ];
        container(contents)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn start_gui(params: Arc<Mutex<AudioParams>>) -> anyhow::Result<()> {
    AudioSettings::run(Settings::with_flags(params))?;
    Ok(())
}
