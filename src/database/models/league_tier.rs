use lazy_static::lazy_static;

// Gets Pareto-assigned tier of a league by its EP-API league slug
pub fn get_league_tier(league_slug : &str) -> usize {

    // TODO: is there a better way to do this? uneven array as a const with size known at compile time?
    lazy_static! {
        // Predefined list of league tiers
        static ref LEAGUE_TIERS : Vec<Vec<&'static str>> = vec![
            vec!["nhl"], // tier 1 
            vec!["khl"], // tier 2
            vec!["czechia","shl","liiga","nla"], // tier 3
            vec!["ahl"], // tier 4
            vec!["hockeyallsvenskan","vhl","slovakia","belarus"], // tier 5
            vec!["mestis"], // tier 6
            vec!["czech2","ncaa","denmark","sl","norway","slovakia2"], // tier 7
            vec!["echl"], // tier 8
            vec!["ohl","ushl","mhl","whl","j20-nationell"], // tier 9
            vec!["qmjhl","u20-sm-sarja"], // tier 10
            vec!["usports","bchl","ajhl","sjhl","mjhl","ojhl"], // tier 11
            vec!["czech-u20","belarus-vysshaya","u20-elit","slovakia-u20"], // tier 12
            vec!["cchl"], // tier 13
            vec!["u18-sm-sarja","j18-region","ushs-mn"], // tier 14
            vec!["czech-u18","slovakia-u18"] // tier 15
            // max tier = 16 (any league not in list)
        ];
    }
    
    for (i,tier) in LEAGUE_TIERS.iter().enumerate() {
        if tier.contains(&league_slug) {
            return i + 1;
        }
    }
    LEAGUE_TIERS.len() + 1 // if league not in list, return maximum tier

}
