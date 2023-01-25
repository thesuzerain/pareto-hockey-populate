use rusqlite::{params, Connection};

use crate::models::{*, self};
use crate::database::models::*;

use crate::database::connect::DATABASE_FILE_LOC;

pub fn populate_leagues(leagues: Vec<league::League>) -> rusqlite::Result<()>{

    let conn = Connection::open(DATABASE_FILE_LOC)?;
    let leagues = leagues.into_iter().map(|l| league_record::LeagueRecord::from(l));

    for l in leagues {
        conn.execute(
            "INSERT INTO league(slug, name, league_tier) VALUES (?1, ?2, ?3)",
            params![l.slug, l.name, l.league_tier],
        )?;
    }
    Ok(())
}

pub fn populate_players(players: Vec<player::Player>, drafts : Vec<draft_selection::DraftSelection>) -> rusqlite::Result<()>{

    let conn = Connection::open(DATABASE_FILE_LOC)?;
    let players = players.into_iter().map( 
        |p| {
            let pid = p.id;
            player_record::PlayerRecord::from(p, drafts.iter().find(|d| d.id == pid))
        }
    );

    for p in players {
        conn.execute(
            "INSERT INTO player(id, name, position, date_of_birth, draft_age, draft_year, round, overall) VALUES (?1, ?2, ?3)",
            params![p.id, p.name, p.position.to_string(), p.date_of_birth, p.draft_age, p.draft_year,
            p.round, p.overall],
        )?;
    }

    Ok(())
}

pub fn populate_player_seasons(player_seasons: Vec<player_season::PlayerSeason>) -> rusqlite::Result<()>{

    let conn = Connection::open(DATABASE_FILE_LOC)?;
    let player_seasons = player_seasons.into_iter().map(|pss| player_season_record::PlayerSeasonRecord::from(pss));

    for pss in player_seasons {
        conn.execute(
            "INSERT INTO player_team_statblock(id, player_id, team_id, sesaon_start_year, gp, g, a, pts, ppg) VALUES (?1, ?2)",
            params![pss.id, pss.player_id, pss.team_id, pss.season_start_year, 
            pss.gp, pss.g, pss.a, pss.pts, pss.ppg],
        )?;
    }
    Ok(())
}

pub fn populate_teams(teams: Vec<team::Team>) -> rusqlite::Result<()>{

    let conn = Connection::open(DATABASE_FILE_LOC)?;
    let teams = teams.into_iter().map(|t| team_record::TeamRecord::from(t));

    for t in teams {
        conn.execute(
            "INSERT INTO team(id, name) VALUES (?1, ?2)",
            params![t.id, t.name],
        )?;
    }
    Ok(())
}

pub fn populate_team_seasons(team_seasons: Vec<team_season::TeamSeason>) -> rusqlite::Result<()>{

    let conn = Connection::open(DATABASE_FILE_LOC)?;
    let team_seasons = team_seasons.into_iter().map(|t| team_season_record ::TeamSeasonRecord::from(t));

    for tr in team_seasons {
        conn.execute(
            "INSERT INTO team_records(id, team_id, season_start_year, group, gp, g, a, pts, ppg) VALUES (?1, ?2, ?3)",
            params![tr.id, tr.team_id, tr.season_start_year, tr.group,
            tr.gp, tr.g, tr.a, tr.pts, tr.ppg],
        )?;
    }
    Ok(())
}
