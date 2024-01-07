use crate::structure::Tamogachi;

const INITIAL_COMMADS_TO_LEVEL_UP: f64 = 30.0;
const BASE_EXPONENTIAL: f64 = 1.1;

fn return_commands_to_level_up(next_level: &i32) -> u64 {
    // This fucntion assumes this result will fit into a u64
    // Its kinda stupid that i'm even using this large of a number
    // but who cares. Basically, using a u64 allows me to fit a super large
    // number into this. according to my calculations the max level before overflow
    // is 279 which requires 1 trillion comamnds to level up. The sum of all the
    // levels before it, so LVL 1-279 = 12 trillion. I might change the level up
    // formula in the future but this works for now.
    let result = INITIAL_COMMADS_TO_LEVEL_UP * BASE_EXPONENTIAL.powi(*next_level);
    return result.round() as u64;
}

pub fn level_up(obj: &mut Tamogachi) {
    // if commands_to_level_up == 1
    // increment level,
    // calculate new commands_to_level_up using a formula
    // maybe there should be a "easy" formula for levels < 50
    // to level up easy and a "hard" formula for levels > 50
    //
    // if not, decrement commands_to_level_up by one

    if obj.commands_to_level_up == 1 {
        obj.level += 1;
        obj.commands_to_level_up = return_commands_to_level_up(&obj.level);
    } else {
        obj.commands_to_level_up -= 1;
    }
}
