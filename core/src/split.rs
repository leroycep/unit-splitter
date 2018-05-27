
use range::Range;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct RequestId {
    pub group_id: u32,
    pub test_id: u32,
}

type GroupId = u32;
type TestId = u32;

#[derive(Clone, Debug)]
pub struct Ranges {
    ranges: HashMap<GroupId, VecDeque<Range>>,
}

impl Ranges {
    pub fn new() -> Self {
        Self {
            ranges: HashMap::new(),
        }
    }
}

impl Ranges {
    pub fn split(&self, requests: HashMap<RequestId, u32>) -> Result<(HashMap<TestId, Ranges>, Self), ()> {
        let mut group_ranges = self.ranges.clone();
        let mut used_group_ranges = HashMap::new();
        for (RequestId {group_id, test_id}, amount) in requests.iter() {
            let mut amount = *amount;
            let mut ranges = group_ranges.get_mut(&group_id).expect("request calls for non-existing group");
            let mut used_ranges = VecDeque::new();

            while amount > 0 {
                let range = ranges.pop_front();
                if range.is_none() {
                    return Err(());
                }
                let range = range.unwrap();
                let (used, unused, amount_left) = range.split(amount);
                amount = amount_left;
                used_ranges.push_back(used);
                if let Some(range) = unused {
                    ranges.push_front(range);
                }
            }

            let test_ranges = used_group_ranges.entry(*test_id).or_insert(Ranges::new());
            test_ranges.ranges.insert(*group_id, used_ranges);
        }
        return Ok((used_group_ranges, Ranges { ranges: group_ranges } ));
    }
}
