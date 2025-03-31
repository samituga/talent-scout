CREATE TABLE match_v5.participants
(
    puuid                              TEXT    NOT NULL,
    match_id                           TEXT    NOT NULL,

    all_in_pings                       INTEGER,
    assist_me_pings                    INTEGER,
    assists                            INTEGER NOT NULL,
    baron_kills                        INTEGER NOT NULL,
    bounty_level                       INTEGER NOT NULL,
    champ_experience                   INTEGER NOT NULL,
    champ_level                        INTEGER NOT NULL,
    champion_id                        INTEGER NOT NULL,
    champion_name                      TEXT    NOT NULL,
    command_pings                      INTEGER,
    champion_transform                 INTEGER NOT NULL,
    consumables_purchased              INTEGER NOT NULL,
--     challenges                         TEXT,
    damage_dealt_to_buildings          INTEGER,
    damage_dealt_to_objectives         INTEGER NOT NULL,
    damage_dealt_to_turrets            INTEGER NOT NULL,
    damage_self_mitigated              INTEGER NOT NULL,
    deaths                             INTEGER NOT NULL,
    detector_wards_placed              INTEGER NOT NULL,
    double_kills                       INTEGER NOT NULL,
    dragon_kills                       INTEGER NOT NULL,
    eligible_for_progression           BOOLEAN,
    enemy_missing_pings                INTEGER,
    enemy_vision_pings                 INTEGER,
    first_blood_assist                 BOOLEAN NOT NULL,
    first_blood_kill                   BOOLEAN NOT NULL,
    first_tower_assist                 BOOLEAN NOT NULL,
    first_tower_kill                   BOOLEAN NOT NULL,
    game_ended_in_early_surrender      BOOLEAN NOT NULL,
    game_ended_in_surrender            BOOLEAN NOT NULL,
    hold_pings                         INTEGER,
    get_back_pings                     INTEGER,
    gold_earned                        INTEGER NOT NULL,
    gold_spent                         INTEGER NOT NULL,
    individual_position                TEXT    NOT NULL,
    inhibitor_kills                    INTEGER NOT NULL,
    inhibitor_takedowns                INTEGER,
    inhibitors_lost                    INTEGER,
    item0                              INTEGER NOT NULL,
    item1                              INTEGER NOT NULL,
    item2                              INTEGER NOT NULL,
    item3                              INTEGER NOT NULL,
    item4                              INTEGER NOT NULL,
    item5                              INTEGER NOT NULL,
    item6                              INTEGER NOT NULL,
    items_purchased                    INTEGER NOT NULL,
    killing_sprees                     INTEGER NOT NULL,
    kills                              INTEGER NOT NULL,
    lane                               TEXT    NOT NULL,
    largest_critical_strike            INTEGER NOT NULL,
    largest_killing_spree              INTEGER NOT NULL,
    largest_multi_kill                 INTEGER NOT NULL,
    longest_time_spent_living          INTEGER NOT NULL,
    magic_damage_dealt                 INTEGER NOT NULL,
    magic_damage_dealt_to_champions    INTEGER NOT NULL,
    magic_damage_taken                 INTEGER NOT NULL,
--     missions                           TEXT,
    neutral_minions_killed             INTEGER NOT NULL,
    need_vision_pings                  INTEGER,
    nexus_kills                        INTEGER NOT NULL,
    nexus_takedowns                    INTEGER,
    nexus_lost                         INTEGER,
    objectives_stolen                  INTEGER NOT NULL,
    objectives_stolen_assists          INTEGER NOT NULL,
    on_my_way_pings                    INTEGER,
    participant_id                     INTEGER NOT NULL,
    player_score0                      DECIMAL(20, 9),
    player_score1                      DECIMAL(20, 9),
    player_score2                      DECIMAL(20, 9),
    player_score3                      DECIMAL(20, 9),
    player_score4                      DECIMAL(20, 9),
    player_score5                      DECIMAL(20, 9),
    player_score6                      DECIMAL(20, 9),
    player_score7                      DECIMAL(20, 9),
    player_score8                      DECIMAL(20, 9),
    player_score9                      DECIMAL(20, 9),
    player_score10                     DECIMAL(20, 9),
    player_score11                     DECIMAL(20, 9),
    penta_kills                        INTEGER NOT NULL,
--     perks                              TEXT    NOT NULL,
    physical_damage_dealt              INTEGER NOT NULL,
    physical_damage_dealt_to_champions INTEGER NOT NULL,
    physical_damage_taken              INTEGER NOT NULL,
    placement                          INTEGER,
    player_augment1                    INTEGER,
    player_augment2                    INTEGER,
    player_augment3                    INTEGER,
    player_augment4                    INTEGER,
    player_subteam_id                  INTEGER,
    push_pings                         INTEGER,
    profile_icon                       INTEGER NOT NULL,
    quadra_kills                       INTEGER NOT NULL,
    riot_id_game_name                  TEXT,
    riot_id_tagline                    TEXT,
    "role"                             TEXT    NOT NULL,
    sight_wards_bought_in_game         INTEGER NOT NULL,
    spell1_casts                       INTEGER NOT NULL,
    spell2_casts                       INTEGER NOT NULL,
    spell3_casts                       INTEGER NOT NULL,
    spell4_casts                       INTEGER NOT NULL,
    subteam_placement                  INTEGER,
    summoner1_casts                    INTEGER NOT NULL,
    summoner1_id                       INTEGER NOT NULL,
    summoner2_casts                    INTEGER NOT NULL,
    summoner2_id                       INTEGER NOT NULL,
    summoner_id                        TEXT    NOT NULL,
    summoner_level                     INTEGER NOT NULL,
    summoner_name                      TEXT    NOT NULL,
    team_early_surrendered             BOOLEAN NOT NULL,
    team_id                            INTEGER NOT NULL,
    team_position                      TEXT    NOT NULL,
    time_c_cing_others                 INTEGER NOT NULL,
    time_played                        INTEGER NOT NULL,
    total_ally_jungle_minions_killed   INTEGER,
    total_damage_dealt                 INTEGER NOT NULL,
    total_damage_dealt_to_champions    INTEGER NOT NULL,
    total_damage_shielded_on_teammates INTEGER NOT NULL,
    total_damage_taken                 INTEGER NOT NULL,
    total_enemy_jungle_minions_killed  INTEGER,
    total_heal                         INTEGER NOT NULL,
    total_heals_on_teammates           INTEGER NOT NULL,
    total_minions_killed               INTEGER NOT NULL,
    total_time_cc_dealt                INTEGER NOT NULL,
    total_time_spent_dead              INTEGER NOT NULL,
    total_units_healed                 INTEGER NOT NULL,
    triple_kills                       INTEGER NOT NULL,
    true_damage_dealt                  INTEGER NOT NULL,
    true_damage_dealt_to_champions     INTEGER NOT NULL,
    true_damage_taken                  INTEGER NOT NULL,
    turret_kills                       INTEGER NOT NULL,
    turret_takedowns                   INTEGER,
    turrets_lost                       INTEGER,
    unreal_kills                       INTEGER NOT NULL,
    vision_score                       INTEGER NOT NULL,
    vision_cleared_pings               INTEGER,
    vision_wards_bought_in_game        INTEGER NOT NULL,
    wards_killed                       INTEGER NOT NULL,
    wards_placed                       INTEGER NOT NULL,
    win                                BOOLEAN NOT NULL,
    bait_pings                         INTEGER,
    danger_pings                       INTEGER,
    basic_pings                        INTEGER,
    player_augment5                    INTEGER,
    player_augment6                    INTEGER,
    riot_id_name                       TEXT,
    retreat_pings                      INTEGER,
    PRIMARY KEY (match_id, puuid),
    FOREIGN KEY (match_id) REFERENCES match_v5.matches (match_id)
);

