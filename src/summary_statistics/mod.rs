//! Summary statistics (e.g. mean, variance, etc.).
use crate::errors::EmptyInput;
use ndarray::{Data, Dimension};
use num_traits::{Float, FromPrimitive, Zero};
use std::ops::{Add, Div};

/// Extension trait for `ArrayBase` providing methods
/// to compute several summary statistics (e.g. mean, variance, etc.).
pub trait SummaryStatisticsExt<A, S, D>
where
    S: Data<Elem = A>,
    D: Dimension,
{
    /// Returns the [`arithmetic mean`] x̅ of all elements in the array:
    ///
    /// ```text
    ///     1   n
    /// x̅ = ―   ∑ xᵢ
    ///     n  i=1
    /// ```
    ///
    /// If the array is empty, `Err(EmptyInput)` is returned.
    ///
    /// **Panics** if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// [`arithmetic mean`]: https://en.wikipedia.org/wiki/Arithmetic_mean
    fn mean(&self) -> Result<A, EmptyInput>
    where
        A: Clone + FromPrimitive + Add<Output = A> + Div<Output = A> + Zero;

    /// Returns the [`harmonic mean`] `HM(X)` of all elements in the array:
    ///
    /// ```text
    ///           ⎛ n     ⎞⁻¹
    /// HM(X) = n ⎜ ∑ xᵢ⁻¹⎟
    ///           ⎝i=1    ⎠
    /// ```
    ///
    /// If the array is empty, `Err(EmptyInput)` is returned.
    ///
    /// **Panics** if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// [`harmonic mean`]: https://en.wikipedia.org/wiki/Harmonic_mean
    fn harmonic_mean(&self) -> Result<A, EmptyInput>
    where
        A: Float + FromPrimitive;

    /// Returns the [`geometric mean`] `GM(X)` of all elements in the array:
    ///
    /// ```text
    ///         ⎛ n   ⎞¹⁄ₙ
    /// GM(X) = ⎜ ∏ xᵢ⎟
    ///         ⎝i=1  ⎠
    /// ```
    ///
    /// If the array is empty, `Err(EmptyInput)` is returned.
    ///
    /// **Panics** if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// [`geometric mean`]: https://en.wikipedia.org/wiki/Geometric_mean
    fn geometric_mean(&self) -> Result<A, EmptyInput>
    where
        A: Float + FromPrimitive;

    /// Returns the [kurtosis] `Kurt[X]` of all elements in the array:
    ///
    /// ```text
    /// Kurt[X] = μ₄ / σ⁴
    /// ```
    ///
    /// where μ₄ is the fourth central moment and σ is the standard deviation of
    /// the elements in the array.
    ///
    /// This is sometimes referred to as _Pearson's kurtosis_. Fisher's kurtosis can be
    /// computed by subtracting 3 from Pearson's kurtosis.
    ///
    /// If the array is empty, `Err(EmptyInput)` is returned.
    ///
    /// **Panics** if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// [kurtosis]: https://en.wikipedia.org/wiki/Kurtosis
    fn kurtosis(&self) -> Result<A, EmptyInput>
    where
        A: Float + FromPrimitive;

    /// Returns the [Pearson's moment coefficient of skewness] γ₁ of all elements in the array:
    ///
    /// ```text
    /// γ₁ = μ₃ / σ³
    /// ```
    ///
    /// where μ₃ is the third central moment and σ is the standard deviation of
    /// the elements in the array.
    ///
    /// If the array is empty, `Err(EmptyInput)` is returned.
    ///
    /// **Panics** if `A::from_usize()` fails to convert the number of elements in the array.
    ///
    /// [Pearson's moment coefficient of skewness]: https://en.wikipedia.org/wiki/Skewness
    fn skewness(&self) -> Result<A, EmptyInput>
    where
        A: Float + FromPrimitive;

    /// Returns the *p*-th [central moment] of all elements in the array, μₚ:
    ///
    /// ```text
    ///      1  n
    /// μₚ = ―  ∑ (xᵢ-x̅)ᵖ
    ///      n i=1
    /// ```
    ///
    /// If the array is empty, `Err(EmptyInput)` is returned.
    ///
    /// The *p*-th central moment is computed using a corrected two-pass algorithm (see Section 3.5
    /// in [Pébay et al., 2016]). Complexity is *O(np)* when *n >> p*, *p > 1*.
    ///
    /// **Panics** if `A::from_usize()` fails to convert the number of elements
    /// in the array or if `order` overflows `i32`.
    ///
    /// [central moment]: https://en.wikipedia.org/wiki/Central_moment
    /// [Pébay et al., 2016]: https://www.osti.gov/pages/servlets/purl/1427275
    fn central_moment(&self, order: u16) -> Result<A, EmptyInput>
    where
        A: Float + FromPrimitive;

    /// Returns the first *p* [central moments] of all elements in the array, see [central moment]
    /// for more details.
    ///
    /// If the array is empty, `Err(EmptyInput)` is returned.
    ///
    /// This method reuses the intermediate steps for the *k*-th moment to compute the *(k+1)*-th,
    /// being thus more efficient than repeated calls to [central moment] if the computation
    /// of central moments of multiple orders is required.
    ///
    /// **Panics** if `A::from_usize()` fails to convert the number of elements
    /// in the array or if `order` overflows `i32`.
    ///
    /// [central moments]: https://en.wikipedia.org/wiki/Central_moment
    /// [central moment]: #tymethod.central_moment
    fn central_moments(&self, order: u16) -> Result<Vec<A>, EmptyInput>
    where
        A: Float + FromPrimitive;

    private_decl! {}
}

mod means;
