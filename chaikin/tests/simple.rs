#[cfg(test)]
mod tests {
    use chaikin::chaikin;

    #[test]
    fn test_chaikin() {
        let points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let result = chaikin(&points);
        assert_eq!(
            result,
            vec![
                (0.0, 0.0),
                (0.25, 0.25),
                (0.75, 0.75),
                (1.25, 0.75),
                (1.75, 0.25),
                (2.0, 0.0)
            ]
        );
    }
}
