fn main() {
    //let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
    let prices: Vec<i32> = vec![9, 1, 5, 3, 7, 2, 4, 8];
    //let prices: Vec<i32> = vec![7, 6, 4, 3, 1];

    fn profit(prices: Vec<i32>, buy_day: usize) -> i32 {
        let slice = &prices[buy_day..];
        let max = slice.iter().max().unwrap();

        println!("Slice: {:?}", slice);
        println!("Buy day: {}", buy_day);
        println!("Buy day value: {}", prices[buy_day - 1]);
        println!("Max value in future: {:?}", max);

        if max < &prices[buy_day - 1] {
            return 0;
        }

        max - prices[buy_day - 1]
    }

    println!("Profit: {}", profit(prices, 4));
}
