#[license = "MIT"];
extern mod thrust;
use thrust::is_even;

#[test]
fn test_is_even() {
    assert!(is_even(0));
    assert!(!is_even(1));
    assert!(is_even(2));
}

