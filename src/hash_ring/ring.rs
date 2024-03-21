use core::fmt;
use std::cmp;
use std::hash::BuildHasher;
use std::hash::Hash;

use siphasher::sip::SipHasher;
use tracing::warn;

/// The default slot size
const DEFAULT_SLOT_SIZE: u64 = 1024;
/// The default ring load factor
const RING_LOAD: f64 = 0.75;

/// A trait for types that support copy, clone, and print
pub trait NodeType: Copy + Clone + PartialEq + Hash + Eq {}

// impl<T> NodeType for T where T: Copy + Clone + std::fmt::Debug {}

/// A slot definition in the hash ring
#[derive(Clone)]
#[allow(dead_code)]
pub struct Slot<T> 
where T: NodeType
{
    /// The start offset of the slot
    start: u64,
    /// The end offset of the slot
    end: u64,
    /// The slot data, contains mapping info
    inner: T,
}

impl <T> Slot<T>
where T: NodeType
{
    /// Create a new slot
    pub fn new(start: u64, end: u64, inner: T) -> Self {
        Self {
            start,
            end,
            inner,
        }
    }

    /// Get the start offset of the slot
    pub fn start(&self) -> u64 {
        self.start
    }

    /// Get the end offset of the slot
    pub fn end(&self) -> u64 {
        self.end
    }

    /// Get the slot data
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl <T: fmt::Debug> fmt::Debug for Slot<T>
where T: NodeType
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Slot: start: {}, end: {}, data: {:?}", self.start, self.end, self.inner)
    }
}

impl<T> PartialEq for Slot<T>
where T: NodeType
{
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl<T> Eq for Slot<T> where T: NodeType {}

impl<T> Ord for Slot<T>
where
    T: NodeType,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl<T> PartialOrd for Slot<T> 
where T: NodeType
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

/// The default hash builder
#[derive(Debug, Clone)]
pub struct DefaultHashBuilder;

impl BuildHasher for DefaultHashBuilder {
    type Hasher = SipHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SipHasher::new()
    }
}

/// The hash ring data structure
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Ring<T, S = DefaultHashBuilder>
where T: NodeType,
      S: BuildHasher
{
    /// The hash builder
    hash_builder: S,
    /// The slots
    slots: Vec<Slot<T>>,
    /// T to slot id mapping, accelerate finding the slot
    /// The slot step of the ring
    capacity: u64,
    /// The version of the ring
    version: u64,
}

impl<T> Default for Ring<T>
where T: NodeType
{
    fn default() -> Self {
        Ring {
            hash_builder: DefaultHashBuilder,
            slots: Vec::new(),
            capacity: DEFAULT_SLOT_SIZE,
            version: 0,
        }
    }
}

impl <T, S> Ring<T, S>
where T: NodeType,
      S: BuildHasher
{
    /// Create a new hash ring with a given hash builder and capacity
    pub fn new(hash_builder: S, capacity: u64) -> Self {
        Self {
            hash_builder,
            slots: Vec::new(),
            capacity,
            version: 0,
        }
    }

    /// Get the slot length
    pub fn len_slots(&self) -> usize {
        self.slots.len()
    }

    /// Get the slot at a given index
    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    /// Get version
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Check if the ring is empty
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Clear the ring
    pub fn slots_clear(&mut self) {
        self.slots.clear();
    }
}

