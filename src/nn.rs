use bevy::reflect::Reflect;
use rand::Rng;

#[derive(Clone, Default, Reflect)]
pub struct Net {
    n_inputs: usize,
    layers: Vec<Layer>,
}

#[derive(Clone, Reflect)]
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
        if inputs.len() != self.n_inputs {
            panic!("Bad input size");
        }
        for i in inputs{
            if !(&0. ..=&1.0).contains(&i){
                dbg!(&i);
                dbg!(&inputs);
                panic!("Incorrect input");
            }
        }

        let mut outputs = Vec::new();
        outputs.push(inputs.clone());
        for (layer_index, layer) in self.layers.iter().enumerate() {
            let layer_results = layer.predict(&outputs[layer_index]);
            outputs.push(layer_results);
        }
        outputs[outputs.len()-1].clone()
    }
    pub fn mutate(&mut self) {
        self.layers.iter_mut().for_each(|l| l.mutate());
    }
}

impl Layer{
    fn new(layer_size: usize, prev_layer_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut nodes: Vec<Vec<f64>> = Vec::with_capacity(layer_size);

        for _ in 0..layer_size {
            let mut node: Vec<f64> = Vec::with_capacity(prev_layer_size);
            for _ in 0..prev_layer_size + 1 {
                let random_weight: f64 = rng.gen_range(-1.0f64..1.0f64);
                node.push(random_weight);
            }
            nodes.push(node);
        }

        Self { nodes }
    }
    fn predict(&self, inputs: &[f64]) -> Vec<f64> {
        let mut layer_results = Vec::new();
        for node in self.nodes.iter() {
            layer_results.push(self.sigmoid(self.dot_prod(node, inputs)));
        }
        layer_results
    }
    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        for n in self.nodes.iter_mut() {
            for val in n.iter_mut() {
                if rng.gen_range(0.0..1.0) >= 5.0 {
                    continue;
                }

                *val += rng.gen_range(-1.5..1.5) as f64;
            }
        }
    }
    fn dot_prod(&self, node: &[f64], values: &[f64]) -> f64 {
        let mut it = node.iter();
        let mut total = *it.next().unwrap();
        for (weight, value) in it.zip(values.iter()) {
            total += weight * value;
        }

        total
    }

    fn sigmoid(&self, y: f64) -> f64 {
        1f64 / (1f64 + (-y).exp())
    }
}