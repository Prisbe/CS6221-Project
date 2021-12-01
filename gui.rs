use eframe::{egui, epi};
use crate::packetsniffer::get_n_packets;
use crate::port_scanner::port_scan;



pub struct OurApp {
    
    label: String,

   
    value: f32,


    // btn_stop_enabled: bool, //TODO

    // stop : bool, //TODO

    btn_clear_enabled: bool,

    packet_sniffer_panel : bool,

    port_scanner_panel : bool,

    port_to_search : String,

    scan_one_port : bool,

    scan_one_port_start : bool,

    scan_all_ports : bool,

    open_ports : String,

    open_single_port : String
}

impl Default for OurApp {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            value: 0.0,
            //btn_stop_enabled: false, //TODO
            //stop : false, // TODO
            btn_clear_enabled: false, 
            packet_sniffer_panel : false,
            port_scanner_panel : false,
            port_to_search : "0".to_string(),
            scan_one_port : false,
            scan_one_port_start : false,
            scan_all_ports : false,
            open_ports : "".to_owned(),
            open_single_port : "".to_owned()
        }
    }
}


impl epi::App for OurApp {
    fn name(&self) -> &str {
        "pAcKeT sNiFfEr & pOrT sCaNnEr"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        
    }
    

    // Called each time the UI needs repainting
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { label, value, btn_clear_enabled,
             packet_sniffer_panel, port_scanner_panel, port_to_search, scan_one_port,
             scan_one_port_start, scan_all_ports, open_ports, open_single_port} = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "Menu", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            
            ui.add_space(5.5); // specific so that heading is even with central panel heading
            ui.heading("Choose Tool");

            let sep = egui::Separator::default();
            ui.add(sep.spacing(12.0));

            ui.spacing_mut().button_padding = emath::vec2(25.0, 20.0);  
            if ui.add(egui::Button::new("Packet Sniffer ").text_style(egui::TextStyle::Heading)).clicked() 
            {                            
                         
                *packet_sniffer_panel = true;
                *port_scanner_panel = false;
            }
             
            
            ui.add_space(4.0); // space between buttons


