extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::{Relm, Widget, Update};
use gtk::{Inhibit, Window, WidgetExt, WindowType, Button, Label, ContainerExt, LabelExt, ButtonExt, Entry, EntryExt, EditableSignals};
use gtk::Orientation::Vertical;

struct Model {
    counter: i32,
}

#[derive(Msg)]
enum Msg {
    Increment,
    Decrement,
    Change(String),
    Quit,
}

#[derive(Clone)]
struct Widgets {
    counter_label: Entry,
    minus_button: Button,
    plus_button: Button,
    window: Window,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

impl Win {
    fn update_label(&mut self) {
        let label = &self.widgets.counter_label;
        label.set_text(&self.model.counter.to_string());
    }
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            counter: 0,
        }
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    // Futures and streams can be connected to send a message when a value is ready.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Increment => {
                self.model.counter += 1;
                self.update_label();
            },
            Msg::Decrement => {
                self.model.counter -= 1;
                self.update_label();
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
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let unit_string_frame = ::gtk::Frame::new("Unit String");
        let label = ::gtk::Label::new("Hello, world!");
        unit_string_frame.add(&label);
        vbox.add(&unit_string_frame);

        use gtk::{Type, TreeViewExt, StaticType, ListStoreExtManual};

        let unit_requests_frame = ::gtk::Frame::new("Unit Requests");
        let requests_store = ::gtk::ListStore::new(&[String::static_type(), String::static_type(), u32::static_type()]);
        requests_store.insert_with_values(None, &[0, 1, 2], &[&"ESD CDM", &"A", &19]);
        let requests_view = ::gtk::TreeView::new();
        append_column(&requests_view, 0);
        append_column(&requests_view, 1);
        append_column(&requests_view, 2);
        requests_view.set_model(Some(&requests_store));
        unit_requests_frame.add(&requests_view);
        vbox.add(&unit_requests_frame);

        let output_frame = ::gtk::Frame::new("Output");
        let label = ::gtk::Label::new("ESD CDM | A=1-19");
        output_frame.add(&label);
        vbox.add(&output_frame);

        let plus_button = Button::new_with_label("+");
        vbox.add(&plus_button);

        let counter_label = Entry::new();
        counter_label.set_text("0");
        vbox.add(&counter_label);

        let minus_button = Button::new_with_label("-");
        vbox.add(&minus_button);
        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);

        window.add(&vbox);

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, plus_button, connect_clicked(_), Msg::Increment);
        connect!(relm, minus_button, connect_clicked(_), Msg::Decrement);
        connect!(relm, counter_label, connect_changed(entry), Msg::Change(entry.get_text().unwrap()));
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win {
            model,
            widgets: Widgets { counter_label, plus_button, minus_button, window },
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}

use gtk::{TreeView, CellRendererText, TreeViewColumn, CellLayoutExt, TreeViewExt};
fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

