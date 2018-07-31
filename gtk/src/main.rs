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
//    units_string: String,
    pub relm: ::relm::Relm<Win>,
    procedures: Vec<(usize, String)>,
    procedure_widgets: Vec<Component<Procedure>>,
    procedure_next_id: usize,
//    requests: Vec<(GroupId, ProcedureId)>,
}

#[derive(Msg)]
pub enum Msg {
    AddProcedure,
    ProcedureNameChanged(usize, String),
    RemoveProcedure(usize),
    Quit,
}

impl Win {
    fn add_procedure(&mut self) {
        let id = self.model.procedure_next_id;
        self.model.procedure_next_id += 1;
        self.model.procedures.push((id, String::new()));
        self.update_procedures();
    }

    fn remove_procedure(&mut self, id: usize) {
        self.model.procedures.retain(|procedure| procedure.0 != id);
        self.update_procedures();
    }

    fn rename_procedure(&mut self, id: usize, name: String) {
        for procedure in self.model.procedures.iter_mut() {
            if procedure.0 == id {
                procedure.1 = name;
                break;
            }
        }
        // TODO: check if there was no procedure with that id?
    }

    fn update_procedures(&mut self) {
        self.clear();
        for procedure in self.model.procedures.iter() {
            let id = procedure.0;
            let widget = self.procedure_view.add_widget::<Procedure>(procedure.clone());

            connect!(
                widget@::procedure::Msg::Remove,
                self.model.relm,
                Msg::RemoveProcedure(id)
            );
            connect!(
                widget@::procedure::Msg::ChangeName(ref text),
                self.model.relm,
                Msg::ProcedureNameChanged(id, text.clone())
            );

            self.model.procedure_widgets.push(widget);
        }
    }

    fn clear(&mut self) {
        for widget in self.procedure_view.get_children().iter() {
            self.procedure_view.remove(widget);
        }
        self.procedure_view.show_all();
        self.model.procedure_widgets = Vec::new();
    }
}

#[widget]
impl Widget for Win {
    fn init_view(&mut self) {
        let test_output = vec![
            ("ESD CDM".into(), "A=1-32, B=36-67".into()),
            ("HAST".into(), "A=1-32, B=36-67".into()),
            ("Precision Drop".into(), "A=1-32, B=36-67".into()),
        ];
        self.output.emit(::output::Msg::UpdateOutput(test_output));
    }

    fn model(relm: &::relm::Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            procedures: vec![],
            procedure_widgets: vec![],
            procedure_next_id: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddProcedure => {
                self.add_procedure();
            },
            Msg::RemoveProcedure(id) => self.remove_procedure(id),
            Msg::ProcedureNameChanged(id, text) => self.rename_procedure(id, text),
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "Unit Splitter",
            property_default_width: 800,
            property_default_height: 640,
            gtk::Box {
                orientation: Vertical,
                gtk::Box {
                    orientation: Horizontal,
                    gtk::Frame {
                        label: "Units",
                        child: {
                            fill: true,
                            expand: true,
                        },
                        gtk::TextView {
                        },
                    },
                    gtk::Frame {
                        label: "Procedures",
                        child: {
                            fill: true,
                            expand: true,
                        },
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
