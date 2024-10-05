use iced::widget::canvas::Image;
use iced::widget::image::Handle;
use bytes::Bytes;

pub struct DynamicImage { // stores an rgba image as a vec
    width: u32,
    height: u32,
    data: Vec<u8>
}

impl DynamicImage {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0u8; (4 * width * height) as usize]
        }
    }
    pub fn get(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        let offset = self.get_offset(x, y);
        (self.data[offset+0], self.data[offset+1], self.data[offset+2], self.data[offset+3])
    }

    fn set(&mut self, x: u32, y: u32, color: (u8, u8, u8, u8)) {
        let offset = self.get_offset(x, y);
        self.data[offset+0] = color.0;
        self.data[offset+1] = color.1;
        self.data[offset+2] = color.2;
        self.data[offset+3] = color.3;
    }

    fn get_offset(&self, x: u32, y: u32) -> usize {
        (((y * self.width) + x) * 4) as usize
    }

    pub fn to_iced_image(&self) -> Image {
        Image::new(Handle::from_rgba(self.width, self.height, Bytes::from(self.data.clone())))
    }
}