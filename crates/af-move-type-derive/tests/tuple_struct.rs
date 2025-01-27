use af_move_type::MoveStruct;
use serde::{Deserialize, Serialize};

#[derive(MoveStruct, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[move_(nameless)]
pub struct Otw(bool);

impl std::fmt::Display for Otw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OTW")
    }
}

fn main() {}
