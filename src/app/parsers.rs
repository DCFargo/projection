use crate::app::structures::arguments::Arguments;
// Todo, handle errors
pub(crate) fn arg_parser(inputs: Vec<String>) -> Arguments {
    let mut op = Vec::new();
    let mut ac = Vec::new();
    for input in inputs.iter() {
        if input.as_bytes()[0] == '-' as u8 {
            op.push(input.clone());
        } else {
            ac.push(input.clone());
        }
    }
    Arguments {
        options: op,
        actions: ac,
    }
}
