// mod algo_warmup;
// use algo_warmup::gcd::execute;
// use algo_warmup::lcm::execute;

// mod greedy_algorithm;
// use greedy_algorithm::maximum_loot::execute;
// use greedy_algorithm::car_fuel::execute;
// use greedy_algorithm::max_ad_revenue::execute;
// use greedy_algorithm::max_num_prizes::execute;
// use greedy_algorithm::max_salary::execute;

// mod divide_and_conquer;
// use divide_and_conquer::binary_search::execute;
// use divide_and_conquer::major_element::execute;
// use divide_and_conquer::quick_sort::execute;
// use divide_and_conquer::merge_sort::execute;
// use divide_and_conquer::counting_inversions::execute;
// use divide_and_conquer::organizing_lottery::execute;

mod dynamic_programming;
// use dynamic_programming::count_change::execute;
// use dynamic_programming::money_change_again::execute;
// use dynamic_programming::primitive_calculator::execute;
// use dynamic_programming::edit_distance::execute;
// use dynamic_programming::longest_common_subsequence::execute;
// use dynamic_programming::max_amount_with_repetitions::execute;
// use dynamic_programming::subset_sum::execute;
use dynamic_programming::max_num_operands::execute;

fn main() {
    execute();
    // maximum_loot::max_loot();
}
