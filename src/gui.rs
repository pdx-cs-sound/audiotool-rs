use iced::executor;
use iced::font::{self, Font, Weight};
use iced::widget::{row, column, container, vertical_slider, text};
use iced::window;
use iced::{Application, Command, Element, Length, Settings, Theme};

use crate::*;

const MUTE: i16 = -61;

#[derive(Debug, Clone)]
enum AudioMessage {
    SetFrequency(i16),
    SetAmplitude(i16),
    FontLoaded(Result<(), font::Error>),
}

struct AudioSettings(Arc<Mutex<AudioParams>>);

impl Application for AudioSettings {
    type Message = AudioMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Arc<Mutex<AudioParams>>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let flags = Self(flags);
        let command = font::load(
            include_bytes!("../assets/fonts/NotoSansMono-Bold.ttf").as_slice()
        ).map(<Self::Message>::FontLoaded);
        (flags, command)
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
            AudioMessage::FontLoaded(result) => {
                result.unwrap();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let audio_params = self.0.lock().unwrap();
        let a0 = audio_params.amplitude;
        let f = audio_params.frequency;
        let (f_lo, f_hi) = audio_params.freq_slider_range();
        drop(audio_params);
        let a = match a0 {
            None => MUTE,
            Some(a) => a,
        };

        let a_slider = vertical_slider(MUTE..=0, a, AudioMessage::SetAmplitude).step(1);
        let a_dbfs = match a0 {
            None => "MUTE".to_string(),
            Some(d) => format!("{d} dBFS"),
        };
        let a_dbfs_text = text(a_dbfs);
        let a_amplitude_text = text(format!("{:0.3}", db_to_amplitude(a0)));
        let a_column = column![
            container(a_slider)
                .width(Length::Fill)
                .height(340)
                .center_x(),
            container(a_dbfs_text)
                .width(Length::Fill)
                .height(40)
                .center_x(),
            container(a_amplitude_text)
                .width(Length::Fill)
                .height(40)
                .center_x(),
        ];

        let f_slider = vertical_slider(f_lo..=f_hi, f, AudioMessage::SetFrequency).step(1);
        let f_keynote = format!(
            "{} {}{}",
            f,
            key_note_name(f),
            key_note_octave(f),
        );
        let f_keynote_text = text(f_keynote);
        let f_freq_text = text(format!("{:0.1} Hz", key_to_freq(f)));
        let f_column = column![
            container(f_slider)
                .width(Length::Fill)
                .height(340)
                .center_x(),
            container(f_keynote_text)
                .width(Length::Fill)
                .height(40)
                .center_x(),
            container(f_freq_text)
                .width(Length::Fill)
                .height(40)
                .center_x(),
        ];

        let contents = row![
            container(a_column)
                .width(Length::Fill)
                .height(420)
                .center_x(),
            container(f_column)
                .width(Length::Fill)
                .height(420)
                .center_x(),
        ];
        container(contents)
            .width(150)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn start_gui(params: Arc<Mutex<AudioParams>>) -> anyhow::Result<()> {
    let mut default_font = Font::with_name("Noto Sans Mono");
    default_font.weight = Weight::Bold;
    let settings = Settings {
        window: window::Settings {
            size: (150, 420),
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        default_font,
        ..Settings::with_flags(params)
    };
    AudioSettings::run(settings)?;
    Ok(())
}
