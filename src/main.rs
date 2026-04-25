use gtk::prelude::*;
use relm4::prelude::*;
use std::process::Command;

struct App {
    sample_rate: u32,
    buffer_size: u32,
}

#[derive(Debug)]
enum Msg {
    SetSampleRate(u32),
    SetBufferSize(u32),
    Apply,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_title: Some("Pro Audio Control"),
            set_default_width: 400,
            set_default_height: 250,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                set_margin_all: 20,

                gtk::Label {
                    set_label: "Sample Rate",
                },

                gtk::DropDown {
                    set_model: Some(&gtk::StringList::new(&[
                        "44100",
                        "48000",
                        "96000",
                        "192000",
                    ])),
                    connect_selected_notify[sender] => move |dropdown| {
                        if let Some(item) = dropdown.selected_item() {
                            let value = item
                            .downcast_ref::<gtk::StringObject>()
                            .unwrap()
                            .string()
                            .parse::<u32>()
                            .unwrap();

                            sender.input(Msg::SetSampleRate(value));
                        }
                    },
                },

                gtk::Label {
                    set_label: "Buffer Size",
                },

                gtk::DropDown {
                    set_model: Some(&gtk::StringList::new(&[
                        "64",
                        "128",
                        "256",
                        "512",
                        "1024",
                    ])),
                    connect_selected_notify[sender] => move |dropdown| {
                        if let Some(item) = dropdown.selected_item() {
                            let value = item
                            .downcast_ref::<gtk::StringObject>()
                            .unwrap()
                            .string()
                            .parse::<u32>()
                            .unwrap();

                            sender.input(Msg::SetBufferSize(value));
                        }
                    },
                },

                gtk::Button {
                    set_label: "Apply",
                    connect_clicked => Msg::Apply,
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!(
                        "Current: {} Hz / {} buffer",
                        model.sample_rate,
                        model.buffer_size
                    ),
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            sample_rate: 48000,
            buffer_size: 128,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Msg, _sender: ComponentSender<Self>) {
        match msg {
            Msg::SetSampleRate(rate) => {
                self.sample_rate = rate;
            }
            Msg::SetBufferSize(size) => {
                self.buffer_size = size;
            }
            Msg::Apply => {
                println!(
                    "Applying: {} Hz, buffer {}",
                    self.sample_rate, self.buffer_size
                );

                // Set sample rate
                let rate_status = Command::new("pw-metadata")
                    .args([
                        "-n",
                        "settings",
                        "0",
                        "clock.force-rate",
                        &self.sample_rate.to_string(),
                    ])
                    .status();

                match rate_status {
                    Ok(status) if status.success() => {
                        println!("Sample rate applied successfully");
                    }
                    Ok(status) => {
                        eprintln!("Failed to set sample rate: {:?}", status);
                    }
                    Err(e) => {
                        eprintln!("Error running pw-metadata (rate): {}", e);
                    }
                }

                // Set buffer size (quantum)
                let buffer_status = Command::new("pw-metadata")
                    .args([
                        "-n",
                        "settings",
                        "0",
                        "clock.force-quantum",
                        &self.buffer_size.to_string(),
                    ])
                    .status();

                match buffer_status {
                    Ok(status) if status.success() => {
                        println!("Buffer size applied successfully");
                    }
                    Ok(status) => {
                        eprintln!("Failed to set buffer size: {:?}", status);
                    }
                    Err(e) => {
                        eprintln!("Error running pw-metadata (buffer): {}", e);
                    }
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("com.example.pro-audio-control");
    app.run::<App>(());
}
