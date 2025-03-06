// Copyright 2018 Chris Pearce
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::item::Item;
use fnv::FnvHashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::slice::Iter;

pub struct Itemizer<T> {
    next_item_id: u32,
    item_str_to_id: FnvHashMap<T, Item>,
    item_id_to_str: Vec<T>,
}

impl<T> Itemizer<T>
where
    T: Eq + Hash + Ord + Clone,
{
    pub fn new() -> Itemizer<T> {
        Itemizer {
            next_item_id: 1,
            item_str_to_id: FnvHashMap::default(),
            item_id_to_str: vec![],
        }
    }

    pub fn id_of(&mut self, item: &T) -> Item {
        if let Some(id) = self.item_str_to_id.get(item) {
            return *id;
        }

        let id = self.next_item_id;
        self.next_item_id += 1;

        self.item_str_to_id.insert(item.clone(), Item::with_id(id));

        self.item_id_to_str.push(item.clone());

        assert_eq!(self.item_id_to_str.len(), id as usize);

        Item::with_id(id)
    }

    pub fn id_of_exists(&self, item: &T) -> Option<Item> {
        if let Some(id) = self.item_str_to_id.get(item) {
            return Some(*id);
        } else {
            None
        }
    }
    pub fn value_of(&self, id: &Item) -> &T {
        &self.item_id_to_str[id.as_index() - 1]
    }

    pub fn len(&self) -> usize {
        self.item_id_to_str.len()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.item_id_to_str.iter()
    }
}

impl Debug for Itemizer<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print all existing keys:
        for (key, value) in &self.item_str_to_id {
            writeln!(f, "{}: {}", key, value.as_index())?;
        }
        Ok(())
    }
}
