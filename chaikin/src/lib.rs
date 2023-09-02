/// Applies the [Chaikin's algorithm](http://graphics.cs.ucdavis.edu/education/CAGDNotes/Chaikins-Algorithm/Chaikins-Algorithm.html)
/// to a given set of points to smooth out a curve.
///
/// # Arguments
///
/// * `points`: A vector of tuples representing the points of the curve.
///
/// # Returns
///
/// A new vector of tuples representing the smoothed curve.
///
/// # Examples
///
/// ```
/// use chaikin::chaikin;
///
/// let points = vec![
///     (0.0, 0.0),
///     (1.0, 1.0),
///     (2.0, 0.0),
/// ];
///
/// let smoothed_points = chaikin(&points);
///
/// assert_eq!(
///     smoothed_points,
///     vec![
///         (0.0, 0.0),
///         (0.25, 0.25),
///         (0.75, 0.75),
///         (1.25, 0.75),
///         (1.75, 0.25),
///         (2.0, 0.0),
///     ]
/// );
/// ```
pub fn chaikin(points: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut new: Vec<(f32, f32)> = vec![(0.0, 0.0); points.len() * 2];

    new[0] = points[0];
    let len = new.len();
    new[len - 1] = points[points.len() - 1];

    for i in 0..(points.len() - 1) {
        let q = (
            0.75 * points[i].0 + 0.25 * points[i + 1].0,
            0.75 * points[i].1 + 0.25 * points[i + 1].1,
        );
        let r = (
            0.25 * points[i].0 + 0.75 * points[i + 1].0,
            0.25 * points[i].1 + 0.75 * points[i + 1].1,
        );

        new[i * 2 + 1] = q;
        new[i * 2 + 2] = r;
    }

    new
}

/// A shorthand for applying Chaikin's algorithm `n` times
///
/// See [`chaikin`](fn.chaikin) for more information.
pub fn chaikin_n(points: &[(f32, f32)], n: usize) -> Vec<(f32, f32)> {
    let mut new = points.to_vec();

    for _ in 0..n {
        new = chaikin(&new);
    }

    new
}
