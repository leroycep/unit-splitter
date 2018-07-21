#![feature(use_extern_macros)]

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;

use relm::{Widget, Component, ContainerWidget};
use relm_attributes::widget;
use gtk::prelude::*;
use gtk::{Inhibit, WidgetExt, ButtonExt};
use gtk::Orientation::{Vertical,Horizontal};

mod procedure;

use procedure::Procedure;

pub struct Model {
    counter: i32,
    procedures: Vec<Component<Procedure>>,
    procedure_next_id: usize,
}

#[derive(Msg)]
pub enum Msg {
    AddProcedure,
    Decrement,
    Change(String),
    Quit,
}

impl Win {
    fn add_procedure(&mut self) {
        let widget = self.procedure_view.add_widget::<Procedure>(self.model.procedure_next_id);
        self.model.procedure_next_id += 1;
        self.model.procedures.push(widget);
    }
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            counter: 0,
            procedures: vec![],
            procedure_next_id: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddProcedure => {
                self.add_procedure();
            },
            Msg::Decrement => {
                self.model.counter -= 1;
            },
            Msg::Change(text) => {
                match text.parse::<i32>() {
                    Ok(num) => {
                        self.model.counter = num;
                    }
                    Err(_) => {}
                }
            },
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "Unit Splitter",
            gtk::Box {
                orientation: Vertical,
                gtk::Box {
                    orientation: Horizontal,
                    gtk::Frame {
                        label: "Units",
                        gtk::TextView {
                        },
                    },
                    gtk::Frame {
                        label: "Procedures",
                        gtk::Box {
                            orientation: Vertical,
                            #[name="procedure_view"]
                            gtk::ListBox {
                                child: {
                                    fill: true,
                                    expand: true,
                                },
                            },
                            gtk::Button {
                                label: "Add Procedure",
                                clicked(_) => Msg::AddProcedure,
                            }
                        }
                    },
                },
                gtk::Frame {
                    label: "Requests",
                    gtk::ListBox {
                    }
                },
                gtk::Frame {
                    label: "Output",
                    gtk::ListBox {
                        gtk::Box {
                            orientation: Horizontal,
                            spacing: 10,
                            gtk::Label {
                                label: "ESD CDM",
                            },
                            gtk::Label {
                                label: "A=1-32,B=36-45",
                            },
                        },
                    }
                },
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
fn main() {
    Win::run(()).unwrap();
}

