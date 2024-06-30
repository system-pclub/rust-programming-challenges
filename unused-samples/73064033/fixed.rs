use std::marker::PhantomData;

type NodeId = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeContents<Value> {
    Leaf(Vec<Value>),
    Internal(Vec<NodeId>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<Key, Value> {
    pub keys: Vec<Key>,
    pub contents: NodeContents<Value>,
}

pub struct NodePool<Key, Value> {
    _p: PhantomData<(Key, Value)>,
}

impl<Key, Value> NodePool<Key, Value> {
    fn lookup(&self, _child_id: NodeId) -> Node<Key, Value> {
        todo!()
    }
}

pub enum ValueHolder<'a, Value> {
    Owned(Value),
    Borrowed(&'a Value),
}

impl<'a, Value> std::ops::Deref for ValueHolder<'a, Value> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        match self {
            ValueHolder::Owned(val) => val,
            ValueHolder::Borrowed(val) => val,
        }
    }
}

impl<Key, Value> Node<Key, Value>
where
    Key: Ord,
{
    pub fn lookup_value(
        &self,
        key: &Key,
        pool: &NodePool<Key, Value>,
    ) -> Option<ValueHolder<Value>> {
        match &self.contents {
            NodeContents::Leaf(values) => {
                // Simple base case. No problems here.
                self.keys
                    .binary_search(key)
                    .map(|index| ValueHolder::Borrowed(&values[index]))
                    .ok()
            }
            NodeContents::Internal(children) => {
                let index = self
                    .keys
                    .binary_search(key)
                    .map(|index| index + 1)
                    .unwrap_or_else(|index| index);
                let child_id = &children[index];

                // Here the borrow checker gets mad:
                let child = pool.lookup(*child_id); // NodePool::lookup(&self, NodeId) -> Node<K, V>
                child
                    .into_lookup_value(key, pool)
                    .map(|val| ValueHolder::Owned(val))
            }
        }
    }

    pub fn into_lookup_value(self, key: &Key, pool: &NodePool<Key, Value>) -> Option<Value> {
        match self.contents {
            NodeContents::Leaf(mut values) => {
                // Simple base case. No problems here.
                self.keys
                    .binary_search(key)
                    .map(|index| values.swap_remove(index))
                    .ok()
            }
            NodeContents::Internal(children) => {
                let index = self
                    .keys
                    .binary_search(key)
                    .map(|index| index + 1)
                    .unwrap_or_else(|index| index);
                let child_id = &children[index];

                // Here the borrow checker gets mad:
                let child = pool.lookup(*child_id); // NodePool::lookup(&self, NodeId) -> Node<K, V>
                child.into_lookup_value(key, pool)
            }
        }
    }
}

fn main() {}
