mod longest_common_prefix;

fn main() {
    let strs = vec![String::from("facebook"), String::from("factory"), String::from("facy")];
    let prefix = longest_common_prefix::Solution::longest_common_prefix(strs);
    println!("Prefix = {}", prefix);

}

