use iced::executor;
use iced::widget::*;
use iced::window;
use iced::{Application, Command, Element, Length, Settings, Theme};

use crate::*;

const MUTE: i16 = -61;

#[derive(Debug, Clone)]
enum AudioMessage {
    SetFrequency(i16),
    SetAmplitude(i16),
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
            AudioMessage::SetAmplitude(a) => {
                audio_params.amplitude = match a {
                    MUTE => None,
                    a => Some(a),
                };
            }
            AudioMessage::SetFrequency(f) => {
                audio_params.frequency = f;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let audio_params = self.0.lock().unwrap();
        let a = match audio_params.amplitude {
            None => MUTE,
            Some(a) => a,
        };
        let f = audio_params.frequency;
        let (f_lo, f_hi) = audio_params.freq_slider_range();
        drop(audio_params);

        let a_slider = vertical_slider(MUTE..=0, a, AudioMessage::SetAmplitude).step(1);
        let f_slider = vertical_slider(f_lo..=f_hi, f, AudioMessage::SetFrequency).step(1);

        let contents = row![
            container(a_slider)
                .width(Length::Fill)
                .height(400)
                .center_x(),
            container(f_slider)
                .width(Length::Fill)
                .height(400)
                .center_x(),
        ];
        container(contents)
            .width(150)
            .height(400)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn start_gui(params: Arc<Mutex<AudioParams>>) -> anyhow::Result<()> {
    let settings = Settings {
        window: window::Settings {
            size: (150, 400),
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        ..Settings::with_flags(params)
    };
    AudioSettings::run(settings)?;
    Ok(())
}
