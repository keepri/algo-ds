pub fn two_crystal_balls(breaks: &Vec<bool>, logger: bool) -> Option<usize> {
    let jump_amount = (breaks.len() as f64).sqrt().floor() as usize;
    let mut found: Option<usize> = None;

    let mut jump_index = jump_amount;
    for i in (jump_index..breaks.len()).step_by(jump_amount) {
        if breaks[i] {
            jump_index = i;
            break;
        }
    }

    jump_index -= jump_amount;

    for j in jump_index..=breaks.len() {
        if breaks[j] {
            found = Some(j);
            break;
        }
    }

    if logger == true {
        println!(
            "two crystal balls: {}",
            match found {
                Some(index) => format!("breaking point at index {index}"),
                None => format!("no breaking point found"),
            }
        );
    }

    return found;
}
