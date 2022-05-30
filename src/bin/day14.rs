use crate::io::{self, stdin, BufRead};
use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct CountedIngredient {
    count: u64,
    ingredient: String,
}

impl FromStr for CountedIngredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, ingredient) = s.split(" ").collect_tuple().unwrap();
        Ok(CountedIngredient {
            count: count.parse().unwrap(),
            ingredient: ingredient.to_string(),
        })
    }
}

#[derive(Debug)]
struct Recipe {
    output: CountedIngredient,
    inputs: Vec<CountedIngredient>,
}

impl FromStr for Recipe {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, out) = s.split(" => ").collect_tuple().unwrap();
        Ok(Recipe {
            output: out.parse().unwrap(),
            inputs: ins.split(", ").map(|s| s.parse().unwrap()).collect(),
        })
    }
}

fn add_ingredients(recipe: &Recipe, required_ingredients: &mut HashMap<String, i64>, times: u64) {
    for ingredient in &recipe.inputs {
        match required_ingredients.get_mut(&ingredient.ingredient) {
            None => {
                required_ingredients.insert(
                    ingredient.ingredient.clone(),
                    (ingredient.count * times) as i64,
                );
            }
            Some(i) => *i += (ingredient.count * times) as i64,
        }
    }
    match required_ingredients.get_mut(&recipe.output.ingredient) {
        None => {
            required_ingredients.insert(
                recipe.output.ingredient.clone(),
                -((recipe.output.count * times) as i64),
            );
        }
        Some(i) => *i -= (recipe.output.count * times) as i64,
    }
}

fn add_fuel(
    recipes: &HashMap<String, Recipe>,
    mut required_ingredients: &mut HashMap<String, i64>,
    required_ore: &mut u64,
    times: u64,
) {
    add_ingredients(recipes.get("FUEL").unwrap(), required_ingredients, times);
    while !required_ingredients.iter().all(|(_, &c)| c <= 0) {
        let keys = required_ingredients.keys().map(|k| k.clone()).collect_vec();
        for ingredient in &keys {
            let count = *required_ingredients.get(ingredient).unwrap();
            if count == 0 {
                required_ingredients.remove(ingredient);
                continue;
            } else if ingredient == "ORE" {
                *required_ore += count as u64;
                required_ingredients.remove(ingredient);
                continue;
            }
            if count <= 0 {
                continue;
            }
            match recipes.get(ingredient) {
                None => panic!("No recipe for {}", ingredient),
                Some(recipe) => {
                    let times = Integer::div_ceil(&(count as u64), &recipe.output.count);
                    add_ingredients(recipe, &mut required_ingredients, times);
                }
            }
        }
    }
}

const ORE_COUNT: u64 = 1000000000000;

#[no_mangle]
pub fn day_14() {
    let stdin = stdin();
    let mut recipes = HashMap::new();
    for recipe in stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().parse::<Recipe>().unwrap())
    {
        recipes.insert(recipe.output.ingredient.clone(), recipe);
    }

    let mut required_ore = 0;
    let mut required_ingredients: HashMap<String, i64> = HashMap::new();
    add_fuel(&recipes, &mut required_ingredients, &mut required_ore, 1);
    io::println!("ORE: {}", required_ore);

    let mut fuel_count = 1;
    let single_fuel_ore = required_ore;

    while required_ore < ORE_COUNT {
        let fuel_guess = (ORE_COUNT - required_ore) / single_fuel_ore;
        if fuel_guess == 0 {
            break;
        }
        add_fuel(
            &recipes,
            &mut required_ingredients,
            &mut required_ore,
            fuel_guess,
        );
        fuel_count += fuel_guess;
        assert!(required_ore <= ORE_COUNT);
    }
    loop {
        add_fuel(&recipes, &mut required_ingredients, &mut required_ore, 1);
        if required_ore > ORE_COUNT {
            break;
        }
        fuel_count += 1;
    }
    io::println!("FUEL: {}", fuel_count);
}
