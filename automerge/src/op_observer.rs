use crate::exid::ExId;
use crate::Prop;
use crate::Value;

/// An observer of operations applied to the document.
pub trait OpObserver {
    /// A new value has been inserted into the given object.
    ///
    /// - `objid`: the object that has been inserted into.
    /// - `index`: the index the new value has been inserted at.
    /// - `tagged_value`: the value that has been inserted and the id of the operation that did the
    /// insert.
    fn insert(&mut self, objid: ExId, index: usize, tagged_value: (Value, ExId));

    /// A new value has been put into the given object.
    ///
    /// - `objid`: the object that has been put into.
    /// - `key`: the key that the value as been put at.
    /// - `tagged_value`: the value that has been put into the object and the id of the operation
    /// that did the put.
    /// - `conflict`: whether this put conflicts with other operations.
    fn put(&mut self, objid: ExId, key: Prop, tagged_value: (Value, ExId), conflict: bool);

    /// A value has beeen deleted.
    ///
    /// - `objid`: the object that has been deleted in.
    /// - `key`: the key of the value that has been deleted.
    fn delete(&mut self, objid: ExId, key: Prop);
}

impl OpObserver for () {
    fn insert(&mut self, _objid: ExId, _index: usize, _tagged_value: (Value, ExId)) {}

    fn put(&mut self, _objid: ExId, _key: Prop, _tagged_value: (Value, ExId), _conflict: bool) {}

    fn delete(&mut self, _objid: ExId, _key: Prop) {}
}

/// Capture operations into a [`Vec`] and store them as patches.
#[derive(Default, Debug, Clone)]
pub struct VecOpObserver {
    patches: Vec<Patch>,
}

impl VecOpObserver {
    /// Take the current list of patches, leaving the internal list empty and ready for new
    /// patches.
    pub fn take_patches(&mut self) -> Vec<Patch> {
        std::mem::take(&mut self.patches)
    }
}

impl OpObserver for VecOpObserver {
    fn insert(&mut self, obj_id: ExId, index: usize, (value, id): (Value, ExId)) {
        self.patches.push(Patch::Insert {
            obj: obj_id,
            index,
            value: (value.into_owned(), id),
        });
    }

    fn put(&mut self, objid: ExId, key: Prop, (value, id): (Value, ExId), conflict: bool) {
        self.patches.push(Patch::Put {
            obj: objid,
            key,
            value: (value.into_owned(), id),
            conflict,
        });
    }

    fn delete(&mut self, objid: ExId, key: Prop) {
        self.patches.push(Patch::Delete { obj: objid, key })
    }
}

/// A notification to the application that something has changed in a document.
#[derive(Debug, Clone, PartialEq)]
pub enum Patch {
    /// Associating a new value with a key in a map, or an existing list element
    Put {
        /// The object that was put into.
        obj: ExId,
        /// The key that the new value was put at.
        key: Prop,
        /// The value that was put, and the id of the operation that put it there.
        value: (Value<'static>, ExId),
        /// Whether this put conflicts with another.
        conflict: bool,
    },
    /// Inserting a new element into a list/text
    Insert {
        /// The object that was inserted into.
        obj: ExId,
        /// The index that the new value was inserted at.
        index: usize,
        /// The value that was inserted, and the id of the operation that inserted it there.
        value: (Value<'static>, ExId),
    },
    /// Deleting an element from a list/text
    Delete {
        /// The object that was deleted from.
        obj: ExId,
        /// The key that was deleted.
        key: Prop,
    },
}
