use std::str::FromStr;

use anyhow::{Error, Ok, Result};

#[derive(Debug)]
struct CubeSet {
    num_red: Option<usize>,
    num_green: Option<usize>,
    num_blue: Option<usize>,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

impl FromStr for CubeSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = s.split(", ");

        let mut num_red: Option<usize> = None;
        let mut num_green: Option<usize> = None;
        let mut num_blue: Option<usize> = None;
        for cubes in list {
            let (num_str, color) = cubes.split_once(" ").unwrap();
            let num = num_str.parse::<usize>().unwrap();

            match color {
                "red" => num_red = Some(num),
                "green" => num_green = Some(num),
                "blue" => num_blue = Some(num),
                _ => panic!(),
            }
        }

        Ok(CubeSet {
            num_red,
            num_green,
            num_blue,
        })
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (game_identifier, set_list) = s.split_once(": ").unwrap();
        let (_, game_id_str) = game_identifier.split_once(" ").unwrap();

        let sets: Vec<CubeSet> = set_list
            .split("; ")
            .map(|set| set.parse().unwrap())
            .collect();

        Ok(Game {
            id: game_id_str.parse::<usize>().unwrap(),
            sets,
        })
    }
}

static MAX_RED: usize = 12;
static MAX_GREEN: usize = 13;
static MAX_BLUE: usize = 14;

impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(|set| {
            set.num_red <= Some(MAX_RED)
                && set.num_green <= Some(MAX_GREEN)
                && set.num_blue <= Some(MAX_BLUE)
        })
    }

    fn min_possible(&self) -> CubeSet {
        let max_red = self.sets.iter().map(|set| set.num_red).max().unwrap();
        let max_green = self.sets.iter().map(|set| set.num_green).max().unwrap();
        let max_blue = self.sets.iter().map(|set| set.num_blue).max().unwrap();

        CubeSet {
            num_red: max_red,
            num_green: max_green,
            num_blue: max_blue,
        }
    }
}

impl CubeSet {
    fn power(&self) -> usize {
        self.num_red.unwrap_or(1) * self.num_blue.unwrap_or(1) * self.num_green.unwrap_or(1)
    }
}

fn part_one(games: &Vec<Game>) -> usize {
    games
        .iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

fn part_two(games: &Vec<Game>) -> usize {
    games
        .iter()
        .map(|game| game.min_possible())
        .map(|set| set.power())
        .sum()
}

fn main() {
    let games: Vec<Game> = include_str!("../inputs/2.txt")
        .lines()
        .map(|line| {
            line.parse::<Game>()
                .expect("Error parsing game from string")
        })
        .collect();

    println!("Part 1: {}", part_one(&games));
    println!("Part 2: {}", part_two(&games));
}
