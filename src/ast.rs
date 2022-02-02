
pub struct Problem {

}

pub struct Predicate {

}

pub  struct Action {

}

pub enum AtomValue {
    Unkown(String), 
    Known(String)
}

pub struct Atom {
    pub value: AtomValue,
    pub ty: u32
}


pub struct Domain {
    pub name: String,
    pub predicates: Vec<Predicate>,
    pub actions: Vec<Action>
}

pub enum Define {
    Problem(Box<Problem>), 
    Domain(Box<Domain>),
}
