mod math;

const NUM_INPUTS:      usize = 3;
const NUM_INNER_NODES: usize = 3;

#[allow(unused_variables, dead_code)]
fn main() {
    let input_layer = vec![1.0; NUM_INPUTS];

    let mut inner_layer   = vec![1.0; NUM_INNER_NODES];
    let mut inner_weights = vec![1.0; NUM_INPUTS];
    let mut inner_bias    = 1.0;

    let mut output_layer   = vec![1.0; 1];
    let mut output_weights = vec![1.0; NUM_INNER_NODES];
    let mut output_bias    = 1.0;

    for i in 0 .. inner_layer.len() {
        inner_layer[i] = activation_func(
            math::dot(&input_layer, &inner_weights).unwrap() + inner_bias
        )
    }

    for i in 0 .. output_layer.len() {
        output_layer[i] = activation_func(
            math::dot(&inner_layer, &output_weights).unwrap() + output_bias
        )
    }

    println!("{:?}", output_layer);
}


#[allow(dead_code)]
fn activation_func(val: f32) -> f32 {
    if val > 0.0 {
        val
    } else {
        0.1 * val
    }
}
