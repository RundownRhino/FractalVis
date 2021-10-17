use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

use num_traits::{FromPrimitive, Num};

use crate::utils::join;

#[derive(Clone)]
pub(crate) struct Polynomial<T> {
    pub(crate) coeffs: Vec<T>,
}

impl<T> Default for Polynomial<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            coeffs: vec![Default::default()],
        }
    }
}

impl<T> Polynomial<T> {
    pub(crate) fn new(coeffs: Vec<T>) -> Self {
        assert!(!coeffs.is_empty());
        Self { coeffs }
    }
}

impl<T> Polynomial<T> {
    /// Evaluate the polynomial at a value.
    pub(crate) fn evaluate(&self, x: T) -> T
    where
        T: Default + Add<T, Output = T> + Mul<T, Output = T> + Copy,
    {
        // c_0 + x*c_1 + x^2*c_2 + ...
        // Let's store them highest to lowest. Then:
        // ((c_n) * x + c_{n-1}) * x + c_{n-2} ... + c_0
        let mut total = T::default();
        for &coeff in &self.coeffs {
            total = total * x + coeff;
        }
        total
    }
    /// Transform all coefficients of the polynomial via a function
    pub(crate) fn map_coeffs<V, F>(&self, fun: F) -> Polynomial<V>
    where
        F: Fn(T) -> V,
        T: Copy,
    {
        let mut new_coeffs = Vec::with_capacity(self.coeffs.len());
        for &coeff in &self.coeffs {
            new_coeffs.push(fun(coeff));
        }
        Polynomial::new(new_coeffs)
    }
    /// The degree of this polynomial
    pub(crate) fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }
    /// Returns the coeff corresponding to x^i. The c_0 coeff is the last one, for example
    pub(crate) fn coeff_at_deg(&self, i: usize) -> &T {
        &self.coeffs[self.coeffs.len() - i - 1]
    }

    pub(crate) fn from_roots(roots: &[T]) -> Self
    where
        T: Num + Neg<Output = T> + Copy + Add<Output = T> + Mul<Output = T> + Default,
    {
        // n roots -> degree n -> n+1 coeffs
        roots
            .iter()
            .map(|&z| Self {
                coeffs: vec![T::one(), -z],
            })
            .reduce(|a, b| &a * &b)
            .unwrap()
    }
    pub(crate) fn derivative(&self) -> Self
    where
        T: Copy + Default + Num + FromPrimitive,
    {
        let n = self.degree();
        if n == 0 {
            return Default::default();
        }

        let mut new_coeffs = Vec::with_capacity(self.coeffs.len() - 1);
        // Every coeff is multiplied by its degree, from n to 1.
        // That's a total of n coeffs.
        for (i, &coeff) in (1..=n).rev().zip(self.coeffs.iter()) {
            new_coeffs.push(coeff * T::from_usize(i).unwrap());
        }
        Polynomial::new(new_coeffs)
    }
}
impl<T> Display for Polynomial<T>
where
    T: Display + Default + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            join(
                self.coeffs.iter().enumerate().filter_map(|(i, coeff)| {
                    if coeff != &T::default() {
                        let n = self.degree() - i;
                        if n == 0 {
                            Some(format!("{}", coeff))
                        } else {
                            Some(format!("{}x^{}", coeff, self.degree() - i))
                        }
                    } else {
                        None
                    }
                }),
                " + "
            )
            .unwrap()
        )
    }
}

