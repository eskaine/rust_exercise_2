#[derive(PartialEq, Eq)]
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

    fn find_buyer(&self, payment_type: PaymentType) -> i32 {
        for (i, buyer) in self.members.iter().enumerate() {
            if buyer.payment_type == payment_type {
                return i as i32;
            }
        }

        return -1;
    }

    fn buy(&mut self, buyer_index: i32, seller: &mut Seller) {
        let mut buyer = &mut self.members[buyer_index as usize];

        while buyer.balance >= seller.price {
            buyer.balance -= seller.price;
            seller.balance += seller.price;

            println!("Seller Balance: {}", seller.balance);
            println!("Buyer Balance: {}", buyer.balance);
        }
    }
}

fn main() {
    let buyer_1 = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00,
    };

    let buyer_2 = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.00,
    };

    let mut buyers = BuyerGroup {
        members: Vec::new(),
    };

    buyers.add_member(buyer_1);
    buyers.add_member(buyer_2);

    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };

    let index = buyers.find_buyer(PaymentType::Cash);
    buyers.buy(index, &mut seller);
}
