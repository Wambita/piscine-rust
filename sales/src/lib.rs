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
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push((product.0.clone(), product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.items.iter().map(|(_, price)| *price).collect::<Vec<f32>>();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
        let mut receipt = Vec::new();
    
        for chunk in prices.chunks(3) {
            if chunk.len() == 3 {
                let mut discounted = chunk.to_vec();
                let min = *chunk.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                let total: f32 = chunk.iter().sum();
    
                // Calculate proportional reduction so the total discount equals `min`
                let factor = min / total;
                for price in chunk {
                    let new_price = ((price - (price * factor)) * 100.0).round() / 100.0;
                    receipt.push(new_price);
                }
            } else {
                // No discount
                receipt.extend(chunk.iter().copied());
            }
        }
    
        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = receipt.clone();
        receipt
    }
    
}