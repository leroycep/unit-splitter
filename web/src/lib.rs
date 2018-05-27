extern crate stdweb;
#[macro_use]
extern crate yew;
extern crate unit_splitter_core as core;
extern crate slab;

use std::collections::HashMap;
use stdweb::web::Date;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use core::unit_requests::UnitRequests;
use slab::Slab;

#[derive(PartialEq, Eq, Hash)]
pub struct AmountId {
    test_id: usize,
    group_id: usize,
}

pub struct Model {
    unit_string: String,
    tests: Slab<String>,
    groups: Slab<String>,
    requests: HashMap<AmountId, u32>,
}

pub enum Msg {
    GotUnits(String),
    EditTestName(usize, String),
    RemoveTest(usize),
    AddTest,
    EditAmount(AmountId, String),
}

impl<CTX> Component<CTX> for Model
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
            Msg::GotUnits(value) => {
                self.unit_string = value;
                context.as_mut().log("unit string updated");
            }
            Msg::EditTestName(idx, value) => {
                self.tests[idx] = value;
                context.as_mut().log("test name updated");
            }
            Msg::RemoveTest(idx) => {
                context.as_mut().log(&format!("removed test \"{}\" from tests", &self.tests[idx]));
                self.tests.remove(idx);
            }
            Msg::AddTest => {
                self.tests.insert(String::new());
                context.as_mut().log("added test");
            }
            Msg::EditAmount(amount_id, amount_string) => {
                if let Ok(amount) = amount_string.parse() {
                    self.requests.insert(amount_id, amount);
                    context.as_mut().log("edited amount");
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
                <div>
                    <h1>{ "Units" }</h1>
                    <input type="text",
                        value=&self.unit_string,
                        oninput=|e: InputData| Msg::GotUnits(e.value),
                        placeholder="enter unit string",>
                    </input>
                </div>
                <div>
                    <h1>{ "Requests" }</h1>
                    <div>
                        <div><span>{"Test Name"}</span>{ self.view_group_headers() }</div>
                        { for self.tests.iter().map(|(i,name)| self.view_test(i, name)) }
                    </div>
                    <button onclick=|_| Msg::AddTest,>{ "[+]" }</button>
                </div>
                <div>
                    <h1>{ "Output" }</h1>
                    { for self.requests.iter().map(|(AmountId {test_id, group_id}, amount)| html! { <p>{
                                                  format!("{}, {}: {}", self.tests.get(*test_id).map(|s| s.as_str()).unwrap_or_else(|| "undefined"),
                                                         self.groups.get(*group_id).map(|s| s.as_str()).unwrap_or_else(|| "undefined"),
                                                         amount)}</p>
                                                  }) }
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_group_headers<CTX>(&self) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        html! {
            <span>
                { for self.groups.iter().map(|(_id, name)| html! { <span>{name}</span> }) }
            </span>
        }
    }

    fn view_test<CTX>(&self, test_id: usize, name: &str) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        html! {
            <div>
                <button onclick=move |_| Msg::RemoveTest(test_id),>{ "[-]" }</button>
                <input
                    type="text",
                    value=name,
                    oninput=move |e: InputData| Msg::EditTestName(test_id, e.value),
                    placeholder="enter test name",></input>
                { self.view_requests(test_id) }
            </div>
        }
    }

    fn view_requests<CTX>(&self, test_id: usize) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        html! {
            <span>
                { for self.groups.iter().map(|(group_id, _v)| self.view_request(test_id, group_id)) }
            </span>
        }
    }

    fn view_request<CTX>(&self, test_id: usize, group_id: usize) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static
    {
        let amount = self.requests.get(&AmountId {test_id, group_id}).map(|e| e.to_string()).unwrap_or_else(|| "".to_string());
        html! {
            <input
                type="number",
                value=amount,
                oninput=move |e: InputData| Msg::EditAmount(AmountId { test_id, group_id }, e.value),>
            </input>
        }
    }
}
