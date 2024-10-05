mod util;
mod image;

use std::time::Duration;
use chrono::{DateTime, Local};
use crate::util::load_image;
use iced::mouse::Cursor;
use iced::widget::canvas::{Geometry, Image};
use iced::widget::{canvas, Canvas};
use iced::{time, Color, Element, Fill, Point, Rectangle, Size, Subscription, Theme};
use crate::image::DynamicImage;

const WINDOW_TITLE: &str = "Render";
const WINDOW_SIZE: Size = Size::new(600f32, 600f32);
const TARGET_FRAME_RATE: u64 = 60;
const TARGET_FRAME_TIME_MILLIS: u64 = 1_000u64 / TARGET_FRAME_RATE;

fn main() {
    // tracing_subscriber::fmt::init();

    match iced::application(WINDOW_TITLE, Application::update, Application::view)
        .subscription(Application::subscription)
        .window_size(WINDOW_SIZE)
        .centered()
        .antialiasing(true)
        .run() { // TODO: template
            Ok(_) => {},
            Err(e) => println!("Couldn't create application wind")
        };
}

struct Application {
    frame_buffer: DynamicImage,
    last_update: i64
} // the iced state
impl Application {
    fn new() -> Self {
        Self {
            frame_buffer: DynamicImage::new(WINDOW_SIZE.width as u32, WINDOW_SIZE.height as u32), //TODO: template
            last_update: Local::now().timestamp_millis()
        }
    }

    fn subscription(&self) -> Subscription<Message> { // produces a message without time every frame
        time::every(Duration::from_millis(TARGET_FRAME_TIME_MILLIS))
            .map(|_| Message::Tick(Local::now()))
    }

    fn update(&mut self, message: Message) { // message is action from view (=render)
        match message {
            Message::Tick(time) => {
                println!("update at {}", time.timestamp_millis());

                let delta = time.timestamp_millis() - self.last_update;
                self.last_update += delta;

            }
        };
    }

    fn view(&self) -> Element<Message> { // returns message based on button press, ...
        Canvas::new(CanvasRenderer::new()).width(Fill).height(Fill).into()
    }
}

impl Default for Application { // state auto created by iced
    fn default() -> Self { Application::new() }
}

struct CanvasRenderer {
    image: Image
}
impl canvas::Program<Message> for CanvasRenderer {
    type State = Application;

    fn draw(&self, state: &Self::State, renderer: &iced::Renderer, theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.draw_image(Rectangle::new(Point::new(100f32, 100f32), Size::new(100f32, 100f32)), self.image.clone());
        frame.fill_rectangle(Point::new(150f32, 250f32), Size::new(100f32, 100f32), Color::new(0.0f32, 0.5f32, 0.0f32, 1.0f32));

        println!("repaint");

        vec![frame.into_geometry()]
    }
}
impl CanvasRenderer {
    fn new() -> Self {
        CanvasRenderer {
            image: load_image("logo.png")
        }
    }
}

#[derive(Debug,Clone)]
enum Message {
    Tick(DateTime<Local>)
}
