mod math;

#[allow(dead_code)]
fn activation_func(val: f32) -> f32 { // leaky reLU (recified linear somethingorother)
    // if val > 0.0 {
    //     val
    // } else {
    //     0.1 * val // if it wasn't "leaky" this would be 0
    // }
    val
}

type Vector = Vec<f32>;
type Matrix = Vec<Vector>;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Layer {
    /* size -> the number to nodes in the layer
     * nodes -> Vec<f32>, the numerical value of the nodes (only used in the input layer)
     * wieghts -> Option<Vec<Vec<f32>>>, weighting matrix, same # of cols as previous, same number of rows as `size`
     * bias_weight -> f32, each layer has a bias weight (unused in the input layer)
     * previous -> except for the input layer, each layer owns the previous one, which has to be heap allocated (Boxed)
     */
    size: usize,
    nodes: Vector,
    weights: Option<Matrix>,
    bias_weight: f32,
    previous: Option<Box<Layer>>
}

#[allow(dead_code, unused_variables)]
impl Layer {
    fn new_input_layer(size: usize) -> Layer {
        Layer {
            size: size,
            nodes: vec![1.0; size],
            weights: None,
            bias_weight: 1.0,
            previous: None
        }
    }

    fn new_ontop_of(prev_layer: Layer, size: usize) -> Layer {
        Layer {
            size: size,
            nodes: vec![1.0; size],
            weights: Some(vec![ vec![1.0; prev_layer.size]; size]), // same number of cols as previous layer's size, number of rows is from this layer
            bias_weight: 1.0,
            previous: Some(Box::from(prev_layer)) // we now own the previous layer through this one
        }
    }

    // RECURSIVE FUNCTION
    // each node's value is obtained by dotting a weight vector with a vector of all the nodes before
    // the weight vector is unique to this node
    // 'a' is the name given to the value of a node before it goes through the activation function
    fn node_a(&self, node_number: usize) -> math::Result<f32> {
        if self.is_input_layer() {
            // if we're on the first layer, then the value of each node is simply part of the input
            // to the net
            return Ok(self.nodes[node_number]);
        } else {
            /* if we're not in the first layer, then we have to calculate the value of each node from
             * the value of all the previous nodes and this layer's weights and bias. This is recursive.
             */
            let weights = &self.weights.clone().unwrap()[node_number]; // pull out the right row from the matrix
            let prev_layer = self.previous.clone().unwrap();
            let mut prev_nodes = Vector::new();

            for i in 0 .. prev_layer.size {
                prev_nodes.push(prev_layer.node_val(i)) // calculate the value of each previous node
            }

            // dot the previous node vector with the weight vector
            let a = math::dot(weights, &prev_nodes).unwrap() + 1.0*self.bias_weight;
            Ok(a)
        }
    }

    // each node's value is the dot product of the corresponding weight vector, and the previous nodes,
    // which is then put through some kind of activation function
    fn node_val(&self, node_number: usize) -> f32 {
        let a = self.node_a(node_number).unwrap();
        activation_func(a)
    }

    fn is_input_layer(&self) -> bool {
        // no weights associated with the input layer
        self.weights.is_none()
    }
}

#[allow(unused_variables, dead_code, unused_mut)]
fn main() {
    let mut input_layer  = Layer::new_input_layer(3);
    input_layer.nodes = vec![1.0, 1.0, 1.0];

    let mut middle_layer = Layer::new_ontop_of(input_layer, 2);
    let mut output_layer = Layer::new_ontop_of(middle_layer, 1);

    // println!("{:#?}", output_layer);
    println!("{:?}", output_layer.node_val(0));

}
