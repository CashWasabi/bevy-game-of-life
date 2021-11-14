pub mod helpers;

#[cfg(test)]
mod tests {
    use super::*;
    use grid::*;

    #[test]
    fn test_count_neighbours() {
        let test_grid = grid![
            [0, 1, 0]
            [1, 1, 0]
            [1, 1, 1]
        ];

        let neighbours = helpers::count_neighbours(&test_grid, 1, 1);
        assert_eq!(neighbours, 5);
    }
}
