use crate::single_match::SingleMatch;

pub fn match_report(s_match: SingleMatch) -> String {
    let mut lines = format!(
        "Match {}\nTotal kills: {}\nRanking:\n",
        s_match.id, s_match.total_kills,
    );
    let mut ranking = s_match
        .kills
        .into_iter()
        .map(|m| (m.0, m.1))
        .collect::<Vec<_>>();
    ranking.sort_by(|a, b| b.1.cmp(&a.1));
    for (i, (player, kills)) in ranking.into_iter().enumerate() {
        lines.push_str(&format!("{} - {player}: {kills}\n", i + 1));
    }

    let mut means = s_match
        .means_of_death
        .into_iter()
        .map(|m| (m.0, m.1))
        .collect::<Vec<_>>();
    means.sort_by(|a, b| b.1.cmp(&a.1));
    for (i, (means, kills)) in means.into_iter().enumerate() {
        lines.push_str(&format!("{} - {means}: {kills}\n", i + 1));
    }

    lines.push('\n');
    lines
}
