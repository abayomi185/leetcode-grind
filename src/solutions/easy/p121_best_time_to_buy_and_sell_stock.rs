/*
Leetcode: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

Progression
1. Started with 2 variables for lowest_price and highest_price.
Looping through and using 2 match cases.

2. Changed to functional way of doing things and convert 2 match cases in 1 match case.
Utilising a max value in the vector as with the 1st attempt.

Improvements
1. Use the fold method on the vec, which is like a reduce in JS.
*/
#[cfg(test)]
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut lowest_price: i32 = prices[0];

    // let mut highest_price: i32 = 0;
    // let mut profit: Vec<i32> = vec![];

    // for price in prices {
    //     match price > highest_price {
    //         true => {
    //             highest_price = price;
    //             profit.push(highest_price - lowest_price)
    //         }
    //         false => (),
    //     }
    //     match price < lowest_price {
    //         true => {
    //             lowest_price = price;
    //             highest_price = price;
    //         }
    //         false => continue,
    //     }
    // }
    // * profit.iter().max().unwrap()

    // The functional way
    prices
        .iter()
        .map(|price| match price < &lowest_price {
            true => {
                lowest_price = *price;
                0
            }
            false => price - lowest_price,
        })
        .max()
        .unwrap()
}

#[test]
fn test_max_profit() {
    let test_cases: [(Vec<i32>, i32); 2] = [(vec![7, 1, 5, 3, 6, 4], 5), (vec![7, 6, 4, 3, 1], 0)];

    for (case, want) in test_cases {
        assert_eq!(max_profit(case), want);
    }
}
