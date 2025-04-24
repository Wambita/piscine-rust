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
    pub items: Vec<(String, f32)>,
}



impl Cart {
    pub fn new() -> Self {
        Self {
            items: vec![],
        }
    }

    pub fn insert_item(&mut self, _store: &Store, item: String) {
        if let Some((_name, price)) = _store.products.iter().find(|(name, _)| name == &item) {
            self.items.push((item.clone(), *price));
        }
    }

    pub fn generate_receipt(&self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut receipt = vec![];
        for chunk in prices.chunks(3) {
            if chunk.len() == 3 {
                let total: f32 = chunk.iter().sum();
                let discount = *chunk.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                let adjusted = chunk
                    .iter()
                    .map(|x| ((x - (discount * x / total)) * 100.0).round() / 100.0)
                    .collect::<Vec<f32>>();
                receipt.extend(adjusted);
            } else {
                receipt.extend_from_slice(chunk);
            }
        }
        receipt
    }
}
