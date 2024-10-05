use iced::application::Update;
use iced::mouse::Cursor;
use iced::widget::canvas::Geometry;
use iced::widget::{canvas, Canvas};
use iced::{Color, Element, Fill, Point, Rectangle, Size, Theme};

const WINDOW_TITLE: &str = "Render";
const WINDOW_SIZE: Size = Size::new(600f32, 600f32);

fn main() {
    match iced::application(
        WINDOW_TITLE,
        Application::update,
        Application::view
    ).window_size(WINDOW_SIZE).centered().run() {
        Ok(_) => {},
        Err(e) => println!("Couldn't create application wind")
    };
}

struct Application {  } // the iced state
impl Application {
    fn new() -> Self {
        let mut res = Self { };
        res.init(); // init in new?
        res
    }

    fn init(&mut self) {
        println!("Initializing renderer");
    }

    fn update(&mut self, message: Message) { // message is action from view (=render)

    }

    fn view(&self) -> Element<Message> { // returns message based on button press, ...
        Canvas::new(CanvasRenderer{}).width(Fill).height(Fill).into()
    }
}

impl Default for Application { // state auto created by iced
    fn default() -> Self { Application::new() }
}

struct CanvasRenderer {}
impl canvas::Program<Message> for CanvasRenderer {
    type State = Application;

    fn draw(&self, state: &Self::State, renderer: &iced::Renderer, theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.fill_rectangle(Point::new(100f32, 100f32), Size::new(100f32, 100f32), Color::new(0.0f32, 0.5f32, 0.0f32, 1.0f32));

        vec![frame.into_geometry()]
    }
}

#[derive(Debug,Clone)]
enum Message {
    None
}
