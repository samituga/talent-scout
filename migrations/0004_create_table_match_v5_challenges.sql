CREATE TABLE match_v5.challenges
(
    match_id                                          TEXT NOT NULL,
    puuid                                             TEXT NOT NULL,

    x_12_assist_streak_count                          INTEGER,
    baron_buff_gold_advantage_over_threshold          INTEGER,
    control_ward_time_coverage_in_river_or_enemy_half DECIMAL(20, 9),
    earliest_baron                                    DECIMAL(20, 9),
    earliest_dragon_takedown                          DECIMAL(20, 9),
    earliest_elder_dragon                             DECIMAL(20, 9),
    early_laning_phase_gold_exp_advantage             DECIMAL(20, 9),
    faster_support_quest_completion                   BOOLEAN,
    fastest_legendary                                 DECIMAL(20, 9),
    had_afk_teammate                                  BOOLEAN,
    highest_champion_damage                           INTEGER,
    highest_crowd_control_score                       BOOLEAN,
    highest_ward_kills                                BOOLEAN,
    jungler_kills_early_jungle                        INTEGER,
    kills_on_laners_early_jungle_as_jungler           INTEGER,
    laning_phase_gold_exp_advantage                   BOOLEAN,
    legendary_count                                   INTEGER,
    max_cs_advantage_on_lane_opponent                 DECIMAL(20, 9),
    max_level_lead_lane_opponent                      INTEGER,
    most_wards_destroyed_one_sweeper                  INTEGER,
    mythic_item_used                                  INTEGER,
    played_champ_select_position                      BOOLEAN,
    solo_turrets_lategame                             INTEGER,
    takedowns_first25_minutes                         INTEGER,
    teleport_takedowns                                INTEGER,
    third_inhibitor_destroyed_time                    DECIMAL(20, 9),
    three_wards_one_sweeper_count                     INTEGER,
    vision_score_advantage_lane_opponent              DECIMAL(20, 9),
    infernal_scale_pickup                             INTEGER,
    fist_bump_participation                           INTEGER,
    void_monster_kill                                 INTEGER,
    ability_uses                                      INTEGER,
    aces_before15_minutes                             INTEGER,
    allied_jungle_monster_kills                       DECIMAL(20, 9),
    baron_takedowns                                   INTEGER,
    blast_cone_opposite_opponent_count                INTEGER,
    bounty_gold                                       DECIMAL(20, 9),
    buffs_stolen                                      INTEGER,
    complete_support_quest_in_time                    INTEGER,
    control_wards_placed                              INTEGER,
    damage_per_minute                                 DECIMAL(20, 9),
    damage_taken_on_team_percentage                   DECIMAL(20, 9),
    danced_with_rift_herald                           INTEGER,
    deaths_by_enemy_champs                            INTEGER,
    dodge_skill_shots_small_window                    INTEGER,
    double_aces                                       INTEGER,
    dragon_takedowns                                  INTEGER,
    legendary_item_used                               INTEGER[],
    effective_heal_and_shielding                      DECIMAL(20, 9),
    elder_dragon_kills_with_opposing_soul             INTEGER,
    elder_dragon_multikills                           INTEGER,
    enemy_champion_immobilizations                    INTEGER,
    enemy_jungle_monster_kills                        DECIMAL(20, 9),
    epic_monster_kills_near_enemy_jungler             INTEGER,
    epic_monster_kills_within30_seconds_of_spawn      INTEGER,
    epic_monster_steals                               INTEGER,
    epic_monster_stolen_without_smite                 INTEGER,
    first_turret_killed                               DECIMAL(20, 9),
    first_turret_killed_time                          DECIMAL(20, 9),
    flawless_aces                                     INTEGER,
    full_team_takedown                                INTEGER,
    game_length                                       DECIMAL(20, 9),
    get_takedowns_in_all_lanes_early_jungle_as_laner  INTEGER,
    gold_per_minute                                   DECIMAL(20, 9),
    had_open_nexus                                    INTEGER,
    immobilize_and_kill_with_ally                     INTEGER,
    initial_buff_count                                INTEGER,
    initial_crab_count                                INTEGER,
    jungle_cs_before10_minutes                        DECIMAL(20, 9),
    jungler_takedowns_near_damaged_epic_monster       INTEGER,
    kda                                               DECIMAL(20, 9),
    kill_after_hidden_with_ally                       INTEGER,
    killed_champ_took_full_team_damage_survived       INTEGER,
    killing_sprees                                    INTEGER,
    kill_participation                                DECIMAL(20, 9),
    kills_near_enemy_turret                           INTEGER,
    kills_on_other_lanes_early_jungle_as_laner        INTEGER,
    kills_on_recently_healed_by_aram_pack             INTEGER,
    kills_under_own_turret                            INTEGER,
    kills_with_help_from_epic_monster                 INTEGER,
    knock_enemy_into_team_and_kill                    INTEGER,
    k_turrets_destroyed_before_plates_fall            INTEGER,
    land_skill_shots_early_game                       INTEGER,
    lane_minions_first10_minutes                      INTEGER,
    lost_an_inhibitor                                 INTEGER,
    max_kill_deficit                                  INTEGER,
    mejais_full_stack_in_time                         INTEGER,
    more_enemy_jungle_than_opponent                   DECIMAL(20, 9),
    multi_kill_one_spell                              INTEGER,
    multikills                                        INTEGER,
    multikills_after_aggressive_flash                 INTEGER,
    multi_turret_rift_herald_count                    INTEGER,
    outer_turret_executes_before10_minutes            INTEGER,
    outnumbered_kills                                 INTEGER,
    outnumbered_nexus_kill                            INTEGER,
    perfect_dragon_souls_taken                        INTEGER,
    perfect_game                                      INTEGER,
    pick_kill_with_ally                               INTEGER,
    poro_explosions                                   INTEGER,
    quick_cleanse                                     INTEGER,
    quick_first_turret                                INTEGER,
    quick_solo_kills                                  INTEGER,
    rift_herald_takedowns                             INTEGER,
    save_ally_from_death                              INTEGER,
    scuttle_crab_kills                                INTEGER,
    shortest_time_to_ace_from_first_takedown          DECIMAL(20, 9),
    skillshots_dodged                                 INTEGER,
    skillshots_hit                                    INTEGER,
    snowballs_hit                                     INTEGER,
    solo_baron_kills                                  INTEGER,
    swarm_defeat_aatrox                               INTEGER,
    swarm_defeat_briar                                INTEGER,
    swarm_defeat_mini_bosses                          INTEGER,
    swarm_evolve_weapon                               INTEGER,
    swarm_have3_passives                              INTEGER,
    swarm_kill_enemy                                  INTEGER,
    swarm_pickup_gold                                 DECIMAL(20, 9),
    swarm_reach_level50                               INTEGER,
    swarm_survive15_min                               INTEGER,
    swarm_win_with5_evolved_weapons                   INTEGER,
    solo_kills                                        INTEGER,
    stealth_wards_placed                              INTEGER,
    survived_single_digit_hp_count                    INTEGER,
    survived_three_immobilizes_in_fight               INTEGER,
    takedown_on_first_turret                          INTEGER,
    takedowns                                         INTEGER,
    takedowns_after_gaining_level_advantage           INTEGER,
    takedowns_before_jungle_minion_spawn              INTEGER,
    takedowns_first_x_minutes                         INTEGER,
    takedowns_in_alcove                               INTEGER,
    takedowns_in_enemy_fountain                       INTEGER,
    team_baron_kills                                  INTEGER,
    team_damage_percentage                            DECIMAL(20, 9),
    team_elder_dragon_kills                           INTEGER,
    team_rift_herald_kills                            INTEGER,
    took_large_damage_survived                        INTEGER,
    turret_plates_taken                               INTEGER,
    turrets_taken_with_rift_herald                    INTEGER,
    turret_takedowns                                  INTEGER,
    twenty_minions_in3_seconds_count                  INTEGER,
    two_wards_one_sweeper_count                       INTEGER,
    unseen_recalls                                    INTEGER,
    vision_score_per_minute                           DECIMAL(20, 9),
    wards_guarded                                     INTEGER,
    ward_takedowns                                    INTEGER,
    ward_takedowns_before20_m                         INTEGER,
    heal_from_map_sources                             DECIMAL(20, 9),

    PRIMARY KEY (match_id, puuid),
    FOREIGN KEY (match_id, puuid)
        REFERENCES match_v5.participants (match_id, puuid)
);

