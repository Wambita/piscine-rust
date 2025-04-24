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
        let mut prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
        let mut receipt = Vec::new();
    
        let mut i = 0;
        while i + 2 < prices.len() {
            // Skip the cheapest one (i), add i+1 and i+2
            receipt.push((prices[i + 1] * 100.0).round() / 100.0);
            receipt.push((prices[i + 2] * 100.0).round() / 100.0);
            i += 3;
        }
    
        // Add any remaining prices (1 or 2 items) — no discount
        while i < prices.len() {
            receipt.push((prices[i] * 100.0).round() / 100.0);
            i += 1;
        }
    
        receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = receipt.clone();
        receipt
    }
    
}