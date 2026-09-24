use iced::widget::{Button, text, Text, Grid, Column, Container, Row, column, container, row, image, Image, text_input, checkbox};
use iced::{Alignment, Fill, Element, Theme, Renderer, Length, FillPortion, Settings, Size, window};



pub fn main() -> iced::Result{
   
    iced::application(Calculator::new, Calculator::update, Calculator::view)
        .run()
}



struct Calculator{
    xs_img_handle: image::Handle,
    xs_targ_iops: String,
    xs_cnt_2u: String,
    xs_cnt_4u: String,
    xs_rack_cnt: String,
    xs_4u_cb: bool,
}

impl Default for Calculator{
    fn default() -> Self{
        Self{
            xs_img_handle : image::Handle::from_bytes(include_bytes!("../resources/ibm.png").to_vec()),
            xs_targ_iops : "".to_string(),
            xs_cnt_2u: "0".to_string(),
            xs_cnt_4u: "0".to_string(),
            xs_rack_cnt: "0".to_string(),
            xs_4u_cb: false,
        }
        
    }
}


//enum type since the message can have multiple, but predetermined types
#[derive(Debug, Clone)]
enum Message {
    XSContentChanged(String),
    XSCBToggled(bool),
}


impl Calculator{

    fn new() -> Self{
        Self::default()
    }

    fn update (&mut self, message: Message){
        match message{
            Message::XSContentChanged(input) => {
                self.xs_targ_iops = input;
                Self::evaluate(self, 0);
            },
            Message::XSCBToggled(new_status) => {
                self.xs_4u_cb = new_status;
                Self::evaluate(self, 0);
            }
        }
        
    }




    //manually building the GUI, gonna be messy, sorry
    fn view (&self) -> Element<'_, Message> {


        let xs_text_box = text_input("Target IOPS here", &self.xs_targ_iops)
            .on_input(Message::XSContentChanged);
        let xs_4u_check_box = checkbox(self.xs_4u_cb).on_toggle(Message::XSCBToggled);
        let xs_2u_out = text(self.xs_cnt_2u.clone());
        let xs_4u_out = text(self.xs_cnt_4u.clone());
        let xs_rack_out = text(self.xs_rack_cnt.clone());

        
        
       //building input column - col1
        let col1 = Column::new()
            .width(Length::FillPortion(1))
            .push(
                //XServer row
                Row::new()
                    .push(
                        Image::new(self.xs_img_handle.clone())
                            .width(Length::FillPortion(1))
                    )
                    .push(
                        Column::new()
                            .push(
                                xs_text_box
                                    .width(Length::FillPortion(3))
                            )
                            .push(
                                xs_4u_check_box
                                    .label("4U Available?")
                            )
                            
                            .padding(20)

                    )
                        
                    
            );

        
        
        
            //Building the output column - col2
            let col2 = Column::new()
            .width(Length::FillPortion(1))
            .push(
                Row::new()
                    .push(
                        Image::new(self.xs_img_handle.clone())
                            .width(Length::FillPortion(1))
                    )
                    .push(
                        Column::new()
                            .push(
                                xs_4u_out
                            )
                            .push(
                                xs_2u_out
                            )
                            .push(
                                xs_rack_out
                            )
                        .width(Length::FillPortion(3))
                    )
            
            );


        
        
        //outputting final columns to the main display
            row![
            col1, 
            col2,
        ]
        
            //.width(Length::Fill)
        .padding(10)
        .into()

    }


    //calculation function that runs every time users input is changed. Currently configured for one user
    //the writing function will be intelligent and write to the server specific variables
    fn evaluate (&mut self, serv_type: i8){        
        
        let mut total = 0;
        let mut target_4u = 0;
        let mut target_2u = 0;
        let mut value = 0;
        let mut racks = 0;
        let mut cb_state = false;

        //match statement to set the server type specific variables; specifically the input, and the checkbox st
        match serv_type{
            0 => {
                //checking if input is valid from the user; if not, it will throw an error, if it is valid, it will set the total variable to the input.
                let test = self.xs_targ_iops.parse::<i32>();
                match test {
                    Err(e) => {self.xs_cnt_2u = "0".to_string(); println!("Error encountered: {}", e)},
                    Ok(ok) => {total = total + ok; println!("Valid input")}, 
                }
                cb_state = self.xs_4u_cb
            },
            _ => {println!("Oopsie poopsies");},
        }



        //checking if the 4u available box is ticked, performing math accordingly. Math is messy, but no casting is needed to floats for calculating
        if cb_state {
            target_4u = total/12000;
            value = target_4u * 12000;
            target_2u = (total-value)/5000;
            
            //checking if another 2u is needed
            if (target_4u * 12000)+(target_2u * 5000) < total{
                target_2u += 1;
            }

            //calculating racks, prioritizing 4us, and not filling with 2us
            racks = total/72000;
            if (racks*72000) < total{
                racks = racks + 1;
            }
            
            
            println!("XS_4u: {}, XS_2u: {}", target_4u, target_2u);
        } 
        
        //same math as above to calculate server counts, but forcing 4u count to 0
        else{
            target_4u = 0;
            value = target_4u * 12000;
            target_2u = (total-value)/5000;
            
            //checking if another 2u is needed
            if (target_4u * 12000)+(target_2u * 5000) < total{
                target_2u += 1;
            }

            //assume there are no 4us, simplifying calculations
            racks = total/80000;
            if (racks*80000) < total{
                racks = racks + 1;
            }

            println!("XS_4u: {}, XS_2u: {}", target_4u, target_2u);
        }



        //final setting of server specific information
        match serv_type{
            0 => {
                self.xs_cnt_4u = target_4u.to_string();
                self.xs_cnt_2u = target_2u.to_string();
                self.xs_rack_cnt = racks.to_string();
            },
            _ => {println!("Oopsie poopsies2");},
        }





    }



}


