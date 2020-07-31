#[derive(Debug, Clone)]
pub struct Coins {
    coins: Vec<u32>,
}

fn count_change(money: u32, all_coins: Coins) -> u32 {
    let mut coins = all_coins.coins.clone();
    coins.sort(); // O(n(log(n)))

    //2d array to save coins relative to the money value
    let mut total_possibilities = vec![vec![0; coins.len()]; money as usize + 1];

    // where money = 6 and coins = [2, 3, 4];
    //  money/coins| 0 | 1 | 2 | 3 | 4 | 5 | 6 |
    //     -----------------------------------------
    // coins-> | 2 | 1 | 0 | 1 | 0 | 1 | 0 | 1 |
    //     --------------------------------------
    // coins-> | 3 | 1 | 0 | 1 | 1 | 0 | 1 | 2 |
    //     --------------------------------------
    // coins-> | 4 | 1 | 0 | 1 | 1 | 1 | 1 | 2 |

    // time complexity => O(mn) where m is 0 to the *money value* given and n is the length of the coins given
    for coin_index in 0..coins.len() as usize {
        for money_value in 0..=money {
            if money_value == 0 {
                total_possibilities[money_value as usize][coin_index as usize] = 1;
            } else if coins[coin_index] == coins[0] {
                if money_value % coins[coin_index] != 0 {
                    // println!("inside life");
                    total_possibilities[money_value as usize][coin_index as usize] = 0;
                } else {
                    total_possibilities[money_value as usize][coin_index as usize] = 1;
                }
            } else {
                let previous_possibilities =
                    total_possibilities[money_value as usize][(coin_index - 1) as usize];
                // if the value of the coin is greater than the money value, just copy the values above it
                if coins[coin_index] > money_value {
                    total_possibilities[money_value as usize][coin_index as usize] =
                        previous_possibilities;
                } else {
                    let remainder = money_value - coins[coin_index];
                    let remainder_value =
                        total_possibilities[remainder as usize][coin_index as usize];

                    total_possibilities[money_value as usize][coin_index as usize] =
                        previous_possibilities + remainder_value;
                }
            }
        }
    }
    println!("FINAL >>>>> {:?}", total_possibilities);
    total_possibilities[money as usize][coins.len() - 1]
}

pub fn execute() {
    let coins = Coins {
        coins: vec![1, 3, 4],
    };
    let result = count_change(34, coins);
    println!("Possible coin types {}", result);
}

// Tutorials I found very useful for this implementation:println!
// https://www.youtube.com/watch?v=L27_JpN6Z1Q
// https://www.youtube.com/watch?v=DJ4a7cmjZY0
