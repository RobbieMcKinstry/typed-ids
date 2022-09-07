use super::generator::Generator;
use crate::SerialUsize;

pub struct SerialUsizeGenerator<T: 'static> {
    gen: Generator<SerialUsize<T>>,
}

impl<T: 'static> SerialUsizeGenerator<T> {
    pub fn new() -> Self {
        // Create a closure which captures an integer counter.
        // This closure provides the generation logic used
        // by the underlying Generator type.
        let mut counter = 0usize;
        let closure = move || {
            counter += 1;
            SerialUsize::try_from(counter).unwrap()
        };
        // Use this closure for generation logic.
        let gen = Generator::new(closure);
        Self { gen }
    }

    pub async fn next(&self) -> SerialUsize<T> {
        self.gen.next().await
    }
}

impl<T: 'static> Default for SerialUsizeGenerator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SerialUsizeGenerator;
    use crate::SerialUsize;
    use pretty_assertions::assert_eq;
    use tokio::runtime::Runtime;

    #[test]
    fn generate_id() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let generator: SerialUsizeGenerator<()> = Default::default();
            for i in 1..11 {
                let expected: SerialUsize<()> = SerialUsize::try_from(i).unwrap();
                let observed = generator.next().await;
                assert_eq!(expected, observed);
            }
        })
    }
}
