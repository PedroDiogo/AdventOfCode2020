use super::lib::*;
use std::collections::{HashMap, HashSet};

extern crate regex;
use self::regex::Regex;

pub fn run() -> (Option<String>, Option<String>) {
    let filename = "inputs/day21.txt";
    let inputs = read_inputs(&filename);
    let allergens = find_allergens(&inputs);
    let all_ingredients = get_ingredients(&inputs);

    let part_one =
        Some(number_of_ingredients_with_no_allergens(&all_ingredients, &allergens).to_string());
    let part_two = None;

    (part_one, part_two)
}

fn find_allergens(inputs: &str) -> HashMap<String, String> {
    let re = Regex::new(r"((?:\w+ )+)\(contains ((?:\w+(?:, )*)*)\)").unwrap();
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    for cap in re.captures_iter(inputs) {
        let ingredients = cap
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();
        let _allergens = cap
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .for_each(|allergen| {
                let ingredients_list = result.get(allergen);
                let new_ingredients_list = match ingredients_list {
                    Some(x) => x
                        .intersection(&ingredients)
                        .map(|x| x.to_string())
                        .collect(),
                    _ => ingredients.clone(),
                };
                result.insert(allergen.to_string(), new_ingredients_list);
            });
    }
    let mut allergens: HashMap<String, String> = HashMap::new();

    while !result.is_empty() {
        result
            .iter()
            .filter(|(_, values)| values.len() == 1)
            .for_each(|(allergen, ingredients)| {
                allergens.insert(allergen.clone(), ingredients.iter().next().unwrap().clone());
            });

        for (allergen, ingredient) in allergens.iter() {
            result.remove(allergen);
            result.iter_mut().for_each(|(_, ingredients)| {
                (*ingredients).remove(ingredient);
            });
        }
    }
    allergens
}

fn get_ingredients(inputs: &str) -> Vec<HashSet<&str>> {
    let re = Regex::new(r"((?:\w+ )+)\(contains ((?:\w+(?:, )*)*)\)").unwrap();

    re.captures_iter(inputs)
        .filter_map(|captures| captures.get(1))
        .map(|x| x.as_str().split_whitespace().collect())
        .collect()
}

fn number_of_ingredients_with_no_allergens(
    all_ingredients: &[HashSet<&str>],
    allergens: &HashMap<String, String>,
) -> usize {
    let allergen_ingredients: Vec<&str> =
        allergens.iter().map(|(_, value)| value.as_str()).collect();

    let all_ingredients_without_allergens: Vec<&str> = all_ingredients
        .iter()
        .fold(Vec::<&str>::new(), |mut result, ingredients_line| {
            result.extend(ingredients_line);
            result
        })
        .into_iter()
        .filter(|ingredient| !allergen_ingredients.contains(ingredient))
        .collect();
    all_ingredients_without_allergens.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASE_1: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_find_allergens() {
        let allergens = find_allergens(TEST_CASE_1);
        let expected_allergens: HashMap<String, String> =
            [("dairy", "mxmxvkd"), ("fish", "sqjhc"), ("soy", "fvjkl")]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

        assert_eq!(expected_allergens, allergens);
    }

    #[test]
    fn test_get_ingredients() {
        let ingredients = get_ingredients(TEST_CASE_1);
        let expected_ingredients: Vec<HashSet<&str>> = vec![
            vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"]
                .into_iter()
                .collect(),
            vec!["trh", "fvjkl", "sbzzf", "mxmxvkd"]
                .into_iter()
                .collect(),
            vec!["sqjhc", "fvjkl"].into_iter().collect(),
            vec!["sqjhc", "mxmxvkd", "sbzzf"].into_iter().collect(),
        ];

        assert_eq!(expected_ingredients, ingredients);
    }

    #[test]
    fn test_number_of_ingredients_with_no_allergens() {
        let ingredients = get_ingredients(TEST_CASE_1);
        let allergens = find_allergens(TEST_CASE_1);
        assert_eq!(
            5,
            number_of_ingredients_with_no_allergens(&ingredients, &allergens)
        );
    }
}
