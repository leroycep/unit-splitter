// Used to learn from: https://www.geeksforgeeks.org/interval-tree/

use ::range::Range;

pub struct IntervalTreeNode {
    interval: Range,
    max: usize,
    left: Option<Box<IntervalTreeNode>>,
    right: Option<Box<IntervalTreeNode>>,
}

impl IntervalTreeNode {
    pub fn new(interval: Range) -> Self {
        let max = interval.last();
        let left = None;
        let right = None;
        IntervalTreeNode {
            interval,
            max,
            left,
            right
        }
    }

    pub fn insert(&mut self, interval: Range) {
        let l = self.interval.first();

        if self.max < interval.last() {
            self.max = interval.last();
        }

        if interval.first() < l {
            match self.left {
                Some(ref mut node) => {
                    node.insert(interval);
                }
                None => {
                    self.left = Some(Box::new(IntervalTreeNode::new(interval)));
                }
            }
        } else {
            match self.right {
                Some(ref mut node) => {
                    node.insert(interval);
                }
                None => {
                    self.right = Some(Box::new(IntervalTreeNode::new(interval)));
                }
            }
        }
    }

    pub fn overlap_search(&self, interval: &Range, overlaps: &mut Vec<Range>) {
        if interval.overlaps(&self.interval) {
            overlaps.push(self.interval.clone());
        }

        if let Some(ref node) = self.left {
            if node.max >= interval.first() {
                node.overlap_search(interval, overlaps);
            }
        }

        if let Some(ref node) = self.right {
            node.overlap_search(interval, overlaps);
        }
    }
}
