enum PaymentType {
    DigitalToken,
    Cash,
}

struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}

struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32,
}

struct BuyerGroup {
    members: Vec<Buyer>,
}

impl BuyerGroup {
    fn add_member(&mut self, member: Buyer) {
        self.members.push(member);
    }
}

fn main() {
    let buyer_1 = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00,
    };

    let mut buyers = BuyerGroup {
        members: Vec::new(),
    };

    buyers.add_member(buyer_1);
}
