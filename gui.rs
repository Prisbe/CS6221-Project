use eframe::{egui, epi};
use crate::packetsniffer::get_n_packets;

pub struct OurApp {
    
    label: String,

   
    value: f32,


    btn_stop_enabled: bool,

    btn_clear_enabled: bool,

    packet_sniffer_panel : bool,

    port_scanner_panel : bool,

    port : String,

    scan_one_port : bool,

    scan_all_ports : bool
}

impl Default for OurApp {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            value: 0.0,
            btn_stop_enabled: false,
            btn_clear_enabled: false,
            packet_sniffer_panel : false,
            port_scanner_panel : false,
            port : "0".to_string(),
            scan_one_port : false,
            scan_all_ports : false
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
        let Self { label, value , btn_stop_enabled, btn_clear_enabled,
             packet_sniffer_panel, port_scanner_panel, port, scan_one_port,scan_all_ports} = self;

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
            ui.heading("Choose Tool");

            if ui.add(egui::Button::new("Packet Sniffer ")).clicked() 
            {                                       
                *packet_sniffer_panel = true;
                *port_scanner_panel = false;
            }
                
                
            if ui.add(egui::Button::new("Port Scanner  ")).clicked() 
            {
                *port_scanner_panel = true;
                *packet_sniffer_panel = false;
            }


        });


        
        egui::CentralPanel::default().show(ctx, |ui| {

            if *packet_sniffer_panel
            {
                ui.heading("Packet Sniffer");
            
                ui.add(egui::Slider::new (value, 0.0..=10.0).text("# of Packets to Capture").integer());
      
                ui.horizontal(|ui| { 
                    if ui.add(egui::Button::new("Start")).clicked() 
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
            
                    if ui.add_enabled(*btn_stop_enabled, egui::Button::new("Stop")).clicked() 
                    {
                        // TODO        
                    }
            
                    if ui.add_enabled(*btn_clear_enabled, egui::Button::new("Clear")).clicked() 
                    {
                        *label = "".to_string(); //resets the data        
                    }
    
                }); // horizontal
    
                let sep = egui::Separator::default();
                
                ui.add(sep.spacing(12.0));
    
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(label);
                });

            } // packet sniffer ui

            
            if *port_scanner_panel
            {
                ui.heading("Port Scanner");
                
                /* These ifs manage the UI when the user wants to search all ports */
                if ui.add(egui::Button::new("Scan all ports")).clicked()
                {
                    *scan_all_ports = true;
                    *scan_one_port = false;
                }
                if *scan_all_ports
                {
                    let sep = egui::Separator::default();
                    ui.add(sep.spacing(12.0));

                    ui.label("TODO");
                }

                /* These ifs manage the UI when user wants to search for only one port */
                if ui.add(egui::Button::new("Scan for specific port")).clicked()
                {
                    *scan_one_port = true;
                    *scan_all_ports = false;
                }
                if *scan_one_port
                {
                    ui.horizontal(|ui| {
                        ui.label("Enter Port Number: ");
                        ui.text_edit_singleline(port);
                    });

                    let sep = egui::Separator::default();
                    ui.add(sep.spacing(12.0));
                }
    
                

            } // port scanner ui
            
           
            
        }); //Center Panel
    } // update()
} // App
