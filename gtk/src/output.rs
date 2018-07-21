use gtk;
use gtk::prelude::*;
use relm_attributes::widget;

#[derive(Msg)]
pub enum Msg {
    UpdateOutput(Vec<(String, String)>),
}

pub struct Model {
    store: ::gtk::ListStore,
}

impl Output {
    fn update_output(&mut self, output: &[(String, String)]) {
        self.model.store.clear();
        println!("Output");
        for (procedure_name, units) in output {
            println!("\t{} | {}", procedure_name, units);
            let row = self.model.store.append();
            self.model.store.set_value(&row, 0, &procedure_name.to_value());
            self.model.store.set_value(&row, 1, &units.to_value());
        }
    }
}

#[widget]
impl ::relm::Widget for Output {
    fn init_view(&mut self) {
        self.tree_view.set_model(Some(&self.model.store));

        let column = ::gtk::TreeViewColumn::new();
        column.set_title("Procedure");
        self.tree_view.append_column(&column);

        let cell = ::gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);


        let column = ::gtk::TreeViewColumn::new();
        column.set_title("Units");
        self.tree_view.append_column(&column);

        let cell = ::gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 1);
    }

    fn model() -> Model {
        Model {
            store: ::gtk::ListStore::new(&[String::static_type(), String::static_type()]),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdateOutput(output) => {
                self.update_output(&output);
            }
        }
    }

    view! {
        #[name="tree_view"]
        gtk::TreeView {
        }
    }
}
