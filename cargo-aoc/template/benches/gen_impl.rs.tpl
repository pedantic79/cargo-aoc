
    {
        let input = input_day{DAY}.clone();
        {GEN_NAME}.bench_function("{NAME}", |b| b.iter(|| Factory::{RUNNER_NAME}(input.clone()).unwrap()));
    }
