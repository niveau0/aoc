#[derive(Debug, Clone)]
struct Input((i16, i16), (i16, i16));

// v = 7,2
// S = 0,-3
// p1 = 7, -1
// p2 = 13, 0
// p3 = 18, 0
// p4 = 22, -1
// p5 = 25, -3

// -3 = 0 + -3
// -1 = 7 + -6
// 0 = 13 + -13
// 0 = 18 + -18
// y = tan(α) * x − (g / 2 * v² * cos²α) * x²
fn main() {
    let input = Input((179, -109), (201, -63));

    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    println!("## Part 1");
    // println!("Result: {}", packet.version_sum());
}

fn part2(input: &Input) {
    println!("## Part 2");
    // println!("Result: {}", packet.calculate());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {}
}
