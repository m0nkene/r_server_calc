use iced::widget::{Button, text, Text, Grid, Column, Container, Row, column, container, row, image, Image, text_input, checkbox, slider, button};
use iced::{Alignment, Fill, Element, Theme, Renderer, Length, FillPortion, Settings, Size, window, border, Border, color, Color};



pub fn main() -> iced::Result{
   
    iced::application(Calculator::new, Calculator::update, Calculator::view)
        //.set_min_size(iced::Size::new(400.0, 600.0))
        //.window_size(iced::Size::new(1200.0, 600.0))
        .window(window::Settings {
            size: Size::new(1200.0, 600.0), // Initial size
            min_size: Some(Size::new(700.0, 450.0)), // Minimum size limit
            ..Default::default()
        })
        .run()
}



struct Calculator{
    xs_img_handle: image::Handle,
    xs_targ_iops: String,
    xs_cnt_2u: String,
    xs_cnt_4u: String,
    xs_rack_cnt: String,
    xs_4u_cb: bool,

    risc_img_handle: image::Handle,
    risc_targ_iops: String,
    risc_cnt_2u: String,
    risc_cnt_4u: String,
    risc_rack_cnt: String,
    risc_4u_cb: bool,

    mf_img_handle: image::Handle,
    mf_targ_iops: String,
    mf_cnt_2u: String,
    mf_cnt_4u: String,
    mf_rack_cnt: String,
    mf_4u_cb: bool,

    gpu_img_handle: image::Handle,
    gpu_targ_iops: String,
    gpu_cnt_2u: String,
    gpu_cnt_4u: String,
    gpu_rack_cnt: String,
    gpu_4u_cb: bool,

    red_slider: f32,
    tot_racks: i32,
}




impl Default for Calculator{
    fn default() -> Self{
        Self{

            //setting defaults for XServer
            xs_img_handle : image::Handle::from_bytes(include_bytes!("../resources/xs_icon.png").to_vec()),
            xs_targ_iops : "".to_string(),
            xs_cnt_2u: "0".to_string(),
            xs_cnt_4u: "0".to_string(),
            xs_rack_cnt: "0".to_string(),
            xs_4u_cb: false,

            risc_img_handle : image::Handle::from_bytes(include_bytes!("../resources/risc_icon.png").to_vec()),
            risc_targ_iops : "".to_string(),
            risc_cnt_2u: "0".to_string(),
            risc_cnt_4u: "0".to_string(),
            risc_rack_cnt: "0".to_string(),
            risc_4u_cb: false,

            mf_img_handle : image::Handle::from_bytes(include_bytes!("../resources/mf_icon.png").to_vec()),
            mf_targ_iops : "".to_string(),
            mf_cnt_2u: "0".to_string(),
            mf_cnt_4u: "0".to_string(),
            mf_rack_cnt: "0".to_string(),
            mf_4u_cb: false,

            gpu_img_handle : image::Handle::from_bytes(include_bytes!("../resources/gpu_icon.png").to_vec()),
            gpu_targ_iops : "".to_string(),
            gpu_cnt_2u: "0".to_string(),
            gpu_cnt_4u: "0".to_string(),
            gpu_rack_cnt: "0".to_string(),
            gpu_4u_cb: false,

            red_slider: 100.0,
            tot_racks: 0,

        }
        
    }
}


//enum type since the message can have multiple, but predetermined types
#[derive(Debug, Clone)]
enum Message {
    XSContentChanged(String),
    XSCBToggled(bool),

    RISCContentChanged(String),
    RISCCBToggled(bool),

    MFContentChanged(String),
    MFCBToggled(bool),

    GPUContentChanged(String),
    GPUCBToggled(bool),

