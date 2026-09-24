use iced::widget::{Button, text, Text, Grid, Column, Container, Row, column, container, row, image, Image, text_input};
use iced::{Alignment, Fill, Element, Theme, Renderer, Length, FillPortion, Settings, Size, window};



pub fn main() -> iced::Result{
   
    iced::application(Calculator::new, Calculator::update, Calculator::view)
        .run()
}



struct Calculator{
    xs_img_handle : image::Handle,

    xs_value: String,
}

impl Default for Calculator{
    fn default() -> Self{
        Self{
            xs_img_handle : image::Handle::from_bytes(include_bytes!("../resources/ibm.png").to_vec()),
            xs_value : "Target IOPS here".to_string(),
        }
        
    }
}


//enum type since the message can have multiple, but predetermined types
#[derive(Debug, Clone)]
enum Message {
    Evaluate,
    Clear,
}


impl Calculator{

    fn new() -> Self{
        Self::default()
    }

    fn update (&mut self, message: Message){
        match message{
            Message::Evaluate=>{
                todo!();
            },
            Message::Clear=>{
                self.xs_value="".to_string();
            },
        }
        
    }

    //manually building the GUI, gonna be messy, sorry
    fn view (&self) -> Element<'_, Message> {


        row![
            Image::new(self.xs_img_handle.clone())
        ]

        //.width(Length::Fill)
        .padding(10)
        .into()

    }
}


