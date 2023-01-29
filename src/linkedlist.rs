use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Default)]
pub enum LNode<A> {
    #[default]
    Null,
    Node(A, Box<LNode<A>>),
}

impl<A: Clone + Display + Debug> From<&Vec<A>> for LNode<A> {
    fn from(value: &Vec<A>) -> Self {
        let mut empty = LNode::empty();
        if value.is_empty() {
            return empty;
        }
        value.iter().for_each(|c| empty = empty.insert(c.clone()));
        empty
    }
}

#[allow(dead_code)]
impl<A: Clone + Display + Debug> LNode<A> {
    pub fn empty() -> LNode<A> {
        LNode::Null
    }

    pub fn singleton(val: A) -> LNode<A> {
        LNode::Node(val, Box::new(LNode::Null))
    }

    pub fn map<B: Clone + Display>(&self, f: fn(A) -> B) -> LNode<B> {
        match self {
            LNode::Null => LNode::Null,
            LNode::Node(a, next) => LNode::Node(f(a.clone()), Box::new(next.map(f))),
        }
    }

    pub fn insert(&self, new_value: A) -> LNode<A> {
        match self {
            LNode::Null => LNode::Node(new_value, Box::new(LNode::Null)),
            LNode::Node(val, next) => LNode::Node(val.clone(), Box::new(next.insert(new_value))),
        }
    }
}

impl<A: Clone + Display> ToString for LNode<A> {
    fn to_string(&self) -> String {
        let mut ll = &*self;
        let mut r = String::new();

        loop {
            match ll {
                LNode::Null => {
                    r.push_str("Null");
                    break;
                }
                LNode::Node(val, next) => {
                    r.push_str(format!("Node({})->", val).as_str());
                    ll = next;
                }
            }
        }
        r
    }
}

impl<A: Clone + Display> Clone for LNode<A> {
    fn clone(&self) -> Self {
        match self {
            LNode::Null => LNode::Null,
            LNode::Node(a, next) => LNode::Node(a.clone(), next.clone()),
        }
    }
}
