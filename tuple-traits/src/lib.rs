struct Person {}

struct Bank {
    conversion_fee: u32,
}

trait Converter {
    type Error;
    fn convert(&self, amount: u32) -> Result<u32, Self::Error>;
}

impl Converter for Person {
    type Error = ();

    // Method returns an overview of the movie
    fn convert(&self, amount: u32) -> Result<u32, Self::Error> {
        match amount {
            1..=50 => Ok(amount),
            _ => Err(())
        }
    }
}

impl Converter for Bank {
    type Error = ();

    // Method returns an overview of the movie
    fn convert(&self, amount: u32) -> Result<u32, Self::Error> {
        Ok(amount - self.conversion_fee)
    }
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl Converter for Tuple {
    type Error = ();
    fn convert(&self, amount: u32) -> Result<u32, Self::Error> {
        for_tuples!( #(
            if let Ok(r) = Tuple.convert(amount) {
                return Ok(r); // early return on success
            }
	    )* );
        Err(())
    }
}

type MultiConverter = (Person, Bank);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let person = Person {};
        let bank = Bank { conversion_fee: 3 };

        let m: MultiConverter = (person, bank);
        println!("Tuple (Person, Bank) converted 10€ into {}€.", m.convert(10).unwrap());
        println!("Tuple (Person, Bank) converted 60€ into {}€.", m.convert(60).unwrap());
    }
}
