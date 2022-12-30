use std::collections::HashMap;

impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
        let mut creator_video_views = HashMap::new();

        for ((creator, id), view_count) in creators.iter().zip(ids).zip(views) {
            creator_video_views
                .entry(creator)
                .or_insert(HashMap::new())
                .insert(id, view_count as u32);
        }

        let highest_total_views = creator_video_views
            .iter()
            .map(|(c, m)| (c, m.values().sum::<u32>()))
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;
        creator_video_views
            .iter()
            .filter(|(_, i_v)| i_v.values().sum::<u32>() == highest_total_views)
            .map(|(c, i_v)| {
                let highest_views = i_v.values().max().unwrap();
                vec![
                    c.to_string(),
                    (i_v.iter()
                        .filter(|(_, v)| *v == highest_views)
                        .min_by(|a, b| a.0.cmp(b.0))
                        .unwrap()
                        .0)
                        .to_string(),
                ]
            })
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn popularity_tie_and_view_count_tie() {
        assert_eq!(
            Solution::most_popular_creator(
                ["alice", "bob", "alice", "chris"]
                    .map(String::from)
                    .to_vec(),
                ["one", "two", "three", "four"].map(String::from).to_vec(),
                vec![5, 10, 5, 4],
            )
                .sort_by(|a, b| a.cmp(b)),
            vec![["alice", "one"], ["bob", "two"]].sort_by(|a, b| a.cmp(b))
        )
    }

    #[test]
    fn view_count_tie() {
        assert_eq!(
            Solution::most_popular_creator(
                ["alice", "alice", "alice"].map(String::from).to_vec(),
                ["a", "b", "c"].map(String::from).to_vec(),
                vec![1, 2, 2],
            )
                .sort_by(|a, b| a.cmp(b)),
            [["alice", "b"]].sort_by(|a, b| a.cmp(b))
        )
    }
}