COMMENT ON TABLE match_v5.challenges IS 'Challenges DTO. Original model name - match-v5.ChallengesDto.';

COMMENT ON COLUMN match_v5.challenges.x_12_assist_streak_count IS 'Original param name - 12AssistStreakCount.';
COMMENT ON COLUMN match_v5.challenges.baron_buff_gold_advantage_over_threshold IS 'Original param name - baronBuffGoldAdvantageOverThreshold.';
COMMENT ON COLUMN match_v5.challenges.control_ward_time_coverage_in_river_or_enemy_half IS 'Original param name - controlWardTimeCoverageInRiverOrEnemyHalf.';
COMMENT ON COLUMN match_v5.challenges.earliest_baron IS 'Original param name - earliestBaron.';
COMMENT ON COLUMN match_v5.challenges.earliest_dragon_takedown IS 'Original param name - earliestDragonTakedown.';
COMMENT ON COLUMN match_v5.challenges.earliest_elder_dragon IS 'Original param name - earliestElderDragon.';
COMMENT ON COLUMN match_v5.challenges.early_laning_phase_gold_exp_advantage IS 'Original param name - earlyLaningPhaseGoldExpAdvantage.';
COMMENT ON COLUMN match_v5.challenges.faster_support_quest_completion IS 'Original param name - fasterSupportQuestCompletion.';
COMMENT ON COLUMN match_v5.challenges.fastest_legendary IS 'Original param name - fastestLegendary.';
COMMENT ON COLUMN match_v5.challenges.had_afk_teammate IS 'Original param name - hadAfkTeammate.';
COMMENT ON COLUMN match_v5.challenges.highest_champion_damage IS 'Original param name - highestChampionDamage.';
COMMENT ON COLUMN match_v5.challenges.highest_crowd_control_score IS 'Original param name - highestCrowdControlScore.';
COMMENT ON COLUMN match_v5.challenges.highest_ward_kills IS 'Original param name - highestWardKills.';
COMMENT ON COLUMN match_v5.challenges.jungler_kills_early_jungle IS 'Original param name - junglerKillsEarlyJungle.';
COMMENT ON COLUMN match_v5.challenges.kills_on_laners_early_jungle_as_jungler IS 'Original param name - killsOnLanersEarlyJungleAsJungler.';
COMMENT ON COLUMN match_v5.challenges.laning_phase_gold_exp_advantage IS 'Original param name - laningPhaseGoldExpAdvantage.';
COMMENT ON COLUMN match_v5.challenges.legendary_count IS 'Original param name - legendaryCount.';
COMMENT ON COLUMN match_v5.challenges.max_cs_advantage_on_lane_opponent IS 'Original param name - maxCsAdvantageOnLaneOpponent.';
COMMENT ON COLUMN match_v5.challenges.max_level_lead_lane_opponent IS 'Original param name - maxLevelLeadLaneOpponent.';
COMMENT ON COLUMN match_v5.challenges.most_wards_destroyed_one_sweeper IS 'Original param name - mostWardsDestroyedOneSweeper.';
COMMENT ON COLUMN match_v5.challenges.mythic_item_used IS 'Original param name - mythicItemUsed.';
COMMENT ON COLUMN match_v5.challenges.played_champ_select_position IS 'Original param name - playedChampSelectPosition.';
COMMENT ON COLUMN match_v5.challenges.solo_turrets_lategame IS 'Original param name - soloTurretsLategame.';
COMMENT ON COLUMN match_v5.challenges.takedowns_first25_minutes IS 'Original param name - takedownsFirst25Minutes.';
COMMENT ON COLUMN match_v5.challenges.teleport_takedowns IS 'Original param name - teleportTakedowns.';
COMMENT ON COLUMN match_v5.challenges.third_inhibitor_destroyed_time IS 'Original param name - thirdInhibitorDestroyedTime.';
COMMENT ON COLUMN match_v5.challenges.three_wards_one_sweeper_count IS 'Original param name - threeWardsOneSweeperCount.';
COMMENT ON COLUMN match_v5.challenges.vision_score_advantage_lane_opponent IS 'Original param name - visionScoreAdvantageLaneOpponent.';
COMMENT ON COLUMN match_v5.challenges.infernal_scale_pickup IS 'Original param name - InfernalScalePickup.';
COMMENT ON COLUMN match_v5.challenges.fist_bump_participation IS 'Original param name - fistBumpParticipation.';
COMMENT ON COLUMN match_v5.challenges.void_monster_kill IS 'Original param name - voidMonsterKill.';
COMMENT ON COLUMN match_v5.challenges.ability_uses IS 'Original param name - abilityUses.';
COMMENT ON COLUMN match_v5.challenges.aces_before15_minutes IS 'Original param name - acesBefore15Minutes.';
COMMENT ON COLUMN match_v5.challenges.allied_jungle_monster_kills IS 'Original param name - alliedJungleMonsterKills.';
COMMENT ON COLUMN match_v5.challenges.baron_takedowns IS 'Original param name - baronTakedowns.';
COMMENT ON COLUMN match_v5.challenges.blast_cone_opposite_opponent_count IS 'Original param name - blastConeOppositeOpponentCount.';
COMMENT ON COLUMN match_v5.challenges.bounty_gold IS 'Original param name - bountyGold.';
COMMENT ON COLUMN match_v5.challenges.buffs_stolen IS 'Original param name - buffsStolen.';
COMMENT ON COLUMN match_v5.challenges.complete_support_quest_in_time IS 'Original param name - completeSupportQuestInTime.';
COMMENT ON COLUMN match_v5.challenges.control_wards_placed IS 'Original param name - controlWardsPlaced.';
COMMENT ON COLUMN match_v5.challenges.damage_per_minute IS 'Original param name - damagePerMinute.';
COMMENT ON COLUMN match_v5.challenges.damage_taken_on_team_percentage IS 'Original param name - damageTakenOnTeamPercentage.';
COMMENT ON COLUMN match_v5.challenges.danced_with_rift_herald IS 'Original param name - dancedWithRiftHerald.';
COMMENT ON COLUMN match_v5.challenges.deaths_by_enemy_champs IS 'Original param name - deathsByEnemyChamps.';
COMMENT ON COLUMN match_v5.challenges.dodge_skill_shots_small_window IS 'Original param name - dodgeSkillShotsSmallWindow.';
COMMENT ON COLUMN match_v5.challenges.double_aces IS 'Original param name - doubleAces.';
COMMENT ON COLUMN match_v5.challenges.dragon_takedowns IS 'Original param name - dragonTakedowns.';
COMMENT ON COLUMN match_v5.challenges.legendary_item_used IS 'Original param name - legendaryItemUsed.';
COMMENT ON COLUMN match_v5.challenges.effective_heal_and_shielding IS 'Original param name - effectiveHealAndShielding.';
COMMENT ON COLUMN match_v5.challenges.elder_dragon_kills_with_opposing_soul IS 'Original param name - elderDragonKillsWithOpposingSoul.';
COMMENT ON COLUMN match_v5.challenges.elder_dragon_multikills IS 'Original param name - elderDragonMultikills.';
COMMENT ON COLUMN match_v5.challenges.enemy_champion_immobilizations IS 'Original param name - enemyChampionImmobilizations.';
COMMENT ON COLUMN match_v5.challenges.enemy_jungle_monster_kills IS 'Original param name - enemyJungleMonsterKills.';
COMMENT ON COLUMN match_v5.challenges.epic_monster_kills_near_enemy_jungler IS 'Original param name - epicMonsterKillsNearEnemyJungler.';
COMMENT ON COLUMN match_v5.challenges.epic_monster_kills_within30_seconds_of_spawn IS 'Original param name - epicMonsterKillsWithin30SecondsOfSpawn.';
COMMENT ON COLUMN match_v5.challenges.epic_monster_steals IS 'Original param name - epicMonsterSteals.';
COMMENT ON COLUMN match_v5.challenges.epic_monster_stolen_without_smite IS 'Original param name - epicMonsterStolenWithoutSmite.';
COMMENT ON COLUMN match_v5.challenges.first_turret_killed IS 'Original param name - firstTurretKilled.';
COMMENT ON COLUMN match_v5.challenges.first_turret_killed_time IS 'Original param name - firstTurretKilledTime.';
COMMENT ON COLUMN match_v5.challenges.flawless_aces IS 'Original param name - flawlessAces.';
COMMENT ON COLUMN match_v5.challenges.full_team_takedown IS 'Original param name - fullTeamTakedown.';
COMMENT ON COLUMN match_v5.challenges.game_length IS 'Original param name - gameLength.';
COMMENT ON COLUMN match_v5.challenges.get_takedowns_in_all_lanes_early_jungle_as_laner IS 'Original param name - getTakedownsInAllLanesEarlyJungleAsLaner.';
COMMENT ON COLUMN match_v5.challenges.gold_per_minute IS 'Original param name - goldPerMinute.';
COMMENT ON COLUMN match_v5.challenges.had_open_nexus IS 'Original param name - hadOpenNexus.';
COMMENT ON COLUMN match_v5.challenges.immobilize_and_kill_with_ally IS 'Original param name - immobilizeAndKillWithAlly.';
COMMENT ON COLUMN match_v5.challenges.initial_buff_count IS 'Original param name - initialBuffCount.';
COMMENT ON COLUMN match_v5.challenges.initial_crab_count IS 'Original param name - initialCrabCount.';
COMMENT ON COLUMN match_v5.challenges.jungle_cs_before10_minutes IS 'Original param name - jungleCsBefore10Minutes.';
COMMENT ON COLUMN match_v5.challenges.jungler_takedowns_near_damaged_epic_monster IS 'Original param name - junglerTakedownsNearDamagedEpicMonster.';
COMMENT ON COLUMN match_v5.challenges.kill_after_hidden_with_ally IS 'Original param name - killAfterHiddenWithAlly.';
COMMENT ON COLUMN match_v5.challenges.killed_champ_took_full_team_damage_survived IS 'Original param name - killedChampTookFullTeamDamageSurvived.';
COMMENT ON COLUMN match_v5.challenges.killing_sprees IS 'Original param name - killingSprees.';
COMMENT ON COLUMN match_v5.challenges.kill_participation IS 'Original param name - killParticipation.';
COMMENT ON COLUMN match_v5.challenges.kills_near_enemy_turret IS 'Original param name - killsNearEnemyTurret.';
COMMENT ON COLUMN match_v5.challenges.kills_on_other_lanes_early_jungle_as_laner IS 'Original param name - killsOnOtherLanesEarlyJungleAsLaner.';
COMMENT ON COLUMN match_v5.challenges.kills_on_recently_healed_by_aram_pack IS 'Original param name - killsOnRecentlyHealedByAramPack.';
COMMENT ON COLUMN match_v5.challenges.kills_under_own_turret IS 'Original param name - killsUnderOwnTurret.';
COMMENT ON COLUMN match_v5.challenges.kills_with_help_from_epic_monster IS 'Original param name - killsWithHelpFromEpicMonster.';
COMMENT ON COLUMN match_v5.challenges.knock_enemy_into_team_and_kill IS 'Original param name - knockEnemyIntoTeamAndKill.';
COMMENT ON COLUMN match_v5.challenges.k_turrets_destroyed_before_plates_fall IS 'Original param name - kTurretsDestroyedBeforePlatesFall.';
COMMENT ON COLUMN match_v5.challenges.land_skill_shots_early_game IS 'Original param name - landSkillShotsEarlyGame.';
COMMENT ON COLUMN match_v5.challenges.lane_minions_first10_minutes IS 'Original param name - laneMinionsFirst10Minutes.';
COMMENT ON COLUMN match_v5.challenges.lost_an_inhibitor IS 'Original param name - lostAnInhibitor.';
COMMENT ON COLUMN match_v5.challenges.max_kill_deficit IS 'Original param name - maxKillDeficit.';
COMMENT ON COLUMN match_v5.challenges.mejais_full_stack_in_time IS 'Original param name - mejaisFullStackInTime.';
COMMENT ON COLUMN match_v5.challenges.more_enemy_jungle_than_opponent IS 'Original param name - moreEnemyJungleThanOpponent.';
COMMENT ON COLUMN match_v5.challenges.multi_kill_one_spell IS 'This is an offshoot of the OneStone challenge. The code checks if a spell with the same instance ID does the final point of damage to at least 2 Champions. It doesn&#39;t matter if they&#39;re enemies, but you cannot hurt your friends.. Original param name - multiKillOneSpell.';
COMMENT ON COLUMN match_v5.challenges.multikills_after_aggressive_flash IS 'Original param name - multikillsAfterAggressiveFlash.';
COMMENT ON COLUMN match_v5.challenges.multi_turret_rift_herald_count IS 'Original param name - multiTurretRiftHeraldCount.';
COMMENT ON COLUMN match_v5.challenges.outer_turret_executes_before10_minutes IS 'Original param name - outerTurretExecutesBefore10Minutes.';
COMMENT ON COLUMN match_v5.challenges.outnumbered_kills IS 'Original param name - outnumberedKills.';
COMMENT ON COLUMN match_v5.challenges.outnumbered_nexus_kill IS 'Original param name - outnumberedNexusKill.';
COMMENT ON COLUMN match_v5.challenges.perfect_dragon_souls_taken IS 'Original param name - perfectDragonSoulsTaken.';
COMMENT ON COLUMN match_v5.challenges.perfect_game IS 'Original param name - perfectGame.';
COMMENT ON COLUMN match_v5.challenges.pick_kill_with_ally IS 'Original param name - pickKillWithAlly.';
COMMENT ON COLUMN match_v5.challenges.poro_explosions IS 'Original param name - poroExplosions.';
COMMENT ON COLUMN match_v5.challenges.quick_cleanse IS 'Original param name - quickCleanse.';
COMMENT ON COLUMN match_v5.challenges.quick_first_turret IS 'Original param name - quickFirstTurret.';
COMMENT ON COLUMN match_v5.challenges.quick_solo_kills IS 'Original param name - quickSoloKills.';
COMMENT ON COLUMN match_v5.challenges.rift_herald_takedowns IS 'Original param name - riftHeraldTakedowns.';
COMMENT ON COLUMN match_v5.challenges.save_ally_from_death IS 'Original param name - saveAllyFromDeath.';
COMMENT ON COLUMN match_v5.challenges.scuttle_crab_kills IS 'Original param name - scuttleCrabKills.';
COMMENT ON COLUMN match_v5.challenges.shortest_time_to_ace_from_first_takedown IS 'Original param name - shortestTimeToAceFromFirstTakedown.';
COMMENT ON COLUMN match_v5.challenges.skillshots_dodged IS 'Original param name - skillshotsDodged.';
COMMENT ON COLUMN match_v5.challenges.skillshots_hit IS 'Original param name - skillshotsHit.';
COMMENT ON COLUMN match_v5.challenges.snowballs_hit IS 'Original param name - snowballsHit.';
COMMENT ON COLUMN match_v5.challenges.solo_baron_kills IS 'Original param name - soloBaronKills.';
COMMENT ON COLUMN match_v5.challenges.swarm_defeat_aatrox IS 'Original param name - SWARM_DefeatAatrox.';
COMMENT ON COLUMN match_v5.challenges.swarm_defeat_briar IS 'Original param name - SWARM_DefeatBriar.';
COMMENT ON COLUMN match_v5.challenges.swarm_defeat_mini_bosses IS 'Original param name - SWARM_DefeatMiniBosses.';
COMMENT ON COLUMN match_v5.challenges.swarm_evolve_weapon IS 'Original param name - SWARM_EvolveWeapon.';
COMMENT ON COLUMN match_v5.challenges.swarm_have3_passives IS 'Original param name - SWARM_Have3Passives.';
COMMENT ON COLUMN match_v5.challenges.swarm_kill_enemy IS 'Original param name - SWARM_KillEnemy.';
COMMENT ON COLUMN match_v5.challenges.swarm_pickup_gold IS 'Original param name - SWARM_PickupGold.';
COMMENT ON COLUMN match_v5.challenges.swarm_reach_level50 IS 'Original param name - SWARM_ReachLevel50.';
COMMENT ON COLUMN match_v5.challenges.swarm_survive15_min IS 'Original param name - SWARM_Survive15Min.';
COMMENT ON COLUMN match_v5.challenges.swarm_win_with5_evolved_weapons IS 'Original param name - SWARM_WinWith5EvolvedWeapons.';
COMMENT ON COLUMN match_v5.challenges.solo_kills IS 'Original param name - soloKills.';
COMMENT ON COLUMN match_v5.challenges.stealth_wards_placed IS 'Original param name - stealthWardsPlaced.';
COMMENT ON COLUMN match_v5.challenges.survived_single_digit_hp_count IS 'Original param name - survivedSingleDigitHpCount.';
COMMENT ON COLUMN match_v5.challenges.survived_three_immobilizes_in_fight IS 'Original param name - survivedThreeImmobilizesInFight.';
COMMENT ON COLUMN match_v5.challenges.takedown_on_first_turret IS 'Original param name - takedownOnFirstTurret.';
COMMENT ON COLUMN match_v5.challenges.takedowns_after_gaining_level_advantage IS 'Original param name - takedownsAfterGainingLevelAdvantage.';
COMMENT ON COLUMN match_v5.challenges.takedowns_before_jungle_minion_spawn IS 'Original param name - takedownsBeforeJungleMinionSpawn.';
COMMENT ON COLUMN match_v5.challenges.takedowns_first_x_minutes IS 'Original param name - takedownsFirstXMinutes.';
COMMENT ON COLUMN match_v5.challenges.takedowns_in_alcove IS 'Original param name - takedownsInAlcove.';
COMMENT ON COLUMN match_v5.challenges.takedowns_in_enemy_fountain IS 'Original param name - takedownsInEnemyFountain.';
COMMENT ON COLUMN match_v5.challenges.team_baron_kills IS 'Original param name - teamBaronKills.';
COMMENT ON COLUMN match_v5.challenges.team_damage_percentage IS 'Original param name - teamDamagePercentage.';
COMMENT ON COLUMN match_v5.challenges.team_elder_dragon_kills IS 'Original param name - teamElderDragonKills.';
COMMENT ON COLUMN match_v5.challenges.team_rift_herald_kills IS 'Original param name - teamRiftHeraldKills.';
COMMENT ON COLUMN match_v5.challenges.took_large_damage_survived IS 'Original param name - tookLargeDamageSurvived.';
COMMENT ON COLUMN match_v5.challenges.turret_plates_taken IS 'Original param name - turretPlatesTaken.';
COMMENT ON COLUMN match_v5.challenges.turrets_taken_with_rift_herald IS 'Any player who damages a tower that is destroyed within 30 seconds of a Rift Herald charge will receive credit. A player who does not damage the tower will not receive credit.. Original param name - turretsTakenWithRiftHerald.';
COMMENT ON COLUMN match_v5.challenges.turret_takedowns IS 'Original param name - turretTakedowns.';
COMMENT ON COLUMN match_v5.challenges.twenty_minions_in3_seconds_count IS 'Original param name - twentyMinionsIn3SecondsCount.';
COMMENT ON COLUMN match_v5.challenges.two_wards_one_sweeper_count IS 'Original param name - twoWardsOneSweeperCount.';
COMMENT ON COLUMN match_v5.challenges.unseen_recalls IS 'Original param name - unseenRecalls.';
COMMENT ON COLUMN match_v5.challenges.vision_score_per_minute IS 'Original param name - visionScorePerMinute.';
COMMENT ON COLUMN match_v5.challenges.wards_guarded IS 'Original param name - wardsGuarded.';
COMMENT ON COLUMN match_v5.challenges.ward_takedowns IS 'Original param name - wardTakedowns.';
COMMENT ON COLUMN match_v5.challenges.ward_takedowns_before20_m IS 'Original param name - wardTakedownsBefore20M.';
COMMENT ON COLUMN match_v5.challenges.heal_from_map_sources IS 'Original param name - HealFromMapSources.';
