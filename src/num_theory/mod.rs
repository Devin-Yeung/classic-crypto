use log::info;

/// credit: https://github.com/Kikks/extended-euclidean-algorithm/blob/main/src/main.rs
pub fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    // This is the base case: when b = 0,
    //     gcd(a, 0) = a
    // Hence the Euclidean equation becomes:
    //     a(1) + b(0) = a
    if b == 0 {
        return (a, 1, 0);
    }

    // Recursively call the extended Euclidean Algorithm
    let (gcd, x1, y1) = egcd(b, a % b);

    // Compute x and y by working backwards the Euclidean Algorithm
    let x = y1;
    let y = x1 - (a / b) * y1;

    info!("ax + by = ({}) * ({}) + ({}) * ({}) = {}", a, x, b, y, gcd);

    // Return the tuple
    (gcd, x, y)
}

#[cfg(test)]
mod tests {
    use crate::num_theory::egcd;

    #[test]
    pub fn test_egcd() {
        let (gcd, x, y) = egcd(16, 23);
        assert_eq!(gcd, 1);
        assert_eq!(x, -10);
        assert_eq!(y, 7);
    }
}
