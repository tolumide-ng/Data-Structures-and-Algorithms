pub fn execute() {
    println!("min coins needed {:?}", money_change(34));
    // money_change(11);
}

#[derive(Debug)]
pub struct MoneyChange {
    min_coins: u8,
    coin_values: Vec<u8>,
}

fn money_change(money: usize) -> MoneyChange {
    use std::cmp;
    let mut coin_denomination = vec![1, 3, 4];
    // let mut coin_denomination = vec![5, 6];

    coin_denomination.sort();

    let mut money_coins = vec![vec![0; coin_denomination.len()]; money + 1];

    for coin_index in 0..coin_denomination.len() {
        for money_index in 0..=money {
            if money_index == 0 {
                money_coins[money_index][coin_index] = 0;
            } else if coin_index == 0 {
                if money_index >= coin_denomination[coin_index] {
                    let remainder = money_index - coin_denomination[coin_index];
                    let value_at_remainder = money_coins[remainder][coin_index];
                    money_coins[money_index][coin_index] = value_at_remainder + 1;
                } else {
                    money_coins[money_index][coin_index] = 0;
                }
            } else {
                if money_index < coin_denomination[coin_index] {
                    money_coins[money_index][coin_index] = money_coins[money_index][coin_index - 1];
                } else {
                    let remainder = money_index - coin_denomination[coin_index];
                    let value_at_remainder = money_coins[remainder][coin_index];
                    let eventual_value = value_at_remainder + 1;
                    let previous_coin_value = money_coins[money_index][coin_index - 1];
                    money_coins[money_index][coin_index] =
                        cmp::min(eventual_value, previous_coin_value);
                }
            }
        }
    }

    // println!("length of the coins {:#?}", money_coins);

    let minimum_coins_needed = money_coins[money][coin_denomination.len() - 1];

    // let mut coin_place = coin_denomination.len() - 1;
    // let mut money_place = money;
    // // let mut total_money_needed = money;
    // let mut coins_needed: Vec<u8> = vec![];

    // loop {
    //     if money_coins[money_place][coin_place] == money_coins[money_place - 1][coin_place - 1] {
    //         println!(" FROM ONE money place {}", money_place);
    //         println!(" FROM ONE coins needed {:?}", coin_place);
    //         coin_place -= 1;
    //         println!("  AFTER  ONE money place {}", money_place);
    //         println!(" AFTER  ONE coins needed {:?}", coin_place);
    //     } else {
    //         coins_needed.push(coin_denomination[coin_place] as u8);
    //         println!(" money place {}", money_place);
    //         println!("coins needed {:?}", coins_needed);
    //         // println!(
    //         //     "is this possible???? {:?}",
    //         //     coins_needed[(coins_needed.len() - 1) as usize] as usize
    //         // );
    //         let money_index_remaining =
    //             money_place - coins_needed[(coins_needed.len() - 1) as usize] as usize;
    //         money_place = money_index_remaining;
    //         if money_place == 0 && coin_place == 0 {
    //             break;
    //         }
    //     }
    // }

    // println!("all coin values {:?}", &coins_needed);

    MoneyChange {
        min_coins: minimum_coins_needed,
        coin_values: vec![],
    }
}
