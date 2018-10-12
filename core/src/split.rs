use range::Range;
use std::collections::HashMap;
use std::collections::VecDeque;

pub type GroupId = usize;
pub type TestId = usize;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct RequestId {
    pub group_id: GroupId,
    pub test_id: TestId,
}

pub type Ranges = HashMap<GroupId, VecDeque<Range>>;

pub fn split(
    ranges: &Ranges,
    requests: &HashMap<RequestId, usize>,
    test_order: &[TestId],
    group_order: &[GroupId],
) -> Result<(HashMap<TestId, Ranges>, Ranges), ()> {
    let mut group_ranges = ranges.clone();
    let mut used_group_ranges = HashMap::new();
    for &test_id in test_order.iter() {
        for &group_id in group_order.iter() {
            let request_id = RequestId { test_id, group_id };
            let amount = requests.get(&request_id).unwrap_or(&0);

            let mut amount = *amount;
            let mut ranges = group_ranges
                .get_mut(&group_id)
                .expect("request calls for non-existing group");
            let mut used_ranges = VecDeque::new();

            while amount > 0 {
                let range = ranges.pop_front();
                if range.is_none() {
                    return Err(());
                }
                let range = range.expect("if returns error before this line can be reached");
                let (used, unused, amount_left) = range.split(amount);
                amount = amount_left;
                used_ranges.push_back(used);
                if let Some(range) = unused {
                    ranges.push_front(range);
                }
            }

            let test_ranges = used_group_ranges.entry(test_id).or_insert(Ranges::new());
            test_ranges.insert(group_id, used_ranges);
        }
    }
    return Ok((used_group_ranges, group_ranges));
}
