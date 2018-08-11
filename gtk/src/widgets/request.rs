use gtk;
use gtk::prelude::*;
use relm::Widget;
use relm_attributes::widget;

#[derive(Clone)]
pub struct Model {
    pub relm: ::relm::Relm<Request>,
    pub group_name: String,
    pub procedure_name: String,
    pub amount: usize,
}

#[derive(Msg)]
pub enum Msg {
    EditAmount(String),
    AmountEdited(usize),
}

impl Request {
    fn emit_update(&mut self) {
        self.model.relm.stream().emit(Msg::AmountEdited(self.model.amount));
    }
}

#[widget]
impl Widget for Request {
    fn init_view(&mut self) {
        self.group_name.set_text(&self.model.group_name);
        self.procedure_name.set_text(&self.model.procedure_name);
    }

    fn model(relm: &::relm::Relm<Self>, (group_name, procedure_name, amount): (String, String, usize)) -> Model {
        Model {
            relm: relm.clone(),
            group_name: group_name,
            procedure_name: procedure_name,
            amount: amount,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::EditAmount(text) => {
                println!("Amount edited: {} {} {}", self.model.group_name, self.model.procedure_name, text);
                match text.parse::<usize>() {
                    Ok(amount) => {
                        println!("Amount number edited: {} {} {}", self.model.group_name, self.model.procedure_name, text);
                        self.model.amount = amount;
                        self.emit_update();
                    },
                    Err(_) => {}
                }
            }
            Msg::AmountEdited(_) => {
                println!("Amount edited event emitted");
            }
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Horizontal,
            spacing: 10,

            #[name="group_name"]
            gtk::Label {
                child: {
                    expand: true,
                    fill: true,
                },
            },
            #[name="procedure_name"]
            gtk::Label {
                child: {
                    expand: true,
                    fill: true,
                },
            },
            gtk::Entry {
                input_purpose: gtk::InputPurpose::Digits,
                changed(entry) => Msg::EditAmount(entry.get_text().unwrap()),
            },
        },
    }
}

