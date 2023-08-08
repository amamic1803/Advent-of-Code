fn main() {
    let input = include_str!("../../../input.txt");
    let mut cubes: Vec<[i64; 3]> = vec![];
    for line in input.trim().split('\n') {
        let mut cube: [i64; 3] = [0; 3];
        for (location, coordinate) in line.split(',').enumerate() {
            cube[location] = coordinate.parse::<i64>().unwrap();
        }
        cubes.push(cube);
    }
    let mut result: i64 = 6 * (cubes.len() as i64);

    for x in 0..cubes.len() {
        for y in (x + 1)..cubes.len() {
            for z in 0..3 {
                if (cubes[x][(z + 1) % 3] == cubes[y][(z + 1) % 3]) && (cubes[x][(z + 2) % 3] == cubes[y][(z + 2) % 3]) && ((cubes[x][z] - cubes[y][z]).abs() == 1) {
                    result -= 2;
                }
            }
        }
    }
    println!("{}", result);

    for x in 0..cubes.len() {

        for y in 0..3 {

            for z in [-1, 1] {
                let mut possible_cube = cubes[x];
                possible_cube[y] += z;

                if !cubes.contains(&possible_cube) {
                    let mut pocket_of_air: bool = true;
                    'outer_loop: for a in 0..3 {
                        for b in [-1, 1] {
                            let mut curr_check_cube = possible_cube;
                            curr_check_cube[a] += b;

                            if !cubes.contains(&curr_check_cube) {
                                pocket_of_air = false;
                                break 'outer_loop;
                            }

                        }
                    }
                    if pocket_of_air {
                        result -= 6;
                    }
                }

            }

        }

    }
    println!("{}", result);
}