COMMENT ON TABLE match_v5.participants IS 'Original model name - match-v5.ParticipantDto.';
COMMENT ON COLUMN match_v5.participants.all_in_pings IS 'Yellow crossed swords. Original param name - allInPings.';
COMMENT ON COLUMN match_v5.participants.assist_me_pings IS 'Green flag. Original param name - assistMePings.';
COMMENT ON COLUMN match_v5.participants.baron_kills IS 'Original param name - baronKills.';
COMMENT ON COLUMN match_v5.participants.bounty_level IS 'Original param name - bountyLevel.';
COMMENT ON COLUMN match_v5.participants.champ_experience IS 'Original param name - champExperience.';
COMMENT ON COLUMN match_v5.participants.champ_level IS 'Original param name - champLevel.';
COMMENT ON COLUMN match_v5.participants.champion_id IS 'Prior to patch 11.4, on Feb 18th, 2021, this field returned invalid championIds. We recommend determining the champion based on the championName field for matches played prior to patch 11.4.. Original param name - championId.';
COMMENT ON COLUMN match_v5.participants.champion_name IS 'Original param name - championName.';
COMMENT ON COLUMN match_v5.participants.command_pings IS 'Blue generic ping (ALT+click). Original param name - commandPings.';
COMMENT ON COLUMN match_v5.participants.champion_transform IS 'This field is currently only utilized for Kayn&#39;s transformations. (Legal values: 0 - None, 1 - Slayer, 2 - Assassin). Original param name - championTransform.';
COMMENT ON COLUMN match_v5.participants.consumables_purchased IS 'Original param name - consumablesPurchased.';
COMMENT ON COLUMN match_v5.participants.damage_dealt_to_buildings IS 'Original param name - damageDealtToBuildings.';
COMMENT ON COLUMN match_v5.participants.damage_dealt_to_objectives IS 'Original param name - damageDealtToObjectives.';
COMMENT ON COLUMN match_v5.participants.damage_dealt_to_turrets IS 'Original param name - damageDealtToTurrets.';
COMMENT ON COLUMN match_v5.participants.damage_self_mitigated IS 'Original param name - damageSelfMitigated.';
COMMENT ON COLUMN match_v5.participants.detector_wards_placed IS 'Original param name - detectorWardsPlaced.';
COMMENT ON COLUMN match_v5.participants.double_kills IS 'Original param name - doubleKills.';
COMMENT ON COLUMN match_v5.participants.dragon_kills IS 'Original param name - dragonKills.';
COMMENT ON COLUMN match_v5.participants.eligible_for_progression IS 'Original param name - eligibleForProgression.';
COMMENT ON COLUMN match_v5.participants.enemy_missing_pings IS 'Yellow questionmark. Original param name - enemyMissingPings.';
COMMENT ON COLUMN match_v5.participants.enemy_vision_pings IS 'Red eyeball. Original param name - enemyVisionPings.';
COMMENT ON COLUMN match_v5.participants.first_blood_assist IS 'Original param name - firstBloodAssist.';
COMMENT ON COLUMN match_v5.participants.first_blood_kill IS 'Original param name - firstBloodKill.';
COMMENT ON COLUMN match_v5.participants.first_tower_assist IS 'Original param name - firstTowerAssist.';
COMMENT ON COLUMN match_v5.participants.first_tower_kill IS 'Original param name - firstTowerKill.';
COMMENT ON COLUMN match_v5.participants.game_ended_in_early_surrender IS 'This is an offshoot of the OneStone challenge. The code checks if a spell with the same instance ID does the final point of damage to at least 2 Champions. It doesn&#39;t matter if they&#39;re enemies, but you cannot hurt your friends.. Original param name - gameEndedInEarlySurrender.';
COMMENT ON COLUMN match_v5.participants.game_ended_in_surrender IS 'Original param name - gameEndedInSurrender.';
COMMENT ON COLUMN match_v5.participants.hold_pings IS 'Original param name - holdPings.';
COMMENT ON COLUMN match_v5.participants.get_back_pings IS 'Yellow circle with horizontal line. Original param name - getBackPings.';
COMMENT ON COLUMN match_v5.participants.gold_earned IS 'Original param name - goldEarned.';
COMMENT ON COLUMN match_v5.participants.gold_spent IS 'Original param name - goldSpent.';
COMMENT ON COLUMN match_v5.participants.individual_position IS 'Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constraint that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.. Original param name - individualPosition.';
COMMENT ON COLUMN match_v5.participants.inhibitor_kills IS 'Original param name - inhibitorKills.';
COMMENT ON COLUMN match_v5.participants.inhibitor_takedowns IS 'Original param name - inhibitorTakedowns.';
COMMENT ON COLUMN match_v5.participants.inhibitors_lost IS 'Original param name - inhibitorsLost.';
COMMENT ON COLUMN match_v5.participants.items_purchased IS 'Original param name - itemsPurchased.';
COMMENT ON COLUMN match_v5.participants.killing_sprees IS 'Original param name - killingSprees.';
COMMENT ON COLUMN match_v5.participants.largest_critical_strike IS 'Original param name - largestCriticalStrike.';
COMMENT ON COLUMN match_v5.participants.largest_killing_spree IS 'Original param name - largestKillingSpree.';
COMMENT ON COLUMN match_v5.participants.largest_multi_kill IS 'Original param name - largestMultiKill.';
COMMENT ON COLUMN match_v5.participants.longest_time_spent_living IS 'Original param name - longestTimeSpentLiving.';
COMMENT ON COLUMN match_v5.participants.magic_damage_dealt IS 'Original param name - magicDamageDealt.';
COMMENT ON COLUMN match_v5.participants.magic_damage_dealt_to_champions IS 'Original param name - magicDamageDealtToChampions.';
COMMENT ON COLUMN match_v5.participants.magic_damage_taken IS 'Original param name - magicDamageTaken.';
COMMENT ON COLUMN match_v5.participants.neutral_minions_killed IS 'neutralMinionsKilled &#x3D; mNeutralMinionsKilled, which is incremented on kills of kPet and kJungleMonster. Original param name - neutralMinionsKilled.';
COMMENT ON COLUMN match_v5.participants.need_vision_pings IS 'Green ward. Original param name - needVisionPings.';
COMMENT ON COLUMN match_v5.participants.nexus_kills IS 'Original param name - nexusKills.';
COMMENT ON COLUMN match_v5.participants.nexus_takedowns IS 'Original param name - nexusTakedowns.';
COMMENT ON COLUMN match_v5.participants.nexus_lost IS 'Original param name - nexusLost.';
COMMENT ON COLUMN match_v5.participants.objectives_stolen IS 'Original param name - objectivesStolen.';
COMMENT ON COLUMN match_v5.participants.objectives_stolen_assists IS 'Original param name - objectivesStolenAssists.';
COMMENT ON COLUMN match_v5.participants.on_my_way_pings IS 'Blue arrow pointing at ground. Original param name - onMyWayPings.';
COMMENT ON COLUMN match_v5.participants.participant_id IS 'Original param name - participantId.';
COMMENT ON COLUMN match_v5.participants.player_score0 IS 'Original param name - playerScore0.';
COMMENT ON COLUMN match_v5.participants.player_score1 IS 'Original param name - playerScore1.';
COMMENT ON COLUMN match_v5.participants.player_score2 IS 'Original param name - playerScore2.';
COMMENT ON COLUMN match_v5.participants.player_score3 IS 'Original param name - playerScore3.';
COMMENT ON COLUMN match_v5.participants.player_score4 IS 'Original param name - playerScore4.';
COMMENT ON COLUMN match_v5.participants.player_score5 IS 'Original param name - playerScore5.';
COMMENT ON COLUMN match_v5.participants.player_score6 IS 'Original param name - playerScore6.';
COMMENT ON COLUMN match_v5.participants.player_score7 IS 'Original param name - playerScore7.';
COMMENT ON COLUMN match_v5.participants.player_score8 IS 'Original param name - playerScore8.';
COMMENT ON COLUMN match_v5.participants.player_score9 IS 'Original param name - playerScore9.';
COMMENT ON COLUMN match_v5.participants.player_score10 IS 'Original param name - playerScore10.';
COMMENT ON COLUMN match_v5.participants.player_score11 IS 'Original param name - playerScore11.';
COMMENT ON COLUMN match_v5.participants.penta_kills IS 'Original param name - pentaKills.';
COMMENT ON COLUMN match_v5.participants.physical_damage_dealt IS 'Original param name - physicalDamageDealt.';
COMMENT ON COLUMN match_v5.participants.physical_damage_dealt_to_champions IS 'Original param name - physicalDamageDealtToChampions.';
COMMENT ON COLUMN match_v5.participants.physical_damage_taken IS 'Original param name - physicalDamageTaken.';
COMMENT ON COLUMN match_v5.participants.player_augment1 IS 'Original param name - playerAugment1.';
COMMENT ON COLUMN match_v5.participants.player_augment2 IS 'Original param name - playerAugment2.';
COMMENT ON COLUMN match_v5.participants.player_augment3 IS 'Original param name - playerAugment3.';
COMMENT ON COLUMN match_v5.participants.player_augment4 IS 'Original param name - playerAugment4.';
COMMENT ON COLUMN match_v5.participants.player_subteam_id IS 'Original param name - playerSubteamId.';
COMMENT ON COLUMN match_v5.participants.push_pings IS 'Green minion. Original param name - pushPings.';
COMMENT ON COLUMN match_v5.participants.profile_icon IS 'Original param name - profileIcon.';
COMMENT ON COLUMN match_v5.participants.quadra_kills IS 'Original param name - quadraKills.';
COMMENT ON COLUMN match_v5.participants.riot_id_game_name IS 'Original param name - riotIdGameName.';
COMMENT ON COLUMN match_v5.participants.riot_id_tagline IS 'Original param name - riotIdTagline.';
COMMENT ON COLUMN match_v5.participants.sight_wards_bought_in_game IS 'Original param name - sightWardsBoughtInGame.';
COMMENT ON COLUMN match_v5.participants.spell1_casts IS 'Original param name - spell1Casts.';
COMMENT ON COLUMN match_v5.participants.spell2_casts IS 'Original param name - spell2Casts.';
COMMENT ON COLUMN match_v5.participants.spell3_casts IS 'Original param name - spell3Casts.';
COMMENT ON COLUMN match_v5.participants.spell4_casts IS 'Original param name - spell4Casts.';
COMMENT ON COLUMN match_v5.participants.subteam_placement IS 'Original param name - subteamPlacement.';
COMMENT ON COLUMN match_v5.participants.summoner1_casts IS 'Original param name - summoner1Casts.';
COMMENT ON COLUMN match_v5.participants.summoner1_id IS 'Original param name - summoner1Id.';
COMMENT ON COLUMN match_v5.participants.summoner2_casts IS 'Original param name - summoner2Casts.';
COMMENT ON COLUMN match_v5.participants.summoner2_id IS 'Original param name - summoner2Id.';
COMMENT ON COLUMN match_v5.participants.summoner_id IS 'Original param name - summonerId.';
COMMENT ON COLUMN match_v5.participants.summoner_level IS 'Original param name - summonerLevel.';
COMMENT ON COLUMN match_v5.participants.summoner_name IS 'Original param name - summonerName.';
COMMENT ON COLUMN match_v5.participants.team_early_surrendered IS 'Original param name - teamEarlySurrendered.';
COMMENT ON COLUMN match_v5.participants.team_id IS 'Original param name - teamId.';
COMMENT ON COLUMN match_v5.participants.team_position IS 'Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constraint that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.. Original param name - teamPosition.';
COMMENT ON COLUMN match_v5.participants.time_c_cing_others IS 'Original param name - timeCCingOthers.';
COMMENT ON COLUMN match_v5.participants.time_played IS 'Original param name - timePlayed.';
COMMENT ON COLUMN match_v5.participants.total_ally_jungle_minions_killed IS 'Original param name - totalAllyJungleMinionsKilled.';
COMMENT ON COLUMN match_v5.participants.total_damage_dealt IS 'Original param name - totalDamageDealt.';
COMMENT ON COLUMN match_v5.participants.total_damage_dealt_to_champions IS 'Original param name - totalDamageDealtToChampions.';
COMMENT ON COLUMN match_v5.participants.total_damage_shielded_on_teammates IS 'Original param name - totalDamageShieldedOnTeammates.';
COMMENT ON COLUMN match_v5.participants.total_damage_taken IS 'Original param name - totalDamageTaken.';
COMMENT ON COLUMN match_v5.participants.total_enemy_jungle_minions_killed IS 'Original param name - totalEnemyJungleMinionsKilled.';
COMMENT ON COLUMN match_v5.participants.total_heal IS 'Whenever positive health is applied (which translates to all heals in the game but not things like regeneration), totalHeal is incremented by the amount of health received. This includes healing enemies, jungle monsters, yourself, etc. Original param name - totalHeal.';
COMMENT ON COLUMN match_v5.participants.total_heals_on_teammates IS 'Whenever positive health is applied (which translates to all heals in the game but not things like regeneration), totalHealsOnTeammates is incremented by the amount of health received.  This is post modified, so if you heal someone missing 5 health for 100 you will get +5 totalHealsOnTeammates. Original param name - totalHealsOnTeammates.';
COMMENT ON COLUMN match_v5.participants.total_minions_killed IS 'totalMillionsKilled &#x3D; mMinionsKilled, which is only incremented on kills of kTeamMinion, kMeleeLaneMinion, kSuperLaneMinion, kRangedLaneMinion and kSiegeLaneMinion. Original param name - totalMinionsKilled.';
COMMENT ON COLUMN match_v5.participants.total_time_cc_dealt IS 'Original param name - totalTimeCCDealt.';
COMMENT ON COLUMN match_v5.participants.total_time_spent_dead IS 'Original param name - totalTimeSpentDead.';
COMMENT ON COLUMN match_v5.participants.total_units_healed IS 'Original param name - totalUnitsHealed.';
COMMENT ON COLUMN match_v5.participants.triple_kills IS 'Original param name - tripleKills.';
COMMENT ON COLUMN match_v5.participants.true_damage_dealt IS 'Original param name - trueDamageDealt.';
COMMENT ON COLUMN match_v5.participants.true_damage_dealt_to_champions IS 'Original param name - trueDamageDealtToChampions.';
COMMENT ON COLUMN match_v5.participants.true_damage_taken IS 'Original param name - trueDamageTaken.';
COMMENT ON COLUMN match_v5.participants.turret_kills IS 'Original param name - turretKills.';
COMMENT ON COLUMN match_v5.participants.turret_takedowns IS 'Original param name - turretTakedowns.';
COMMENT ON COLUMN match_v5.participants.turrets_lost IS 'Original param name - turretsLost.';
COMMENT ON COLUMN match_v5.participants.unreal_kills IS 'Original param name - unrealKills.';
COMMENT ON COLUMN match_v5.participants.vision_score IS 'Original param name - visionScore.';
COMMENT ON COLUMN match_v5.participants.vision_cleared_pings IS 'Original param name - visionClearedPings.';
COMMENT ON COLUMN match_v5.participants.vision_wards_bought_in_game IS 'Original param name - visionWardsBoughtInGame.';
COMMENT ON COLUMN match_v5.participants.wards_killed IS 'Original param name - wardsKilled.';
COMMENT ON COLUMN match_v5.participants.wards_placed IS 'Original param name - wardsPlaced.';
COMMENT ON COLUMN match_v5.participants.bait_pings IS 'Original param name - baitPings.';
COMMENT ON COLUMN match_v5.participants.danger_pings IS 'https:..github.com.RiotGames.developer-relations.issues.870. Original param name - dangerPings.';
COMMENT ON COLUMN match_v5.participants.basic_pings IS 'https:..github.com.RiotGames.developer-relations.issues.814. Original param name - basicPings.';
COMMENT ON COLUMN match_v5.participants.player_augment5 IS 'Original param name - playerAugment5.';
COMMENT ON COLUMN match_v5.participants.player_augment6 IS 'Original param name - playerAugment6.';
COMMENT ON COLUMN match_v5.participants.riot_id_name IS 'Deprecated, use &#x60;riotIdGameName&#x60;. This field name was briefly used instead of &#x60;riotIdGameName&#x60;, prior to patch 14.5.. Original param name - riotIdName.';
COMMENT ON COLUMN match_v5.participants.retreat_pings IS 'https:..github.com.RiotGames.developer-relations.issues.814. Original param name - retreatPings.';
