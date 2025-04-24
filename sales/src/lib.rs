#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<String, f32>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart{
            items: vec![],
            receipt:vec![],
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product)= s.products.iter().find(|(n, _)| n == &name){
            self.items.push(product.clone());
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap()); // sort ascending

        let mut result: Vec<f32> = vec![];

        for chunk in prices.chunks(3) {
            if chunk.len() == 3 {
                let total: f32 = chunk.iter().sum();
                let discount = chunk.iter().cloned().fold(f32::INFINITY, f32::min);

                let adjusted = chunk
                    .iter()
                    .map(|x| ((x - (discount * x / total)) * 100.0).round() / 100.0)
                    .collect::<Vec<f32>>();
                result.extend(adjusted);
            } else {
                result.extend(chunk.iter().map(|x| (x * 100.0).round() / 100.0));
            }
        }

        result.sort_by(|a, b| a.partial_cmp(b).unwrap()); // sort the receipt
        self.receipt = result.clone();
        result
 
    }
}