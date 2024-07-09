const SIZE: usize = 20; // TODO: make generic

#[derive(Default)]
pub struct Histogram {
    columns: [u16; SIZE],
    max_amount: u16,
}

impl Histogram {
    pub fn store(&mut self, value: f32) {
        assert!(value >= 0.0);
        assert!(value <= 1.0);

        let i = (value * SIZE as f32).round() as usize % SIZE;
        let mut amount = &mut self.columns[i];
        *amount += 1;

        if *amount > self.max_amount {
            self.max_amount = *amount;
        }
    }

    pub fn print(&self) {
        println!();
        let max = self.max_amount as f32;

        for percent in (1..=20).rev() {
            let p = percent as f32 / 20.0; // TODO: simplify

            print!("{:.2} ", p);

            for amount in self.columns {
                let amount_normalized = amount as f32 / max;

                if amount_normalized >= p {
                    print!("W ");
                } else if percent == 1 {
                    print!("_ ");
                } else {
                    print!("  ");
                }
            }

            println!("|")
        }
    }
}
