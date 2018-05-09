
use std::collections::HashMap;

type UnitRequestId = u32;
type GroupName = String;

pub struct UnitRequests {
    names: HashMap<UnitRequestId, String>,
    amounts: HashMap<(UnitRequestId, GroupName), u32>,
}

impl UnitRequests {
    pub fn amounts(&self) -> HashMap<(UnitRequestId, GroupName), u32> {
        self.amounts.clone()
    }
}

#[cfg(test)]
mod tests {
    // TODO: add tests?
}
