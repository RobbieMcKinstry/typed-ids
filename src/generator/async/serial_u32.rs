use super::generator::Generator;
use crate::SerialU32;

pub struct SerialU32Generator<T: 'static> {
    gen: Generator<SerialU32<T>>,
}

impl<T: 'static> SerialU32Generator<T> {
    pub fn new() -> Self {
        // Create a closure which captures an integer counter.
        // This closure provides the generation logic used
        // by the underlying Generator type.
        let mut counter = 0u32;
        let closure = move || {
            counter += 1;
            SerialU32::try_from(counter).unwrap()
        };
        // Use this closure for generation logic.
        let gen = Generator::new(closure);
        Self { gen }
    }

    pub async fn next(&self) -> SerialU32<T> {
        self.gen.next().await
    }
}

impl<T: 'static> Default for SerialU32Generator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SerialU32Generator;
    use crate::SerialU32;
    use pretty_assertions::assert_eq;
    use tokio::runtime::Runtime;

    #[test]
    fn generate_id() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let generator: SerialU32Generator<()> = Default::default();
            for i in 1..11 {
                let expected: SerialU32<()> = SerialU32::try_from(i).unwrap();
                let observed = generator.next().await;
                assert_eq!(expected, observed);
            }
        })
    }
}
