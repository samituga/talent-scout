use anyhow::Result;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn fetch_all_match_ids(
    api: &riven::RiotApi,
    puuid: &str,
    route: riven::consts::RegionalRoute,
) -> Result<Vec<String>> {
    let match_v5 = api.match_v5();
    let mut all_match_ids = Vec::new();
    let mut start = 0;
    let count = 100;

    loop {
        let match_ids = match_v5
            .get_match_ids_by_puuid(route, puuid, Some(count), None, None, None, Some(start), None)
            .await?;

        if match_ids.is_empty() {
            break;
        }

        all_match_ids.extend(match_ids.clone());
        start += match_ids.len() as i32;
    }

    Ok(all_match_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
