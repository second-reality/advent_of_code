struct State {
    x: i64,
    y: i64,
    x_velocity: i64,
    y_velocity: i64,
    max_y: i64,
}

struct Target {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl State {
    fn step(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;
        self.max_y = i64::max(self.y, self.max_y);

        match self.x_velocity.cmp(&0) {
            std::cmp::Ordering::Less => self.x_velocity += 1,
            std::cmp::Ordering::Greater => self.x_velocity -= 1,
            std::cmp::Ordering::Equal => (),
        }
        self.y_velocity -= 1;
    }
}

fn highest_altitude(x_init: i64, y_init: i64, target: &Target) -> (i64, bool) {
    let mut state = State {
        x: 0,
        y: 0,
        x_velocity: x_init,
        y_velocity: y_init,
        max_y: i64::MIN,
    };

    while state.x <= target.x_max && state.y >= target.y_min {
        if state.x >= target.x_min
            && state.x <= target.x_max
            && state.y >= target.y_min
            && state.y <= target.y_max
        {
            return (state.max_y, true);
        }
        state.step();
    }

    (i64::MIN, false)
}

fn part1(target: &Target) -> (i64, i64, i64) {
    let mut launches: Vec<(i64, i64, i64)> = vec![];

    for x in -1000..1000 {
        for y in -1000..1000 {
            let (alt, found) = highest_altitude(x, y, target);
            if found {
                launches.push((x, y, alt));
            }
        }
    }

    let highest = *launches.iter().max_by_key(|launch| (*launch).2).unwrap();
    highest
}

fn part2(target: &Target) -> i64 {
    let mut count = 0;
    for x in -1000..1000 {
        for y in -1000..1000 {
            let (_, found) = highest_altitude(x, y, target);
            if found {
                count += 1;
            }
        }
    }

    count
}

const INPUT: &Target = &Target {
    x_min: 70,
    x_max: 96,
    y_min: -179,
    y_max: -124,
};
const TEST: &Target = &Target {
    x_min: 20,
    x_max: 30,
    y_min: -10,
    y_max: -5,
};

fn main() {
    assert_eq!(45, highest_altitude(6, 9, TEST).0);
    println!("{:?}", part1(INPUT));
    assert_eq!((7, 9, 45), part1(TEST));
    assert_eq!(112, part2(TEST));
    println!("{:?}", part2(INPUT));
}