impl<T> Add for &Polynomial<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Polynomial<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let (longer, shorter) = if rhs.coeffs.len() > self.coeffs.len() {
            (rhs, self)
        } else {
            (self, rhs)
        };
        let (n, m) = (longer.coeffs.len(), shorter.coeffs.len());
        let mut result_coeffs = Vec::with_capacity(n);
        let len_diff = n - m;
        result_coeffs.extend_from_slice(&longer.coeffs[..len_diff]);
        for i in len_diff..n {
            result_coeffs.push(longer.coeffs[i] + shorter.coeffs[i - len_diff]);
        }
        Self::Output::new(result_coeffs)
    }
}
impl<T, N> Neg for &Polynomial<T>
where
    T: Neg<Output = N> + Copy,
{
    type Output = Polynomial<N>;

    fn neg(self) -> Self::Output {
        self.map_coeffs(|x| -x)
    }
}
impl<T> Sub for &Polynomial<T>
where
    T: Copy + Add<Output = T> + Neg<Output = T>,
{
    type Output = Polynomial<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

impl<T> Mul for &Polynomial<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default,
{
    type Output = Polynomial<T>;

    fn mul(self, other: Self) -> Self::Output {
        let (n, m) = (self.degree(), other.degree());
        let mut result_coeffs = Vec::<T>::with_capacity(n + m + 1);
        for i in 0..=(n + m) {
            let mut c = T::default();
            // This is calculating the coefficient c_{n+m-i}
            // It's equal to the sum of a_j * b_k where j+k=n+m-i, and j goes from 0 to n, and k from 0 to m.
            // Lowest j: n - i, highest: n+m-i
            // We also need to check that it's from 0 to n.
            let from_j = if i > n { 0 } else { n - i };
            let to_j = if i < m { n } else { n + m - i };
            for j in from_j..=to_j {
                let k = n + m - i - j; // we can see it goes from k to 0, as it should
                c = c + *self.coeff_at_deg(j) * *other.coeff_at_deg(k);
            }
            result_coeffs.push(c);
        }
        Self::Output::new(result_coeffs)
    }
}

impl<T> Mul<T> for &Polynomial<T>
where
    T: Copy + Mul<T, Output = T>,
{
    type Output = Polynomial<T>;

    fn mul(self, k: T) -> Self::Output {
        self.map_coeffs(|x| x * k)
    }
}

impl<T> PartialEq for Polynomial<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}

#[cfg(test)]
mod polynomial_tests {
    use super::*;
    #[test]
    #[allow(clippy::identity_op)]
    fn test_poly_coeffs_evaluation() {
        let poly = Polynomial::new(vec![3, 2, 1, 0]); // that's 0 + x + 2x^2 + 3x^3
        assert!(poly.degree() == 3);
        for (i, &res) in [0, 1, 2, 3].iter().enumerate() {
            assert!(*poly.coeff_at_deg(i) == res);
        }
        for x in [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5] {
            assert!(poly.evaluate(x) == (0 + x + 2 * x * x + 3 * x * x * x));
        }
    }
    #[test]
    fn test_poly_add_sub() {
        let poly1 = Polynomial::new(vec![3, 2, 1]); //  1 + 2x + 3x^2
        let poly2 = Polynomial::new(vec![-2, 4, 6, 8]); //  8 + 6x + 4x^2  -2x^3
        let tot1 = &poly1 + &poly2;
        let tot2 = &poly2 + &poly1;
        for t in [&tot1, &tot2] {
            assert!(t.coeffs == [-2, 7, 8, 9]);
        }
        assert!(tot1 == tot2);
    }

    #[test]
    fn test_poly_mul() {
        let poly1 = Polynomial::new(vec![3, 2, 1]); //  1 + 2x + 3x^2
        let poly2 = Polynomial::new(vec![-2, 4, 6, 8]); //  8 + 6x + 4x^2  -2x^3
        let res1 = &poly1 * &poly2;
        let res2 = &poly2 * &poly1; // 8 + 22x + 40x^2 + 24x^3 + 8x^4 - 6 x^5
        for t in [&res1, &res2] {
            assert!(t.coeffs == [-6, 8, 24, 40, 22, 8]);
        }
        assert!(res1 == res2);
    }
    #[should_panic]
    #[test]
    fn test_no_empty_polys() {
        let _poly = Polynomial::<usize>::new(vec![]);
    }
    #[test]
    fn test_poly_fromroots() {
        let poly = Polynomial::from_roots(&[1, 2, 3]);
        // (x-1)(x-2)(x-3) = x^3 - 6x^2 + 11x - 6
        assert!(poly.coeffs == [1, -6, 11, -6]);
    }
    #[test]
    fn test_poly_derivative() {
        let poly = Polynomial::new(vec![3, 2, 1]); // 3x^2 + 2x + 1
        let der = poly.derivative(); // 6x + 2
        assert!(der.coeffs == [6, 2]);
    }
}
