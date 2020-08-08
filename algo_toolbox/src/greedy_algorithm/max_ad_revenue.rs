pub struct Advertisement {
    ads_number: u32,
    profit_per_click: Vec<i32>,
    average_daily_click: Vec<i32>,
}

pub fn maximum_ad_revenue(ad_info: &mut Advertisement) -> i32 {
    ad_info.profit_per_click.sort();
    ad_info.average_daily_click.sort();
    let mut cumulative = 0;

    if ad_info.profit_per_click.len() == ad_info.average_daily_click.len() {
        for index in 0..ad_info.profit_per_click.len() {
            cumulative += ad_info.profit_per_click[index] * ad_info.average_daily_click[index]
        }
    }
    cumulative
}

pub fn execute() {
    let mut ad1 = Advertisement {
        ads_number: 1,
        profit_per_click: vec![23],
        average_daily_click: vec![39],
    };
    println!("{}", &mut maximum_ad_revenue(&mut ad1));

    let mut ad2 = Advertisement {
        ads_number: 3,
        profit_per_click: vec![1, 3, -5],
        average_daily_click: vec![-2, 4, 1],
    };
    println!("{}", &mut maximum_ad_revenue(&mut ad2));

    let mut ad3 = Advertisement {
        ads_number: 3,
        profit_per_click: vec![1, 3, -5],
        average_daily_click: vec![2, 4, 1],
    };
    println!("{}", maximum_ad_revenue(&mut ad3));
}
