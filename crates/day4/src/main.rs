use direction::{Direction, OrdinalDirection};
use utils::*;

fn part1(input: &str) -> usize {
    let grid = grid::Grid::char_grid(input);
    grid.iter()
        .filter_map(|(pos, c)| {
            (*c == 'X').then(|| {
                OrdinalDirection::all()
                    .iter()
                    .filter(|direction| {
                        let m = direction.add_unsigned(&pos, 1);
                        let a = direction.add_unsigned(&pos, 2);
                        let s = direction.add_unsigned(&pos, 3);
                        grid.get_virtual(m).is_some_and(|&c| c == 'M')
                            && grid.get_virtual(a).is_some_and(|&c| c == 'A')
                            && grid.get_virtual(s).is_some_and(|&c| c == 'S')
                    })
                    .count()
            })
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = grid::Grid::char_grid(input);
    grid.iter()
        .filter_map(|(pos, &c)| {
            let nw = *grid.get_virtual(OrdinalDirection::NorthWest.add_unsigned(&pos, 1))?;
            let se = *grid.get_virtual(OrdinalDirection::SouthEast.add_unsigned(&pos, 1))?;
            let ne = *grid.get_virtual(OrdinalDirection::NorthEast.add_unsigned(&pos, 1))?;
            let sw = *grid.get_virtual(OrdinalDirection::SouthWest.add_unsigned(&pos, 1))?;
            (c == 'A'
                && matches!((nw, se), ('M', 'S') | ('S', 'M'))
                && matches!((ne, sw), ('M', 'S') | ('S', 'M')))
            .then_some(())
        })
        .count()
}

fn main() {
    part1_test!(18);
    part1_answer!(2358);
    part2_test!(9);
    part2_answer!(1737);
}