            ui.spacing_mut().button_padding = emath::vec2(27.75, 20.0);
            if ui.add(egui::Button::new("Port Scanner  ").text_style(egui::TextStyle::Heading)).clicked() 
            {
                *port_scanner_panel = true;
                *packet_sniffer_panel = false;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.style_mut().body_text_style = egui::TextStyle::Small;
                
                
                ui.label("C. Norton, P. Chang, B. Prisbe, M. Posa");
                ui.label("Developed by");
                
            });

        });


        
        egui::CentralPanel::default().show(ctx, |ui| {

            if *packet_sniffer_panel
            {
                ui.heading("Packet Sniffer");

                let sep = egui::Separator::default();
                ui.add(sep.spacing(12.0));
            
                ui.spacing_mut().slider_width = 350.0;
                ui.add(egui::Slider::new (value, 0.0..=50.0).text("# of Packets to Capture").integer());
                
                ui.horizontal(|ui| { 
                    
                    if ui.add(egui::Button::new("Start").text_style(egui::TextStyle::Heading)).clicked()
                    {
                        
                        *label = "".to_string(); //resets the data 
                        let mut count = 0.0; // keeps track of # of packets grabbed
                
                        while count < *value // value = # of packets desired by user
                        {
                            label.push_str(&get_n_packets());                           
                            count += 1.0;
                        }
                                
                        if *value != 0.0 // so that clear button isn't active if there are no packets requested
                        {
                            *btn_clear_enabled = true;
                        }
                        *value = 0.0; // resets slider back to 0 packets
            
                      
                    }

                   
                 
            
                    /* TODO
                    
                     if ui.add_enabled(*btn_stop_enabled, egui::Button::new("Stop").text_style(egui::TextStyle::Heading)).clicked() 
                    {
                        *stop = true;
                        *btn_stop_enabled = false;    
                        
                    }
                    
                    */
                   
            
                    if ui.add_enabled(*btn_clear_enabled, egui::Button::new("Clear").text_style(egui::TextStyle::Heading)).clicked() 
                    {
                        *label = "".to_string(); //resets the data        
                    }
    
                }); // horizontal
    
                let sep = egui::Separator::default();
                
                ui.add(sep.spacing(12.0));
    
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.style_mut().body_text_style = egui::TextStyle::Monospace;
                    ui.label(label);
                });

            } // packet sniffer ui

            
            if *port_scanner_panel
            {
                ui.heading("Port Scanner");

                let sep = egui::Separator::default();
                ui.add(sep.spacing(12.0));

                ui.horizontal(|ui| { 
                    
                        /* These ifs manage the UI when the user wants to search all ports */
                     if ui.add(egui::Button::new("Scan all ports").text_style(egui::TextStyle::Heading)).clicked()
                    {
                        *scan_all_ports = true;
                        *scan_one_port = false;
                        *btn_clear_enabled = true;

                        *open_ports = port_scan(0); // gave bad value to induce port scan of all ports
                    }
                   
                                
                    
                    /* These ifs manage the UI when user wants to search for only one port */
                    if ui.add(egui::Button::new("Scan for specific port").text_style(egui::TextStyle::Heading)).clicked()
                    {
                        *scan_one_port = true;
                        *scan_all_ports = false;
                    }
                    
                      
                }); //Horizontal Layout
                
                if *scan_all_ports
                {
                    
                    
                    ui.add_space(5.0);

                    if ui.add(egui::Button::new("Clear").text_style(egui::TextStyle::Heading)).clicked()
                    {
                        *open_ports = "".to_string();
                        *scan_all_ports = false;
                    }
                    

                    let sep = egui::Separator::default();
                    ui.add(sep.spacing(12.0));

                    ui.style_mut().body_text_style = egui::TextStyle::Monospace;
                    ui.label(open_ports);

                }    
                else {
                    *open_ports = "".to_string(); //resets the output
                }

                
                if *scan_one_port
                {
                    

                    ui.add_space(5.0);

                    let sep = egui::Separator::default();
                    ui.add(sep.spacing(12.0));

                    ui.horizontal(|ui| {
                        ui.label("Enter Port Number: ");
                        ui.text_edit_singleline(port_to_search);
                    
                        if ui.add(egui::Button::new("Start").text_style(egui::TextStyle::Heading)).clicked()
                        {
                            let num : u16 = match port_to_search.parse::<u16>() {
                                Ok(n) => {
                                    n
                                },
                                Err(_) => {
                                    *scan_one_port_start = true;
                                    *open_single_port = "Please use valid port number".to_string();
                                    *port_to_search = "0".to_string();
                                    return;
                                },
                            };
                            
                            if num == 0
                            {
                                *scan_one_port_start = true;
                                *open_single_port = "Please use non-reserved port number".to_string();
                            }
                            else
                            {
                                *open_single_port = port_scan(num);
                                *scan_one_port_start = true;
                                *port_to_search = "0".to_string();
                            }

                            
                        }
                    });

                    if *scan_one_port_start
                    {
                        ui.add_space(5.0);
                        if ui.add(egui::Button::new("Clear").text_style(egui::TextStyle::Heading)).clicked()
                        {
                            *open_single_port = "".to_string();
                            *scan_one_port_start = false;
                        }

                        let sep = egui::Separator::default();
                        ui.add(sep.spacing(12.0));

                    
                        ui.style_mut().body_text_style = egui::TextStyle::Monospace;
                        ui.label(open_single_port);
                    }
                   
                }
                else {
                    *open_single_port = "".to_string(); //resets the output
                }
                

            } // port scanner ui
            
           
            
        }); //Center Panel

        
    } // update()
} // App
