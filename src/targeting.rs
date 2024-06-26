use {
    crate::{
        quickcheck::{
            Arbitrary,
            Mutate,
            RunResult,
        },
        seedpool::{
            Seed,
            SeedPool,
        },
    },
    std::fmt::Debug,
};

pub fn maximizing_targeting_loop<
    Domain: Clone + Arbitrary + Mutate,
    Codomain,
    Feedback: Clone + Ord + Debug,
>(
    f: fn(Domain) -> Codomain,
    fb: fn(Domain, Codomain) -> Feedback,
) -> Seed<Domain, Feedback> {
    let mut pool: SeedPool<Domain, Feedback> = SeedPool::new();
    let fuel = 100000;

    for i in 1..=fuel {
        if i % 1000 == 0 {
            println!("Iteration: {}", i);
            println!("Pool size: {}", pool.size());
            println!("Best of all time: {:?}", pool.best_of_all_time.clone().unwrap().feedback);
            println!("====================\n");
        }
        let input = if let Some(seed) = pool.pop() {
            Domain::mutate(&seed.input)
        } else {
            Domain::generate()
        };

        let result = f(input.clone());
        let feedback = fb(input.clone(), result);

        if pool.is_empty() || feedback > pool.best().clone().feedback {
            let seed = Seed { input, feedback, energy: 1000 };
            pool.add_seed(seed);
        }
    }

    pool.best_of_all_time.unwrap().clone()
}

pub fn prop_targeting_loop<Domain: Clone + Arbitrary + Mutate, Feedback: Clone + Ord + Debug>(
    f: fn(Domain) -> bool,
    fb: fn(Domain) -> Feedback,
) -> RunResult<Seed<Domain, Feedback>> {
    let mut pool: SeedPool<Domain, Feedback> = SeedPool::new();
    let fuel = 100000;

    for i in 1..=fuel {
        if i % 1000 == 0 {
            println!("Iteration: {}", i);
            println!("Pool size: {}", pool.size());
            println!("Best of all time: {:?}", pool.best_of_all_time.clone().unwrap().feedback);
            println!("====================\n");
        }
        let input = if let Some(seed) = pool.pop() {
            Domain::mutate(&seed.input)
        } else {
            Domain::generate()
        };

        let result = f(input.clone());
        let feedback = fb(input.clone());

        if !result {
            return RunResult {
                passed: i,
                discarded: 0,
                counterexample: Some(Seed { input, feedback, energy: 1000 }),
            };
        }

        if pool.is_empty() || feedback > pool.best().clone().feedback {
            let seed = Seed { input, feedback, energy: 1000 };
            pool.add_seed(seed);
        }
    }

    RunResult { passed: fuel, discarded: 0, counterexample: None }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximizing_targeting_loop() {
        let result =
            maximizing_targeting_loop(|x: Vec<i32>| x.iter().sum(), |_x: Vec<i32>, y: i32| y);

        let avg: i32 = <Vec<i32>>::generate().iter().sum();

        assert!(result.feedback > avg);
    }
}