    RSliderChanged(f32),
    ClearButton,
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
            },


            Message::RISCContentChanged(input) => {
                self.risc_targ_iops = input;
                Self::evaluate(self, 1);
            },
            Message::RISCCBToggled(new_status) => {
                self.risc_4u_cb = new_status;
                Self::evaluate(self, 1);
            },


            Message::MFContentChanged(input) => {
                self.mf_targ_iops = input;
                Self::evaluate(self, 2);
            },
            Message::MFCBToggled(new_status) => {
                self.mf_4u_cb = new_status;
                Self::evaluate(self, 2);
            },


            Message::GPUContentChanged(input) => {
                self.gpu_targ_iops = input;
                Self::evaluate(self, 3);
            },
            Message::GPUCBToggled(new_status) => {
                self.gpu_4u_cb = new_status;
                Self::evaluate(self, 3);
            },


            Message::RSliderChanged(val) => {
                self.red_slider = val;
                Self::evaluate(self,0);
                Self::evaluate(self,1);
                Self::evaluate(self,2);
                Self::evaluate(self,3);
            },

            Message::ClearButton => {
                *self = Self::default();
            }
        }
        
    }




    //manually building the GUI, gonna be messy, sorry
    fn view (&self) -> Element<'_, Message> {


        let xs_text_box = text_input("Target IOPS here", &self.xs_targ_iops).on_input(Message::XSContentChanged);
        let xs_4u_check_box = checkbox(self.xs_4u_cb).on_toggle(Message::XSCBToggled);
        let xs_2u_out = text(self.xs_cnt_2u.clone());
        let xs_4u_out = text(self.xs_cnt_4u.clone());
        let xs_rack_out = text(self.xs_rack_cnt.clone());


        let risc_text_box = text_input("Target IOPS here", &self.risc_targ_iops).on_input(Message::RISCContentChanged);
        let risc_4u_check_box = checkbox(self.risc_4u_cb).on_toggle(Message::RISCCBToggled);
        let risc_2u_out = text(self.risc_cnt_2u.clone());
        let risc_4u_out = text(self.risc_cnt_4u.clone());
        let risc_rack_out = text(self.risc_rack_cnt.clone());


        let mf_text_box = text_input("Target IOPS here", &self.mf_targ_iops).on_input(Message::MFContentChanged);
        let mf_4u_check_box = checkbox(self.mf_4u_cb).on_toggle(Message::MFCBToggled);
        let mf_2u_out = text(self.mf_cnt_2u.clone());
        let mf_4u_out = text(self.mf_cnt_4u.clone());
        let mf_rack_out = text(self.mf_rack_cnt.clone());


        let gpu_text_box = text_input("Target IOPS here", &self.gpu_targ_iops).on_input(Message::GPUContentChanged);
        let gpu_4u_check_box = checkbox(self.gpu_4u_cb).on_toggle(Message::GPUCBToggled);
        let gpu_2u_out = text(self.gpu_cnt_2u.clone());
        let gpu_4u_out = text(self.gpu_cnt_4u.clone());
        let gpu_rack_out = text(self.gpu_rack_cnt.clone());


        let redundancy_slider = slider(100.0..=200.0, self.red_slider, Message::RSliderChanged);
        let red_text_box = text(self.red_slider.clone());
        let tot_rack_cnt = text(self.tot_racks.clone());
        let reset_button = button("Clear").on_press(Message::ClearButton);

        
        
       //building input column - col1
        let col1 = Column::new()
            .width(Length::FillPortion(1))
            .push(
                Container::new(
                        //XServer row
                        Row::new()
                            .push(
                                Image::new(self.xs_img_handle.clone())
                                    .width(Length::FillPortion(1))
                                    .height(Length::FillPortion(1))

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
                            .align_y(Alignment::Center)
                        

                )
                .style( |_theme| container::Style{
                    border: Border {
                        color: color!(0x000000),         // Hex code or RGB Color
                        width: 2.0.into(),               // Border thickness in pixels
                        radius: border::radius(5.0),     // Rounded corners (all sides)
                    },
                    ..Default::default()
                })
                .height(Length::FillPortion(3))
                .padding(10)  
                            
            )
            .push(
                Container::new(
                    //RISC row
                    Row::new()
                        .push(
                            Image::new(self.risc_img_handle.clone())
                                .width(Length::FillPortion(1))
                                .height(Length::FillPortion(1))
                        )
                        .push(
                            Column::new()
                                .push(
                                    risc_text_box
                                        .width(Length::FillPortion(3))
                                )
                                .push(
                                    risc_4u_check_box
                                        .label("4U Available?")
                                )
                                    
                                .padding(20)

                        )
                        
                        .align_y(Alignment::Center)

                )
                .style( |_theme| container::Style{
                    border: Border {
                        color: color!(0x000000),         // Hex code or RGB Color
                        width: 2.0.into(),               // Border thickness in pixels
                        radius: border::radius(5.0),     // Rounded corners (all sides)
                    },
                    ..Default::default()
                })
                .height(Length::FillPortion(3))
                .padding(10)  
                            
            )
            .push(
                Container::new(
                    //MF row
                    Row::new()
                        .push(
                            Image::new(self.mf_img_handle.clone())
                                .width(Length::FillPortion(1))
                                .height(Length::FillPortion(1))
                        )
                        .push(
                            Column::new()
                                .push(
                                    mf_text_box
                                        .width(Length::FillPortion(3))
                                )
                                .push(
                                    mf_4u_check_box
                                        .label("4U Available?")
                                )
                                    
                                .padding(20)

                        )
                        
                        .align_y(Alignment::Center)

                )
                .style( |_theme| container::Style{
                    border: Border {
                        color: color!(0x000000),         // Hex code or RGB Color
                        width: 2.0.into(),               // Border thickness in pixels
                        radius: border::radius(5.0),     // Rounded corners (all sides)
                    },
                    ..Default::default()
                })
                .height(Length::FillPortion(3))
                .padding(10)  
                            
            )
            .push(
                Container::new(
                    //GPU row
                    Row::new()
                        .push(
                            Image::new(self.gpu_img_handle.clone())
                                .width(Length::FillPortion(1))
                                .height(Length::FillPortion(1))
                        )
                        .push(
                            Column::new()
                                .push(
                                    gpu_text_box
                                        .width(Length::FillPortion(3))
                                )
                                .push(
                                    gpu_4u_check_box
                                        .label("4U Available?")
                                )
                                    
                                .padding(20)

                        )
                        
                        .align_y(Alignment::Center)

                )
                .style( |_theme| container::Style{
                    border: Border {
                        color: color!(0x000000),         // Hex code or RGB Color
                        width: 2.0.into(),               // Border thickness in pixels
                        radius: border::radius(5.0),     // Rounded corners (all sides)
                    },
                    ..Default::default()
                })
                .height(Length::FillPortion(3))
                .padding(10)  
                            
            )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Container::new(
                                text("Red. %: ")
                            )
                            .width(Length::FillPortion(2))
                        )
                        .push(
                            Container::new(
                                redundancy_slider
                            )
                            .width(Length::FillPortion(6))
                            //.padding(20)
                        )
                        .push(
                            Container::new(
                                red_text_box
                            )
                            .width(Length::FillPortion(2))
                            .align_x(Alignment::Start)
                        )
                )
                .height(Length::FillPortion(1))
                .align_y(Alignment::Center)
            )
            .padding(10);
            
        
        
        
            //Building the output column - col2
        let col2 = Column::new()
            .width(Length::FillPortion(1))
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Image::new(self.xs_img_handle.clone())
                                .width(Length::FillPortion(1))
                                .height(Length::FillPortion(1))
                        )
                        .push(
                            Column::new()
                                .push(
                                    Row::new()
                                        .push("4U Servers: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            xs_4u_out
                                                .width(Length::FillPortion(2))
                                        )
                                        .push("2U Servers: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            xs_2u_out
                                                .width(Length::FillPortion(2))
                                        )
                                )

                                .push(
                                    Row::new()
                                        .push("Min. Racks Needed: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            xs_rack_out
                                                .width(Length::FillPortion(1))
                                        )
                                )
                            .padding(iced::padding::left(20))
                            .width(Length::FillPortion(3))
                        )
                        .align_y(Alignment::Center)
                
                    )
                    .style( |_theme| container::Style{
                        border: Border {
                            color: color!(0x000000),         // Hex code or RGB Color
                            width: 2.0.into(),               // Border thickness in pixels
                            radius: border::radius(5.0),     // Rounded corners (all sides)
                        },
                        ..Default::default()
                    })
                    .height(Length::FillPortion(3))
                    .padding(10)
                )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Image::new(self.risc_img_handle.clone())
                                .width(Length::FillPortion(1))
                                .height(Length::FillPortion(1))
                        )
                        .push(
                            Column::new()
                                .push(
                                    Row::new()
                                        .push("4U Servers: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            risc_4u_out
                                                .width(Length::FillPortion(2))
                                        )
                                        .push("2U Servers: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            risc_2u_out
                                                .width(Length::FillPortion(2))
                                        )
                                )

                                .push(
                                    Row::new()
                                        .push("Min. Racks Needed: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            risc_rack_out
                                                .width(Length::FillPortion(1))
                                        )
                                )
                            .padding(iced::padding::left(20))
                            .width(Length::FillPortion(3))
                        )
                        .align_y(Alignment::Center)
                
                    )
                    .style( |_theme| container::Style{
                        border: Border {
                            color: color!(0x000000),         // Hex code or RGB Color
                            width: 2.0.into(),               // Border thickness in pixels
                            radius: border::radius(5.0),     // Rounded corners (all sides)
                        },
                        ..Default::default()
                    })
                    .height(Length::FillPortion(3))
                    .padding(10)
                )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Image::new(self.mf_img_handle.clone())
                                .width(Length::FillPortion(1))
                                .height(Length::FillPortion(1))
                        )
                        .push(
                            Column::new()
                                .push(
                                    Row::new()
                                        .push("4U Servers: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            mf_4u_out
                                                .width(Length::FillPortion(2))
                                        )
                                        .push("2U Servers: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            mf_2u_out
                                                .width(Length::FillPortion(2))
                                        )
                                )

                                .push(
                                    Row::new()
                                        .push("Min. Racks Needed: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            mf_rack_out
                                                .width(Length::FillPortion(1))
                                        )
                                )
                            .padding(iced::padding::left(20))
                            .width(Length::FillPortion(3))
                        )
                        .align_y(Alignment::Center)
                
                    )
                    .style( |_theme| container::Style{
                        border: Border {
                            color: color!(0x000000),         // Hex code or RGB Color
                            width: 2.0.into(),               // Border thickness in pixels
                            radius: border::radius(5.0),     // Rounded corners (all sides)
                        },
                        ..Default::default()
                    })
                    .height(Length::FillPortion(3))
                    .padding(10)
                )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Image::new(self.gpu_img_handle.clone())
                                .width(Length::FillPortion(1))
                                .height(Length::FillPortion(1))
                        )
                        .push(
                            Column::new()
                                .push(
                                    Row::new()
                                        .push("4U Servers: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            gpu_4u_out
                                                .width(Length::FillPortion(2))
                                        )
                                        .push("2U Servers: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            gpu_2u_out
                                                .width(Length::FillPortion(2))
                                        )
                                )

                                .push(
                                    Row::new()
                                        .push("Min. Racks Needed: ")
                                            .width(Length::FillPortion(1))
                                        .push(
                                            gpu_rack_out
                                                .width(Length::FillPortion(1))
                                        )
                                )
                            .padding(iced::padding::left(20))
                            .width(Length::FillPortion(3))
                        )
                        .align_y(Alignment::Center)
                
                    )
                    .style( |_theme| container::Style{
                        border: Border {
                            color: color!(0x000000),         // Hex code or RGB Color
                            width: 2.0.into(),               // Border thickness in pixels
                            radius: border::radius(5.0),     // Rounded corners (all sides)
                        },
                        ..Default::default()
                    })
                    .height(Length::FillPortion(3))
                    .padding(10)
                )
            .push(
                Container::new(
                    Row::new()
                        .push(
                            Container::new(
                                text("Total Min. Racks: ")
                            )
                            .width(Length::FillPortion(3))
                        )
                        .push(
                            Container::new(
                                tot_rack_cnt
                            )
                            .width(Length::FillPortion(5))
                        )
                        .push(
                            Container::new(
                                reset_button
                            )
                            .width(Length::FillPortion(2))
                            //.padding(20)
                        )
                )
                .height(Length::FillPortion(1))
                .align_y(Alignment::Center)
            )
            .padding(10);






        
        
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
            1 => {
                //checking if input is valid from the user; if not, it will throw an error, if it is valid, it will set the total variable to the input.
                let test = self.risc_targ_iops.parse::<i32>();
                match test {
                    Err(e) => {self.risc_cnt_2u = "0".to_string(); println!("Error encountered: {}", e)},
                    Ok(ok) => {total = total + ok; println!("Valid input")}, 
                }
                cb_state = self.risc_4u_cb
            },
            2 => {
                //checking if input is valid from the user; if not, it will throw an error, if it is valid, it will set the total variable to the input.
                let test = self.mf_targ_iops.parse::<i32>();
                match test {
                    Err(e) => {self.mf_cnt_2u = "0".to_string(); println!("Error encountered: {}", e)},
                    Ok(ok) => {total = total + ok; println!("Valid input")}, 
                }
                cb_state = self.mf_4u_cb
            },
            3 => {
                //checking if input is valid from the user; if not, it will throw an error, if it is valid, it will set the total variable to the input.
                let test = self.gpu_targ_iops.parse::<i32>();
                match test {
                    Err(e) => {self.gpu_cnt_2u = "0".to_string(); println!("Error encountered: {}", e)},
                    Ok(ok) => {total = total + ok; println!("Valid input")}, 
                }
                cb_state = self.gpu_4u_cb
            },
            _ => {println!("Oopsie poopsies");},
        }



        //checking if the 4u available box is ticked, performing math accordingly. Math is messy, but no casting is needed to floats for calculating
        if cb_state {
            total = (total as f32 * (self.red_slider as f32 / 100.0)) as i32;
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
            total = (total as f32 * (self.red_slider as f32 / 100.0)) as i32;
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
                self.tot_racks = self.xs_rack_cnt.parse::<i32>().unwrap() + self.risc_rack_cnt.parse::<i32>().unwrap() + self.mf_rack_cnt.parse::<i32>().unwrap() + self.gpu_rack_cnt.parse::<i32>().unwrap();
            },
            1 => {
                self.risc_cnt_4u = target_4u.to_string();
                self.risc_cnt_2u = target_2u.to_string();
                self.risc_rack_cnt = racks.to_string();
                self.tot_racks = self.xs_rack_cnt.parse::<i32>().unwrap() + self.risc_rack_cnt.parse::<i32>().unwrap() + self.mf_rack_cnt.parse::<i32>().unwrap() + self.gpu_rack_cnt.parse::<i32>().unwrap();
            },
            2 => {
                self.mf_cnt_4u = target_4u.to_string();
                self.mf_cnt_2u = target_2u.to_string();
                self.mf_rack_cnt = racks.to_string();
                self.tot_racks = self.xs_rack_cnt.parse::<i32>().unwrap() + self.risc_rack_cnt.parse::<i32>().unwrap() + self.mf_rack_cnt.parse::<i32>().unwrap() + self.gpu_rack_cnt.parse::<i32>().unwrap();
            },
            3 => {
                self.gpu_cnt_4u = target_4u.to_string();
                self.gpu_cnt_2u = target_2u.to_string();
                self.gpu_rack_cnt = racks.to_string();
                self.tot_racks = self.xs_rack_cnt.parse::<i32>().unwrap() + self.risc_rack_cnt.parse::<i32>().unwrap() + self.mf_rack_cnt.parse::<i32>().unwrap() + self.gpu_rack_cnt.parse::<i32>().unwrap();
            },
            _ => {println!("Oopsie poopsies2");},
        }





    }



}