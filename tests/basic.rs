use hyperloglog::HyperLogLog;

#[test]
fn test_estimation_accuracy() {
    let mut hll = HyperLogLog::new();
    for i in 0..10_000 {
        hll.add(&i);
    }

    let estimate = hll.estimate();
    println!("Estimate: {}", estimate);
    assert!(
        (9500.0..10500.0).contains(&estimate),
        "Estimate {} is out of expected range",
        estimate
    );
}
