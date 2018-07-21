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
mod output;

use procedure::Procedure;
use output::Output as OutputWidget;

pub struct Model {
    procedures: Vec<Component<Procedure>>,
    procedure_next_id: usize,
}

#[derive(Msg)]
pub enum Msg {
    AddProcedure,
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
    fn init_view(&mut self) {
        self.output.emit(::output::Msg::UpdateOutput(vec![ ("ESD CDM".into(), "A=1-32, B=36-67".into()) ]));
    }

    fn model() -> Model {
        Model {
            procedures: vec![],
            procedure_next_id: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddProcedure => {
                self.add_procedure();
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
                    #[name="output"]
                    OutputWidget {
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
