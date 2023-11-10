use rand::Rng;

#[derive(Clone)]
pub struct Net {
    n_inputs: usize,
    layers: Vec<Layer>,
}

#[derive(Clone)]
struct Layer {
    nodes: Vec<Vec<f64>>,
}

impl Net {
    pub fn new(layer_sizes: Vec<usize>) -> Self {
        if layer_sizes.len() < 2 {
            panic!("Need at least 2 layers");
        }
        for &size in layer_sizes.iter() {
            if size < 1 {
                panic!("Empty layers not allowed");
            }
        }

        let mut layers = Vec::new();
        let first_layer_size = *layer_sizes.first().unwrap();
        let mut prev_layer_size = first_layer_size;

        for &layer_size in layer_sizes[1..].iter() {
            layers.push(Layer::new(layer_size, prev_layer_size));
            prev_layer_size = layer_size;
        }

        Self {
            layers,
            n_inputs: first_layer_size,
        }
    }
    pub fn decide(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        vec![rng.gen::<f64>(),rng.gen::<f64>(),rng.gen::<f64>(),rng.gen::<f64>(),rng.gen::<f64>()]
    }
}

impl Layer{
    fn new(layer_size: usize, prev_layer_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut nodes: Vec<Vec<f64>> = Vec::new();

        for _ in 0..layer_size {
            let mut node: Vec<f64> = Vec::new();
            for _ in 0..prev_layer_size + 1 {
                let random_weight: f64 = rng.gen_range(-1.0f64..1.0f64);
                node.push(random_weight);
            }
            nodes.push(node);
        }

        Self { nodes }
    }
}