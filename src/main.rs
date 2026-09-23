use iced::widget::{Button, text, Text, Grid, Column, Container, Row, column, container, row};
use iced::{Alignment, Fill, Element, Theme, Renderer, Length, FillPortion, Settings, Size, window};



//struct that sets the type of data that the object can take. structs are basically arrays/tuples that can be used as immutable datatypes
#[derive(Default)]
struct TextStack{
    value: String,
}




#[derive(Default)]
struct Calculator{
    value: String,
}

//enum type since the message can have multiple, but predetermined types
#[derive(Debug, Clone)]
enum Message {
    Evaluate,
    Clear,
}


impl Calculator{

    fn new() -> Self{
        Self{
            value: "".to_string(),
        }
    }

    fn update (&mut self, message: Message){
        match message{
            Message::Evaluate=>{
                todo!();
            },
            Message::Clear=>{
                self.value="".to_string();
            },
        }
        
    }

    //manually building the GUI, gonna be messy, sorry
    fn view (&self) -> Element<'_, Message> {


        Container::new(Column::new())

        //.width(Length::Fill)
        .padding(10)
        .into()

    }
}


pub fn main() -> iced::Result{
   
    iced::application(Calculator::new, Calculator::update, Calculator::view)
        .window_size(iced::Size::new(400.0, 500.0))
        .run()
}