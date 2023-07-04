
    let mut {GEN_NAME} = c.benchmark_group("Generator Day{DAY}");

    {IMPLS}

    {GEN_NAME}.finish();
