#![feature(use_extern_macros)]

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;
extern crate unit_splitter_core as core;

use relm::{Widget, Component, ContainerWidget};
use relm_attributes::widget;
use gtk::prelude::*;
use gtk::{Inhibit, WidgetExt, ButtonExt};
use gtk::Orientation::{Vertical,Horizontal};
use core::group::Group;

mod widgets;
mod procedure;
mod output;

use widgets::units::Units;
use widgets::units::Msg::UpdateUnits as UpdateUnitsEvent;
use widgets::request::Msg::AmountEdited as AmountEditedEvent;
use widgets::request::Request;
use procedure::Procedure;
use output::Output as OutputWidget;

type GroupId = usize;
type ProcedureId = usize;

pub struct Model {
    pub relm: ::relm::Relm<Win>,
    units: Vec<Group>,
    procedures: Vec<(usize, String)>,
    procedure_next_id: usize,
    procedure_widgets: Vec<Component<Procedure>>,
    requests: Vec<(GroupId, ProcedureId, usize)>,
    request_widgets: Vec<Component<Request>>,
}

#[derive(Msg)]
pub enum Msg {
    UnitsUpdated(Vec<Group>),
    AddProcedure,
    ProcedureNameChanged(usize, String),
    RemoveProcedure(usize),
    RequestAmountEdited(usize, usize, usize),
    Quit,
}

impl Win {
    fn add_procedure(&mut self) {
        let id = self.model.procedure_next_id;
        self.model.procedure_next_id += 1;
        self.model.procedures.push((id, String::new()));
        self.update_procedures();
        self.update_requests();
    }

    fn remove_procedure(&mut self, id: usize) {
        self.model.procedures.retain(|procedure| procedure.0 != id);
        self.update_procedures();
        self.update_requests();
    }

    fn rename_procedure(&mut self, id: usize, name: String) {
        for procedure in self.model.procedures.iter_mut() {
            if procedure.0 == id {
                procedure.1 = name;
                break;
            }
        }
        self.update_requests();
        // TODO: check if there was no procedure with that id?
    }

    fn set_request_amount(&mut self, group_id: usize, procedure_id: usize, amount: usize) {
        let mut request_found = false;
        for request in self.model.requests.iter_mut() {
            if request.0 == group_id && request.1 == procedure_id {
                request.2 = amount;
                request_found = true;
                break;
            }
        }
        if !request_found {
            println!("Request not found");
        }
        self.update_output();
    }

    fn units_updated(&mut self, units: Vec<Group>) {
        self.model.units = units;
        self.update_requests();
    }

    fn update_procedures(&mut self) {
        self.clear_procedure_view();
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

    fn update_requests(&mut self) {
        self.clear_requests_view();
        self.model.requests.clear();
        for (group_id, group) in self.model.units.iter().enumerate() {
            for procedure in self.model.procedures.iter() {
                let widget = self.requests_view.add_widget::<Request>((group.name().to_string(), procedure.1.clone(), 0));

                let procedure_id = procedure.0;

                connect!(
                    widget@AmountEditedEvent(amount),
                    self.model.relm,
                    Msg::RequestAmountEdited(group_id, procedure_id, amount)
                );

                self.model.request_widgets.push(widget);
                self.model.requests.push((group_id, procedure_id, 0));
            }
        }
    }

    fn update_output(&mut self) {
        // TODO:
    }

    fn clear_requests_view(&mut self) {
        for widget in self.requests_view.get_children().iter() {
            self.requests_view.remove(widget);
        }
        self.requests_view.show_all();
        self.model.request_widgets = Vec::new();
    }

    fn clear_procedure_view(&mut self) {
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

        let widget = self.requests_view.add_widget::<Request>(("Hello".into(), "world".into(), 0));
        self.model.request_widgets.push(widget);
    }

    fn model(relm: &::relm::Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            units: vec![],
            procedures: vec![],
            procedure_widgets: vec![],
            procedure_next_id: 0,
            requests: vec![],
            request_widgets: vec![],
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UnitsUpdated(units) => self.units_updated(units),
            Msg::AddProcedure => {
                self.add_procedure();
            },
            Msg::RemoveProcedure(id) => self.remove_procedure(id),
            Msg::ProcedureNameChanged(id, text) => self.rename_procedure(id, text),
            Msg::RequestAmountEdited(group_id, procedure_id, amount) => self.set_request_amount(group_id, procedure_id, amount),
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
                        Units {
                            UpdateUnitsEvent(ref units) => Msg::UnitsUpdated(units.clone()),
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
                    #[name="requests_view"]
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
