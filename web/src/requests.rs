
use std::collections::HashMap;
use slab::Slab;

pub struct Requests {
    tests: Slab<String>,
    groups: Slab<String>,
    requests: HashMap<(usize, usize), u32>,
}

pub enum Msg {
    AddTest,
    EditTestName(usize, String),
    RemoveTest(usize),
    EditAmount(usize, usize, String),
}

impl<CTX> Component<CTX> for Requests
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        let mut groups = Slab::new();
        groups.insert("A".into());
        groups.insert("B".into());
        Model {
            unit_string: "".into(),
            tests: Slab::new(),
            groups: groups,
            requests: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::AddTest => {
                self.tests.insert(String::new());
                context.as_mut().log("added test");
            }
            Msg::EditTestName(idx, value) => {
                self.tests[idx] = value;
                context.as_mut().log("test name updated");
            }
            Msg::RemoveTest(idx) => {
                context.as_mut().log(&format!("removed test \"{}\" from tests", &self.tests[idx]));
                self.tests.remove(idx);
            }
            Msg::EditAmount(test_id, group_id, amount_string) => {
                if let Ok(amount) = amount_string.parse() {
                    self.requests.insert((test_id, group_id), amount);
                    context.as_mut().log("edited amount");
                } else {
                    context.as_mut().log("failed to edit amount");
                }
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <h1>{ "Requests" }</h1>
                <div>
                    <div><span>{"Test Name"}</span>{ self.view_group_headers() }</div>
                    { for self.tests.iter().map(|(i,name)| self.view_test(i, name)) }
                </div>
                <button onclick=|_| Msg::AddTest,>{ "[+]" }</button>
            </div>
        }
    }
}