impl<T, S:BuildHasher> Ring<T, S>
where T: NodeType,
      S: BuildHasher
{
    /// Add a node to a slot
    /// We will create a new slot and update slot mapping, then add to the ring
    /// If must is true, the ring need to be rebalanced or expanded
    pub fn add(&mut self, node: T, must: bool) -> Option<T> {
        // If the ring is full, return None
        if self.slots.len() >= self.capacity as usize {
            return None;
        }

        // Try to modify the ring, so we need to increase the version
        // TODO1: if the version is too large, we need to reset it
        // if th slot allocation failed, we need to keep the version
        self.version += 1;

        // If there are no slots, add the first one covering the whole range
        if self.slots.is_empty() {
            let new_slot = Slot::new(1, self.capacity, node);
            self.slots.push(new_slot);

            return Some(node);
        }

        // Try to judge if the ring need to be expanded
        while self.slots.len() >= self.capacity as usize * RING_LOAD as usize && must {
            self.expand();
        }

        // Find the slot with the largest range
        let (index, _) = self.slots.iter().enumerate().max_by_key(|(_, slot)| slot.end - slot.start).unwrap();

        // Calculate the new ranges for the split
        let slot_to_split = &self.slots[index];
        let mid_point = (slot_to_split.start + slot_to_split.end) / 2;

        // Create new slot with the second half of the range
        let new_slot = Slot::new(mid_point + 1, slot_to_split.end, node);

        // Update the end of the existing slot to the mid_point
        self.slots[index].end = mid_point;

        // Insert the new slot to index+1, and shift the rest of the slots
        self.slots.insert(index + 1, new_slot);

        // Try to rebalance the ring
        // If must is true and the rebalance failed, return None
        if must && !self.rebalance() {
            warn!("Rebalance failed");
            
            return None;
        }

        Some(node)
    }

    /// Add a batch of slots
    /// If must is true, the ring need to be rebalanced or expanded
    pub fn batch_add(&mut self, nodes: Vec<T>, must: bool) -> Option<Vec<T>> {
        // If must is true, we need to expand the ring
        if must && (self.slots.len() + nodes.len() > self.capacity as usize) {
            if !self.expand() {
                warn!("Expand failed");
                
                return None;
            }

            if !self.rebalance() {
                warn!("Rebalance failed");
                
                return None;
            }
        } else if self.slots.len() + nodes.len() > self.capacity as usize {
            // If not satisfy the expand condition but too many nodes, return None
            return None;
        }

        // Store the success nodes
        let mut success_nodes = Vec::new();

        // Iterate the nodes to add
        for node in nodes {
            // Try to rebalance it later
            self.add(node, false).map(|n| success_nodes.push(n));
        }

        // Try to rebalance the ring
        if must {
            self.rebalance();
        }

        Some(success_nodes)
    }

    /// Remove a slot
    /// If must is true, the ring need to be rebalanced
    pub fn remove(&mut self, node: T, must: bool) -> Option<T> {
        // Find the slot to remove
        // TODO: Find the slot with faster way?
        let index = self.slots.iter().position(|slot| slot.inner == node);

        // If the slot is not found, return None
        let index = match index {
            Some(index) => index,
            None => return None,
        };

        // Remove the slot by index
        let removed = self.remove_by_index(index, false);
        if removed.is_none() {
            return None;
        }

        // Try to rebalance the ring
        if must {
            self.rebalance();
        }

        Some(node)
    }

    /// Remove a slot by index
    pub fn remove_by_index(&mut self, index: usize, must: bool) -> Option<T> {
        // Try to modify the ring, so we need to increase the version
        // TODO1: if the version is too large, we need to reset it
        // TODO2: if the slot allocation failed, we need to keep the version
        // Maybe we need to get the atomic function to update this version
        self.version += 1;

        // If the slot is not found, return None
        if index >= self.slots.len() {
            return None;
        }

        // Remove the slot, shift the rest of the slots
        let removed_slot = self.slots.remove(index);

        // Merge current slot range to previous slot
        if index > 0 {
            // other slots
            self.slots[index - 1].end = removed_slot.end;
        } else if self.slots.len() > 0 {
            // first slot, try to merge to the next slot
            self.slots[0].start = removed_slot.start;
        }

        // Try to rebalance the ring
        if must && !self.rebalance() {
            warn!("Rebalance failed");
            
            return None;
        }

        Some(removed_slot.inner)
    }

    /// Remove a batch of slots
    /// If must is true, the ring need to be rebalanced or expanded
    pub fn batch_remove(&mut self, nodes: Vec<T>, must: bool) -> Option<Vec<T>> {
        // TODO: Find the slot with faster way?
        let mut indexes_to_remove: Vec<usize> = nodes.iter().filter_map(|node| {
            self.slots.iter().position(|slot| &slot.inner == node)
        }).collect();

        // Try to modify the ring, so we need to increase the version
        indexes_to_remove.sort_unstable_by(|a, b| b.cmp(a));

        let mut success_nodes = Vec::new();

        // If must is true, we need to expand the ring
        // Find the slot to remove
        for index in indexes_to_remove {
            self.remove_by_index(index, false).map(|n| success_nodes.push(n));
        }

        // Try to rebalance the ring
        if must && !self.rebalance() {
            warn!("Rebalance failed");
            
            return None;
        }

        Some(success_nodes)
    }

    /// Get the slot of a given key
    pub fn get_slot<U: Hash>(&self, key: &U) -> Option<&Slot<T>> {
        if self.slots.is_empty() {
            return None;
        }

        let idx = get_hash(&self.hash_builder, key) % self.capacity;

        // Find the slot with binary search
        match self.slots.binary_search_by(|slot| 
            slot.start.cmp(&idx)
        ) {
            Err(index) => {
                if index == 0 {
                    // redirect to the last slot(ring)
                    Some(&self.slots[self.slots.len() - 1])
                } else {
                    // previous start index
                    Some(&self.slots[index - 1])
                }
            },
            Ok(index) => Some(&self.slots[index]),
        }
    }

    /// Get the node of a given key
    pub fn get_node<U: Hash>(&self, key: &U) -> Option<&T> {
        self.get_slot(key).map(|slot| slot.inner())
    }

    /// Get the replicas slots of a given key
    /// if n is larger than the slot size, return all slots
    pub fn get_replicas<U: Hash>(&self, key: &U, n: usize) -> Option<Vec<&Slot<T>>> {
        if self.slots.is_empty() {
            return None;
        }

        if n > self.slots.len() {
            return Some(self.slots.iter().collect());
        }

        let idx = get_hash(&self.hash_builder, key) % self.capacity;

        // Find the slot with binary search
        // If the idx is not in slot start, binary search will return the next slot
        // We can set the index to the range start
        match self.slots.binary_search_by(|slot| 
            slot.start.cmp(&idx)
        ) {
            Err(index) => {
                // If the key is not in the slots, return the last n slots
                Some(self.slots.iter().cycle().skip(index - 1).take(n).collect())
            },
            // If the key is in the slots, return the next n slots
            // If the left slot is not enough, cycle the slots and take the rest
            Ok(index) => Some(self.slots.iter().cycle().skip(index).take(n).collect()),
        }
    }

    /// Rebalance the ring
    /// Try to rebalance the ring
    pub fn rebalance(&mut self) -> bool {
        if self.slots.is_empty() {
            return false;
        }
    
        // update version
        self.version += 1;

        // calculate new slot size
        let total_range = self.capacity;
        let new_slot_size = total_range / self.slots.len() as u64;
        let mut start = 1u64;
    
        // update slot range
        for slot in self.slots.iter_mut() {
            slot.start = start;
            start += new_slot_size;
            slot.end = start - 1;
        }
    
        // update the last slot
        if let Some(last_slot) = self.slots.last_mut() {
            last_slot.end = self.capacity;
        }
    
        true
    }

    /// Expand the ring
    /// Try to expand the ring to a new capacity, default times is 2
    pub fn expand(&mut self) -> bool {
        if self.slots.is_empty() {
            return false;
        }

        // update version
        self.version += 1;

        // calculate new capacity
        if let Some(new_capacity) = self.capacity.checked_mul(2) {
            self.capacity = new_capacity;
        } else {
            return false;
        }

        // calculate new slot size
        let total_range = self.capacity;
        let new_slot_size = total_range / self.slots.len() as u64;
        let mut start = 1u64;

        // update slot range
        for slot in self.slots.iter_mut() {
            slot.start = start;
            start += new_slot_size;
            slot.end = start - 1;
        }

        // update the last slot
        if let Some(last_slot) = self.slots.last_mut() {
            last_slot.end = self.capacity;
        }

        // update the capacity
        self.capacity = self.capacity;

        true
    }
}

