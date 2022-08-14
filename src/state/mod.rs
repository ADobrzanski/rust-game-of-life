pub struct GameMatrix<const W: usize, const H: usize> (pub [[bool; H]; W]);

impl<const W: usize, const H: usize> GameMatrix<W, H> {
    pub fn new() -> GameMatrix<W, H> {
        GameMatrix([[false; H]; W])
    }

    pub fn width(&self) -> usize {
        W
    }

    pub fn height(&self) -> usize {
        H
    }

    pub fn is_cell_alive(&self, x: usize, y: usize) -> bool {
        let GameMatrix(matrix) = self;

        if let Some(column) = matrix.get(x) {
            if let Some(cell_is_alive) = column.get(y) {
                return *cell_is_alive;
            }
        } 

        false
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = ((usize, usize), &bool)> {
        let GameMatrix(matrix) = self;
        matrix
            .iter().enumerate()
            .flat_map(|(x, col)| col
                .iter().enumerate()
                .map(move |(y, cell)| ((x, y), cell))
            )
    }
}
