use crate::generator::Generator;
use crate::SerialU64;

pub struct SerialU64Generator<T: 'static> {
    gen: Generator<SerialU64<T>>,
}

impl<T: 'static> SerialU64Generator<T> {
    pub fn new() -> Self {
        // Create a closure which captures an integer counter.
        // This closure provides the generation logic used
        // by the underlying Generator type.
        let mut counter = 0u64;
        let closure = move || {
            counter += 1;
            SerialU64::try_from(counter).unwrap()
        };
        // Use this closure for generation logic.
        let gen = Generator::new(closure);
        Self { gen }
    }

    pub async fn next(&self) -> SerialU64<T> {
        self.gen.next().await
    }

    pub async fn next_sync(&self) -> SerialU64<T> {
        self.gen.next_sync()
    }
}

impl<T: 'static> Default for SerialU64Generator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SerialU64Generator;
    use crate::SerialU64;
    use pretty_assertions::assert_eq;
    use tokio::runtime::Runtime;

    #[test]
    fn generate_id() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let generator: SerialU64Generator<()> = Default::default();
            for i in 1..11 {
                let expected: SerialU64<()> = SerialU64::try_from(i).unwrap();
                let observed = generator.next().await;
                assert_eq!(expected, observed);
            }
        })
    }
}