/// Get the hash index of a key
fn get_hash<T, S>(hash_builder: &S, key: T) -> u64
where T: Hash,
      S: BuildHasher
{
    hash_builder.hash_one(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Node {
        id: u64,
    }

    impl NodeType for Node {}

    #[test]
    fn test_slot() {
        let node = Node { id: 1 };
        let slot = Slot::new(1, 10, node);

        assert_eq!(slot.start(), 1);
        assert_eq!(slot.end(), 10);
        assert_eq!(slot.inner().id, 1);
    }

    #[test]
    fn test_expand() {
        let node1 = Node { id: 1 };
        let node2 = Node { id: 2 };
        let node3 = Node { id: 3 };

        let mut ring = Ring::new(DefaultHashBuilder, 1024);

        ring.add(node1, false);
        ring.add(node2, false);
        ring.add(node3, false);

        assert_eq!(ring.len_slots(), 3);
        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.version(), 3);

        ring.expand();

        assert_eq!(ring.len_slots(), 3);
        assert_eq!(ring.capacity(), 2048);
        assert_eq!(ring.version(), 4);
    }

    #[test]
    fn test_ring() {
        let node1 = Node { id: 1 };
        let node2 = Node { id: 2 };
        let node3 = Node { id: 3 };

        let mut ring = Ring::new(DefaultHashBuilder, 1024);

        ring.add(node1, false);
        ring.add(node2, false);
        ring.add(node3, false);

        assert_eq!(ring.len_slots(), 3);
        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.version(), 3);

        ring.remove(node2, false);

        assert_eq!(ring.len_slots(), 2);
        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.version(), 4);
    }

    #[test]
    fn test_batch_add() {
        let node1 = Node { id: 1 };
        let node2 = Node { id: 2 };
        let node3 = Node { id: 3 };

        let mut ring = Ring::new(DefaultHashBuilder, 1024);

        ring.batch_add(vec![node1, node2, node3], false);

        assert_eq!(ring.len_slots(), 3);
        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.version(), 3);

        // node1
        assert_eq!(ring.get_slot(&1).unwrap().start, 1);
        assert_eq!(ring.get_slot(&1).unwrap().end, 512);
        // node2
        assert_eq!(ring.get_slot(&999).unwrap().start, 769);
        assert_eq!(ring.get_slot(&999).unwrap().end, 1024);
        // node3
        assert_eq!(ring.get_slot(&10000).unwrap().start, 513);
        assert_eq!(ring.get_slot(&10000).unwrap().end, 768);

        ring.slots_clear();
        assert_eq!(ring.version(), 3);

        ring.batch_add(vec![node1, node2, node3], true);

        assert_eq!(ring.len_slots(), 3);
        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.version(), 7); // 3 + 3 + 1(rebalance)

        // node1
        assert_eq!(ring.get_slot(&1).unwrap().start, 1);
        assert_eq!(ring.get_slot(&1).unwrap().end, 341);
        // node2
        assert_eq!(ring.get_slot(&999).unwrap().start, 683);
        assert_eq!(ring.get_slot(&999).unwrap().end, 1024);
        // node3
        assert_eq!(ring.get_slot(&10000).unwrap().start, 342);
        assert_eq!(ring.get_slot(&10000).unwrap().end, 682);
    }

    #[test]
    fn test_batch_remove() {
        let node1 = Node { id: 1 };
        let node2 = Node { id: 2 };
        let node3 = Node { id: 3 };

        let mut ring = Ring::new(DefaultHashBuilder, 1024);

        ring.batch_add(vec![node1, node2, node3], true);

        assert_eq!(ring.len_slots(), 3);
        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.version(), 4); // 3 + 1

        ring.batch_remove(vec![node2], true);

        assert_eq!(ring.len_slots(), 2);
        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.version(), 6); // 4 + 1 + 1(rebalance)

        assert_eq!(ring.get_slot(&1).unwrap().start, 1);
        assert_eq!(ring.get_slot(&1).unwrap().end, 512);
        assert_eq!(ring.get_slot(&10000).unwrap().start, 513);
        assert_eq!(ring.get_slot(&10000).unwrap().end, 1024);

        ring.batch_remove(vec![node3], true);

        assert_eq!(ring.len_slots(), 1);
        assert_eq!(ring.capacity(), 1024);
        assert_eq!(ring.version(), 8); // 6 + 1 + 1(rebalance)

        assert_eq!(ring.get_slot(&1).unwrap().start, 1);
        assert_eq!(ring.get_slot(&1).unwrap().end, 1024);
    }

    /// Test the ring add and remove
    /// 
    /// 
    /// 3 nodes without balance => [500617, 249146, 250236]
    /// ```
    /// let mut node_hit_count = vec![0; 3];
    /// for i in 1..1000000 {
    ///    let slot = ring.get_slot(&i).unwrap();
    ///        node_hit_count[slot.inner().id as usize - 1] += 1;
    /// }
    /// println!("{:?}", node_hit_count);
    /// ```
    #[test]
    fn test_ring_get_slot() {
        let node1 = Node { id: 1 };
        let node2 = Node { id: 2 };
        let node3 = Node { id: 3 };

        let mut ring = Ring::new(DefaultHashBuilder, 1024);

        ring.add(node1, false);
        ring.add(node2, false);
        ring.add(node3, false);

        let slot = ring.get_slot(&1).unwrap();
        assert_eq!(slot.inner().id, 1);

        let slot = ring.get_slot(&999).unwrap();
        assert_eq!(slot.inner().id, 3);

        let slot = ring.get_slot(&10000).unwrap();
        assert_eq!(slot.inner().id, 2);
    }

    #[test]
    fn test_ring_get_replicas() {
        let node1 = Node { id: 1 };
        let node2 = Node { id: 2 };
        let node3 = Node { id: 3 };

        let mut ring = Ring::new(DefaultHashBuilder, 1024);

        println!("{:?}", ring.get_replicas(&1, 2));

        ring.add(node1, false);
        ring.add(node2, false);
        ring.add(node3, false);

        println!("{:?}", ring.get_replicas(&1, 2));

        let slots = ring.get_replicas(&1, 2).unwrap();
        assert_eq!(slots.len(), 2);
        assert_eq!(slots[0].inner().id, 1);
        assert_eq!(slots[1].inner().id, 2);

        let slots = ring.get_replicas(&10000, 2).unwrap();
        assert_eq!(slots.len(), 2);
        assert_eq!(slots[0].inner().id, 2);
        assert_eq!(slots[1].inner().id, 3);
    }
}