// Generated using https://github.com/a2x/cs2-dumper
// 2024-10-20 16:30:59.112920300 UTC

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod cs2_dumper {
    pub mod schemas {
        // Module: client.dll
        // Class count: 420
        // Enum count: 7
        pub mod client_dll {
            // Alignment: 4
            // Member count: 15
            #[repr(u32)]
            pub enum CompositeMaterialInputLooseVariableType_t {
                LOOSE_VARIABLE_TYPE_BOOLEAN = 0x0,
                LOOSE_VARIABLE_TYPE_INTEGER1 = 0x1,
                LOOSE_VARIABLE_TYPE_INTEGER2 = 0x2,
                LOOSE_VARIABLE_TYPE_INTEGER3 = 0x3,
                LOOSE_VARIABLE_TYPE_INTEGER4 = 0x4,
                LOOSE_VARIABLE_TYPE_FLOAT1 = 0x5,
                LOOSE_VARIABLE_TYPE_FLOAT2 = 0x6,
                LOOSE_VARIABLE_TYPE_FLOAT3 = 0x7,
                LOOSE_VARIABLE_TYPE_FLOAT4 = 0x8,
                LOOSE_VARIABLE_TYPE_COLOR4 = 0x9,
                LOOSE_VARIABLE_TYPE_STRING = 0xA,
                LOOSE_VARIABLE_TYPE_SYSTEMVAR = 0xB,
                LOOSE_VARIABLE_TYPE_RESOURCE_MATERIAL = 0xC,
                LOOSE_VARIABLE_TYPE_RESOURCE_TEXTURE = 0xD,
                LOOSE_VARIABLE_TYPE_PANORAMA_RENDER = 0xE
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum CompositeMaterialInputTextureType_t {
                INPUT_TEXTURE_TYPE_DEFAULT = 0x0,
                INPUT_TEXTURE_TYPE_NORMALMAP = 0x1,
                INPUT_TEXTURE_TYPE_COLOR = 0x2,
                INPUT_TEXTURE_TYPE_MASKS = 0x3,
                INPUT_TEXTURE_TYPE_ROUGHNESS = 0x4,
                INPUT_TEXTURE_TYPE_PEARLESCENCE_MASK = 0x5,
                INPUT_TEXTURE_TYPE_AO = 0x6
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum CompositeMaterialInputContainerSourceType_t {
                CONTAINER_SOURCE_TYPE_TARGET_MATERIAL = 0x0,
                CONTAINER_SOURCE_TYPE_MATERIAL_FROM_TARGET_ATTR = 0x1,
                CONTAINER_SOURCE_TYPE_SPECIFIC_MATERIAL = 0x2,
                CONTAINER_SOURCE_TYPE_LOOSE_VARIABLES = 0x3,
                CONTAINER_SOURCE_TYPE_VARIABLE_FROM_TARGET_ATTR = 0x4,
                CONTAINER_SOURCE_TYPE_TARGET_INSTANCE_MATERIAL = 0x5
            }
            // Alignment: 4
            // Member count: 10
            #[repr(u32)]
            pub enum CompMatPropertyMutatorType_t {
                COMP_MAT_PROPERTY_MUTATOR_INIT = 0x0,
                COMP_MAT_PROPERTY_MUTATOR_COPY_MATCHING_KEYS = 0x1,
                COMP_MAT_PROPERTY_MUTATOR_COPY_KEYS_WITH_SUFFIX = 0x2,
                COMP_MAT_PROPERTY_MUTATOR_COPY_PROPERTY = 0x3,
                COMP_MAT_PROPERTY_MUTATOR_SET_VALUE = 0x4,
                COMP_MAT_PROPERTY_MUTATOR_GENERATE_TEXTURE = 0x5,
                COMP_MAT_PROPERTY_MUTATOR_CONDITIONAL_MUTATORS = 0x6,
                COMP_MAT_PROPERTY_MUTATOR_POP_INPUT_QUEUE = 0x7,
                COMP_MAT_PROPERTY_MUTATOR_DRAW_TEXT = 0x8,
                COMP_MAT_PROPERTY_MUTATOR_RANDOM_ROLL_INPUT_VARIABLES = 0x9
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum CompositeMaterialVarSystemVar_t {
                COMPMATSYSVAR_COMPOSITETIME = 0x0,
                COMPMATSYSVAR_EMPTY_RESOURCE_SPACER = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum CompositeMaterialMatchFilterType_t {
                MATCH_FILTER_MATERIAL_ATTRIBUTE_EXISTS = 0x0,
                MATCH_FILTER_MATERIAL_SHADER = 0x1,
                MATCH_FILTER_MATERIAL_NAME_SUBSTR = 0x2,
                MATCH_FILTER_MATERIAL_ATTRIBUTE_EQUALS = 0x3,
                MATCH_FILTER_MATERIAL_PROPERTY_EXISTS = 0x4,
                MATCH_FILTER_MATERIAL_PROPERTY_EQUALS = 0x5
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum CompMatPropertyMutatorConditionType_t {
                COMP_MAT_MUTATOR_CONDITION_INPUT_CONTAINER_EXISTS = 0x0,
                COMP_MAT_MUTATOR_CONDITION_INPUT_CONTAINER_VALUE_EXISTS = 0x1,
                COMP_MAT_MUTATOR_CONDITION_INPUT_CONTAINER_VALUE_EQUALS = 0x2
            }
            // Parent: C_CSGO_TeamPreviewCharacterPosition
            // Field count: 0
            pub mod C_CSGO_TeamSelectCharacterPosition {
            }
            // Parent: C_BaseCSGrenade
            // Field count: 0
            pub mod C_HEGrenade {
            }
            // Parent: C_BaseToggle
            // Field count: 0
            pub mod C_FuncMover {
            }
            // Parent: C_BaseTrigger
            // Field count: 0
            pub mod C_TriggerLerpObject {
            }
            // Parent: CPlayer_MovementServices_Humanoid
            // Field count: 39
            //
            // Metadata:
            // NetworkVarNames: m_flMaxFallVelocity (float)
            // NetworkVarNames: m_vecLadderNormal (Vector)
            // NetworkVarNames: m_nLadderSurfacePropIndex (int)
            // NetworkVarNames: m_flDuckAmount (float)
            // NetworkVarNames: m_flDuckSpeed (float)
            // NetworkVarNames: m_bDuckOverride (bool)
            // NetworkVarNames: m_bDesiresDuck (bool)
            // NetworkVarNames: m_nDuckTimeMsecs (uint32)
            // NetworkVarNames: m_nDuckJumpTimeMsecs (uint32)
            // NetworkVarNames: m_nJumpTimeMsecs (uint32)
            // NetworkVarNames: m_flLastDuckTime (float)
            // NetworkVarNames: m_nGameCodeHasMovedPlayerAfterCommand (int)
            // NetworkVarNames: m_bOldJumpPressed (bool)
            // NetworkVarNames: m_flJumpUntil (float)
            // NetworkVarNames: m_flJumpVel (float)
            // NetworkVarNames: m_fStashGrenadeParameterWhen (GameTime_t)
            // NetworkVarNames: m_nButtonDownMaskPrev (ButtonBitMask_t)
            // NetworkVarNames: m_flOffsetTickCompleteTime (float)
            // NetworkVarNames: m_flOffsetTickStashedSpeed (float)
            // NetworkVarNames: m_flStamina (float)
            pub mod CCSPlayer_MovementServices {
                pub const m_flMaxFallVelocity: usize = 0x218; // 
                pub const m_vecLadderNormal: usize = 0x21C; // 
                pub const m_nLadderSurfacePropIndex: usize = 0x228; // 
                pub const m_flDuckAmount: usize = 0x22C; // 
                pub const m_flDuckSpeed: usize = 0x230; // 
                pub const m_bDuckOverride: usize = 0x234; // 
                pub const m_bDesiresDuck: usize = 0x235; // 
                pub const m_flDuckOffset: usize = 0x238; // 
                pub const m_nDuckTimeMsecs: usize = 0x23C; // 
                pub const m_nDuckJumpTimeMsecs: usize = 0x240; // 
                pub const m_nJumpTimeMsecs: usize = 0x244; // 
                pub const m_flLastDuckTime: usize = 0x248; // 
                pub const m_vecLastPositionAtFullCrouchSpeed: usize = 0x258; // 
                pub const m_duckUntilOnGround: usize = 0x260; // 
                pub const m_bHasWalkMovedSinceLastJump: usize = 0x261; // 
                pub const m_bInStuckTest: usize = 0x262; // 
                pub const m_flStuckCheckTime: usize = 0x270; // 
                pub const m_nTraceCount: usize = 0x470; // 
                pub const m_StuckLast: usize = 0x474; // 
                pub const m_bSpeedCropped: usize = 0x478; // 
                pub const m_flGroundMoveEfficiency: usize = 0x47C; // 
                pub const m_nOldWaterLevel: usize = 0x480; // 
                pub const m_flWaterEntryTime: usize = 0x484; // 
                pub const m_vecForward: usize = 0x488; // 
                pub const m_vecLeft: usize = 0x494; // 
                pub const m_vecUp: usize = 0x4A0; // 
                pub const m_nGameCodeHasMovedPlayerAfterCommand: usize = 0x4AC; // 
                pub const m_bOldJumpPressed: usize = 0x4B0; // 
                pub const m_flJumpPressedTime: usize = 0x4B4; // 
                pub const m_flJumpUntil: usize = 0x4B8; // 
                pub const m_flJumpVel: usize = 0x4BC; // 
                pub const m_fStashGrenadeParameterWhen: usize = 0x4C0; // 
                pub const m_nButtonDownMaskPrev: usize = 0x4C8; // 
                pub const m_flOffsetTickCompleteTime: usize = 0x4D0; // 
                pub const m_flOffsetTickStashedSpeed: usize = 0x4D4; // 
                pub const m_flStamina: usize = 0x4D8; // 
                pub const m_flHeightAtJumpStart: usize = 0x4DC; // 
                pub const m_flMaxJumpHeightThisJump: usize = 0x4E0; // 
                pub const m_flMaxJumpHeightLastJump: usize = 0x4E4; // 
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // NetworkVarNames: scale (int16)
            // NetworkVarNames: origin (Vector)
            // NetworkVarNames: bClip3DSkyBoxNearToWorldFar (bool)
            // NetworkVarNames: flClip3DSkyBoxNearToWorldFarOffset (float32)
            // NetworkVarNames: fog (fogparams_t)
            // NetworkVarNames: m_nWorldGroupID (WorldGroupId_t)
            pub mod sky3dparams_t {
                pub const scale: usize = 0x8; // 
                pub const origin: usize = 0xC; // 
                pub const bClip3DSkyBoxNearToWorldFar: usize = 0x18; // 
                pub const flClip3DSkyBoxNearToWorldFarOffset: usize = 0x1C; // 
                pub const fog: usize = 0x20; // 
                pub const m_nWorldGroupID: usize = 0x88; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponM4A1 {
            }
            // Parent: C_EconEntity
            // Field count: 1
            pub mod C_Item {
                pub const m_pReticleHintTextName: usize = 0x1668; // 
            }
            // Parent: C_SoundEventEntity
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_vecCornerPairsNetworked (SoundeventPathCornerPairNetworked_t)
            pub mod C_SoundEventPathCornerEntity {
                pub const m_vecCornerPairsNetworked: usize = 0x628; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponM249 {
            }
            // Parent: C_TeamplayRules
            // Field count: 100
            //
            // Metadata:
            // NetworkVarNames: m_bFreezePeriod (bool)
            // NetworkVarNames: m_bWarmupPeriod (bool)
            // NetworkVarNames: m_fWarmupPeriodEnd (GameTime_t)
            // NetworkVarNames: m_fWarmupPeriodStart (GameTime_t)
            // NetworkVarNames: m_bServerPaused (bool)
            // NetworkVarNames: m_bTerroristTimeOutActive (bool)
            // NetworkVarNames: m_bCTTimeOutActive (bool)
            // NetworkVarNames: m_flTerroristTimeOutRemaining (float)
            // NetworkVarNames: m_flCTTimeOutRemaining (float)
            // NetworkVarNames: m_nTerroristTimeOuts (int)
            // NetworkVarNames: m_nCTTimeOuts (int)
            // NetworkVarNames: m_bTechnicalTimeOut (bool)
            // NetworkVarNames: m_bMatchWaitingForResume (bool)
            // NetworkVarNames: m_iRoundTime (int)
            // NetworkVarNames: m_fMatchStartTime (float)
            // NetworkVarNames: m_fRoundStartTime (GameTime_t)
            // NetworkVarNames: m_flRestartRoundTime (GameTime_t)
            // NetworkVarNames: m_bGameRestart (bool)
            // NetworkVarNames: m_flGameStartTime (float)
            // NetworkVarNames: m_timeUntilNextPhaseStarts (float)
            // NetworkVarNames: m_gamePhase (int)
            // NetworkVarNames: m_totalRoundsPlayed (int)
            // NetworkVarNames: m_nRoundsPlayedThisPhase (int)
            // NetworkVarNames: m_nOvertimePlaying (int)
            // NetworkVarNames: m_iHostagesRemaining (int)
            // NetworkVarNames: m_bAnyHostageReached (bool)
            // NetworkVarNames: m_bMapHasBombTarget (bool)
            // NetworkVarNames: m_bMapHasRescueZone (bool)
            // NetworkVarNames: m_bMapHasBuyZone (bool)
            // NetworkVarNames: m_bIsQueuedMatchmaking (bool)
            // NetworkVarNames: m_nQueuedMatchmakingMode (int)
            // NetworkVarNames: m_bIsValveDS (bool)
            // NetworkVarNames: m_bLogoMap (bool)
            // NetworkVarNames: m_bPlayAllStepSoundsOnServer (bool)
            // NetworkVarNames: m_iSpectatorSlotCount (int)
            // NetworkVarNames: m_MatchDevice (int)
            // NetworkVarNames: m_bHasMatchStarted (bool)
            // NetworkVarNames: m_nNextMapInMapgroup (int)
            // NetworkVarNames: m_szTournamentEventName (char)
            // NetworkVarNames: m_szTournamentEventStage (char)
            // NetworkVarNames: m_szMatchStatTxt (char)
            // NetworkVarNames: m_szTournamentPredictionsTxt (char)
            // NetworkVarNames: m_nTournamentPredictionsPct (int)
            // NetworkVarNames: m_flCMMItemDropRevealStartTime (GameTime_t)
            // NetworkVarNames: m_flCMMItemDropRevealEndTime (GameTime_t)
            // NetworkVarNames: m_bIsDroppingItems (bool)
            // NetworkVarNames: m_bIsQuestEligible (bool)
            // NetworkVarNames: m_bIsHltvActive (bool)
            // NetworkVarNames: m_arrProhibitedItemIndices (uint16)
            // NetworkVarNames: m_arrTournamentActiveCasterAccounts (uint32)
            // NetworkVarNames: m_numBestOfMaps (int)
            // NetworkVarNames: m_nHalloweenMaskListSeed (int)
            // NetworkVarNames: m_bBombDropped (bool)
            // NetworkVarNames: m_bBombPlanted (bool)
            // NetworkVarNames: m_iRoundWinStatus (int)
            // NetworkVarNames: m_eRoundWinReason (int)
            // NetworkVarNames: m_bTCantBuy (bool)
            // NetworkVarNames: m_bCTCantBuy (bool)
            // NetworkVarNames: m_iMatchStats_RoundResults (int)
            // NetworkVarNames: m_iMatchStats_PlayersAlive_CT (int)
            // NetworkVarNames: m_iMatchStats_PlayersAlive_T (int)
            // NetworkVarNames: m_TeamRespawnWaveTimes (float)
            // NetworkVarNames: m_flNextRespawnWave (GameTime_t)
            // NetworkVarNames: m_nServerQuestID (int)
            // NetworkVarNames: m_vMinimapMins (Vector)
            // NetworkVarNames: m_vMinimapMaxs (Vector)
            // NetworkVarNames: m_MinimapVerticalSectionHeights (float)
            // NetworkVarNames: m_nEndMatchMapGroupVoteTypes (int)
            // NetworkVarNames: m_nEndMatchMapGroupVoteOptions (int)
            // NetworkVarNames: m_nEndMatchMapVoteWinner (int)
            // NetworkVarNames: m_iNumConsecutiveCTLoses (int)
            // NetworkVarNames: m_iNumConsecutiveTerroristLoses (int)
            // NetworkVarNames: m_nMatchAbortedEarlyReason (int)
            // NetworkVarNames: m_pGameModeRules (CCSGameModeRules*)
            // NetworkVarNames: m_RetakeRules (CRetakeGameRules)
            // NetworkVarNames: m_nMatchEndCount (uint8)
            // NetworkVarNames: m_nTTeamIntroVariant (int)
            // NetworkVarNames: m_nCTTeamIntroVariant (int)
            // NetworkVarNames: m_bTeamIntroPeriod (bool)
            // NetworkVarNames: m_iRoundEndWinnerTeam (int)
            // NetworkVarNames: m_eRoundEndReason (int)
            // NetworkVarNames: m_bRoundEndShowTimerDefend (bool)
            // NetworkVarNames: m_iRoundEndTimerTime (int)
            // NetworkVarNames: m_sRoundEndFunFactToken (CUtlString)
            // NetworkVarNames: m_iRoundEndFunFactPlayerSlot (CPlayerSlot)
            // NetworkVarNames: m_iRoundEndFunFactData1 (int)
            // NetworkVarNames: m_iRoundEndFunFactData2 (int)
            // NetworkVarNames: m_iRoundEndFunFactData3 (int)
            // NetworkVarNames: m_sRoundEndMessage (CUtlString)
            // NetworkVarNames: m_iRoundEndPlayerCount (int)
            // NetworkVarNames: m_bRoundEndNoMusic (bool)
            // NetworkVarNames: m_iRoundEndLegacy (int)
            // NetworkVarNames: m_nRoundEndCount (uint8)
            // NetworkVarNames: m_iRoundStartRoundNumber (int)
            // NetworkVarNames: m_nRoundStartCount (uint8)
            pub mod C_CSGameRules {
                pub const m_bFreezePeriod: usize = 0x40; // 
                pub const m_bWarmupPeriod: usize = 0x41; // 
                pub const m_fWarmupPeriodEnd: usize = 0x44; // 
                pub const m_fWarmupPeriodStart: usize = 0x48; // 
                pub const m_bServerPaused: usize = 0x4C; // 
                pub const m_bTerroristTimeOutActive: usize = 0x4D; // 
                pub const m_bCTTimeOutActive: usize = 0x4E; // 
                pub const m_flTerroristTimeOutRemaining: usize = 0x50; // 
                pub const m_flCTTimeOutRemaining: usize = 0x54; // 
                pub const m_nTerroristTimeOuts: usize = 0x58; // 
                pub const m_nCTTimeOuts: usize = 0x5C; // 
                pub const m_bTechnicalTimeOut: usize = 0x60; // 
                pub const m_bMatchWaitingForResume: usize = 0x61; // 
                pub const m_iRoundTime: usize = 0x64; // 
                pub const m_fMatchStartTime: usize = 0x68; // 
                pub const m_fRoundStartTime: usize = 0x6C; // 
                pub const m_flRestartRoundTime: usize = 0x70; // 
                pub const m_bGameRestart: usize = 0x74; // 
                pub const m_flGameStartTime: usize = 0x78; // 
                pub const m_timeUntilNextPhaseStarts: usize = 0x7C; // 
                pub const m_gamePhase: usize = 0x80; // 
                pub const m_totalRoundsPlayed: usize = 0x84; // 
                pub const m_nRoundsPlayedThisPhase: usize = 0x88; // 
                pub const m_nOvertimePlaying: usize = 0x8C; // 
                pub const m_iHostagesRemaining: usize = 0x90; // 
                pub const m_bAnyHostageReached: usize = 0x94; // 
                pub const m_bMapHasBombTarget: usize = 0x95; // 
                pub const m_bMapHasRescueZone: usize = 0x96; // 
                pub const m_bMapHasBuyZone: usize = 0x97; // 
                pub const m_bIsQueuedMatchmaking: usize = 0x98; // 
                pub const m_nQueuedMatchmakingMode: usize = 0x9C; // 
                pub const m_bIsValveDS: usize = 0xA0; // 
                pub const m_bLogoMap: usize = 0xA1; // 
                pub const m_bPlayAllStepSoundsOnServer: usize = 0xA2; // 
                pub const m_iSpectatorSlotCount: usize = 0xA4; // 
                pub const m_MatchDevice: usize = 0xA8; // 
                pub const m_bHasMatchStarted: usize = 0xAC; // 
                pub const m_nNextMapInMapgroup: usize = 0xB0; // 
                pub const m_szTournamentEventName: usize = 0xB4; // 
                pub const m_szTournamentEventStage: usize = 0x2B4; // 
                pub const m_szMatchStatTxt: usize = 0x4B4; // 
                pub const m_szTournamentPredictionsTxt: usize = 0x6B4; // 
                pub const m_nTournamentPredictionsPct: usize = 0x8B4; // 
                pub const m_flCMMItemDropRevealStartTime: usize = 0x8B8; // 
                pub const m_flCMMItemDropRevealEndTime: usize = 0x8BC; // 
                pub const m_bIsDroppingItems: usize = 0x8C0; // 
                pub const m_bIsQuestEligible: usize = 0x8C1; // 
                pub const m_bIsHltvActive: usize = 0x8C2; // 
                pub const m_arrProhibitedItemIndices: usize = 0x8C4; // 
                pub const m_arrTournamentActiveCasterAccounts: usize = 0x98C; // 
                pub const m_numBestOfMaps: usize = 0x99C; // 
                pub const m_nHalloweenMaskListSeed: usize = 0x9A0; // 
                pub const m_bBombDropped: usize = 0x9A4; // 
                pub const m_bBombPlanted: usize = 0x9A5; // 
                pub const m_iRoundWinStatus: usize = 0x9A8; // 
                pub const m_eRoundWinReason: usize = 0x9AC; // 
                pub const m_bTCantBuy: usize = 0x9B0; // 
                pub const m_bCTCantBuy: usize = 0x9B1; // 
                pub const m_iMatchStats_RoundResults: usize = 0x9B4; // 
                pub const m_iMatchStats_PlayersAlive_CT: usize = 0xA2C; // 
                pub const m_iMatchStats_PlayersAlive_T: usize = 0xAA4; // 
                pub const m_TeamRespawnWaveTimes: usize = 0xB1C; // 
                pub const m_flNextRespawnWave: usize = 0xB9C; // 
                pub const m_nServerQuestID: usize = 0xC1C; // 
                pub const m_vMinimapMins: usize = 0xC20; // 
                pub const m_vMinimapMaxs: usize = 0xC2C; // 
                pub const m_MinimapVerticalSectionHeights: usize = 0xC38; // 
                pub const m_bSpawnedTerrorHuntHeavy: usize = 0xC58; // 
                pub const m_nEndMatchMapGroupVoteTypes: usize = 0xC5C; // 
                pub const m_nEndMatchMapGroupVoteOptions: usize = 0xC84; // 
                pub const m_nEndMatchMapVoteWinner: usize = 0xCAC; // 
                pub const m_iNumConsecutiveCTLoses: usize = 0xCB0; // 
                pub const m_iNumConsecutiveTerroristLoses: usize = 0xCB4; // 
                pub const m_bMarkClientStopRecordAtRoundEnd: usize = 0xCD0; // 
                pub const m_nMatchAbortedEarlyReason: usize = 0xD78; // 
                pub const m_bHasTriggeredRoundStartMusic: usize = 0xD7C; // 
                pub const m_bSwitchingTeamsAtRoundReset: usize = 0xD7D; // 
                pub const m_pGameModeRules: usize = 0xD98; // 
                pub const m_RetakeRules: usize = 0xDA0; // 
                pub const m_nMatchEndCount: usize = 0xEB8; // 
                pub const m_nTTeamIntroVariant: usize = 0xEBC; // 
                pub const m_nCTTeamIntroVariant: usize = 0xEC0; // 
                pub const m_bTeamIntroPeriod: usize = 0xEC4; // 
                pub const m_iRoundEndWinnerTeam: usize = 0xEC8; // 
                pub const m_eRoundEndReason: usize = 0xECC; // 
                pub const m_bRoundEndShowTimerDefend: usize = 0xED0; // 
                pub const m_iRoundEndTimerTime: usize = 0xED4; // 
                pub const m_sRoundEndFunFactToken: usize = 0xED8; // 
                pub const m_iRoundEndFunFactPlayerSlot: usize = 0xEE0; // 
                pub const m_iRoundEndFunFactData1: usize = 0xEE4; // 
                pub const m_iRoundEndFunFactData2: usize = 0xEE8; // 
                pub const m_iRoundEndFunFactData3: usize = 0xEEC; // 
                pub const m_sRoundEndMessage: usize = 0xEF0; // 
                pub const m_iRoundEndPlayerCount: usize = 0xEF8; // 
                pub const m_bRoundEndNoMusic: usize = 0xEFC; // 
                pub const m_iRoundEndLegacy: usize = 0xF00; // 
                pub const m_nRoundEndCount: usize = 0xF04; // 
                pub const m_iRoundStartRoundNumber: usize = 0xF08; // 
                pub const m_nRoundStartCount: usize = 0xF0C; // 
                pub const m_flLastPerfSampleTime: usize = 0x4F18; // 
            }
            // Parent: CBodyComponent
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_skeletonInstance (CSkeletonInstance)
            pub mod CBodyComponentSkeletonInstance {
                pub const m_skeletonInstance: usize = 0x50; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponCZ75a {
            }
            // Parent: C_BaseEntity
            // Field count: 33
            //
            // Metadata:
            // NetworkVarNames: m_CRenderComponent (CRenderComponent::Storage_t)
            // NetworkVarNames: m_CHitboxComponent (CHitboxComponent::Storage_t)
            // NetworkVarNames: m_nRenderMode (RenderMode_t)
            // NetworkVarNames: m_nRenderFX (RenderFx_t)
            // NetworkVarNames: m_clrRender (Color)
            // NetworkVarNames: m_vecRenderAttributes (EntityRenderAttribute_t)
            // NetworkVarNames: m_bRenderToCubemaps (bool)
            // NetworkVarNames: m_bNoInterpolate (bool)
            // NetworkVarNames: m_Collision (CCollisionProperty)
            // NetworkVarNames: m_Glow (CGlowProperty)
            // NetworkVarNames: m_flGlowBackfaceMult (float)
            // NetworkVarNames: m_fadeMinDist (float32)
            // NetworkVarNames: m_fadeMaxDist (float32)
            // NetworkVarNames: m_flFadeScale (float32)
            // NetworkVarNames: m_flShadowStrength (float32)
            // NetworkVarNames: m_nObjectCulling (uint8)
            // NetworkVarNames: m_nAddDecal (int)
            // NetworkVarNames: m_vDecalPosition (Vector)
            // NetworkVarNames: m_vDecalForwardAxis (Vector)
            // NetworkVarNames: m_flDecalHealBloodRate (float)
            // NetworkVarNames: m_flDecalHealHeightRate (float)
            // NetworkVarNames: m_ConfigEntitiesToPropagateMaterialDecalsTo (CHandle<C_BaseModelEntity>)
            pub mod C_BaseModelEntity {
                pub const m_CRenderComponent: usize = 0xA50; // 
                pub const m_CHitboxComponent: usize = 0xA58; // 
                pub const m_LastHitGroup: usize = 0xA80; // 
                pub const m_bInitModelEffects: usize = 0xAA8; // 
                pub const m_bIsStaticProp: usize = 0xAA9; // 
                pub const m_nLastAddDecal: usize = 0xAAC; // 
                pub const m_nDecalsAdded: usize = 0xAB0; // 
                pub const m_iOldHealth: usize = 0xAB4; // 
                pub const m_nRenderMode: usize = 0xAB8; // 
                pub const m_nRenderFX: usize = 0xAB9; // 
                pub const m_bAllowFadeInView: usize = 0xABA; // 
                pub const m_clrRender: usize = 0xAD8; // 
                pub const m_vecRenderAttributes: usize = 0xAE0; // 
                pub const m_bRenderToCubemaps: usize = 0xB48; // 
                pub const m_bNoInterpolate: usize = 0xB49; // 
                pub const m_Collision: usize = 0xB50; // 
                pub const m_Glow: usize = 0xC00; // 
                pub const m_flGlowBackfaceMult: usize = 0xC58; // 
                pub const m_fadeMinDist: usize = 0xC5C; // 
                pub const m_fadeMaxDist: usize = 0xC60; // 
                pub const m_flFadeScale: usize = 0xC64; // 
                pub const m_flShadowStrength: usize = 0xC68; // 
                pub const m_nObjectCulling: usize = 0xC6C; // 
                pub const m_nAddDecal: usize = 0xC70; // 
                pub const m_vDecalPosition: usize = 0xC74; // 
                pub const m_vDecalForwardAxis: usize = 0xC80; // 
                pub const m_flDecalHealBloodRate: usize = 0xC8C; // 
                pub const m_flDecalHealHeightRate: usize = 0xC90; // 
                pub const m_ConfigEntitiesToPropagateMaterialDecalsTo: usize = 0xC98; // 
                pub const m_vecViewOffset: usize = 0xCB0; // 
                pub const m_pClientAlphaProperty: usize = 0xCE0; // 
                pub const m_ClientOverrideTint: usize = 0xCE8; // 
                pub const m_bUseClientOverrideTint: usize = 0xCEC; // 
            }
            // Parent: C_BaseTrigger
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_source (string_t)
            // NetworkVarNames: m_destination (string_t)
            pub mod C_FootstepControl {
                pub const m_source: usize = 0xD30; // 
                pub const m_destination: usize = 0xD38; // 
            }
            // Parent: C_BaseClientUIEntity
            // Field count: 13
            //
            // Metadata:
            // NetworkVarNames: m_bIgnoreInput (bool)
            // NetworkVarNames: m_flWidth (float)
            // NetworkVarNames: m_flHeight (float)
            // NetworkVarNames: m_flDPI (float)
            // NetworkVarNames: m_flInteractDistance (float)
            // NetworkVarNames: m_flDepthOffset (float)
            // NetworkVarNames: m_unOwnerContext (uint32)
            // NetworkVarNames: m_unHorizontalAlign (uint32)
            // NetworkVarNames: m_unVerticalAlign (uint32)
            // NetworkVarNames: m_unOrientation (uint32)
            // NetworkVarNames: m_bAllowInteractionFromAllSceneWorlds (bool)
            // NetworkVarNames: m_vecCSSClasses (string_t)
            pub mod C_PointClientUIHUD {
                pub const m_bCheckCSSClasses: usize = 0xD60; // 
                pub const m_bIgnoreInput: usize = 0xEE0; // 
                pub const m_flWidth: usize = 0xEE4; // 
                pub const m_flHeight: usize = 0xEE8; // 
                pub const m_flDPI: usize = 0xEEC; // 
                pub const m_flInteractDistance: usize = 0xEF0; // 
                pub const m_flDepthOffset: usize = 0xEF4; // 
                pub const m_unOwnerContext: usize = 0xEF8; // 
                pub const m_unHorizontalAlign: usize = 0xEFC; // 
                pub const m_unVerticalAlign: usize = 0xF00; // 
                pub const m_unOrientation: usize = 0xF04; // 
                pub const m_bAllowInteractionFromAllSceneWorlds: usize = 0xF08; // 
                pub const m_vecCSSClasses: usize = 0xF10; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_flLightScale (float32)
            // NetworkVarNames: m_Radius (float32)
            pub mod C_SpotlightEnd {
                pub const m_flLightScale: usize = 0xD28; // 
                pub const m_Radius: usize = 0xD2C; // 
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iAttributeDefinitionIndex (attrib_definition_index_t)
            // NetworkVarNames: m_flValue (float)
            // NetworkVarNames: m_flInitialValue (float)
            // NetworkVarNames: m_nRefundableCurrency (int)
            // NetworkVarNames: m_bSetBonus (bool)
            pub mod CEconItemAttribute {
                pub const m_iAttributeDefinitionIndex: usize = 0x30; // 
                pub const m_flValue: usize = 0x34; // 
                pub const m_flInitialValue: usize = 0x38; // 
                pub const m_nRefundableCurrency: usize = 0x3C; // 
                pub const m_bSetBonus: usize = 0x40; // 
            }
            // Parent: C_BaseEntity
            // Field count: 26
            //
            // Metadata:
            // NetworkVarNames: m_FOV (float)
            // NetworkVarNames: m_Resolution (float)
            // NetworkVarNames: m_bFogEnable (bool)
            // NetworkVarNames: m_FogColor (Color)
            // NetworkVarNames: m_flFogStart (float)
            // NetworkVarNames: m_flFogEnd (float)
            // NetworkVarNames: m_flFogMaxDensity (float)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bUseScreenAspectRatio (bool)
            // NetworkVarNames: m_flAspectRatio (float)
            // NetworkVarNames: m_bNoSky (bool)
            // NetworkVarNames: m_fBrightness (float)
            // NetworkVarNames: m_flZFar (float)
            // NetworkVarNames: m_flZNear (float)
            // NetworkVarNames: m_bCanHLTVUse (bool)
            // NetworkVarNames: m_bAlignWithParent (bool)
            // NetworkVarNames: m_bDofEnabled (bool)
            // NetworkVarNames: m_flDofNearBlurry (float)
            // NetworkVarNames: m_flDofNearCrisp (float)
            // NetworkVarNames: m_flDofFarCrisp (float)
            // NetworkVarNames: m_flDofFarBlurry (float)
            // NetworkVarNames: m_flDofTiltToGround (float)
            pub mod C_PointCamera {
                pub const m_FOV: usize = 0x568; // 
                pub const m_Resolution: usize = 0x56C; // 
                pub const m_bFogEnable: usize = 0x570; // 
                pub const m_FogColor: usize = 0x571; // 
                pub const m_flFogStart: usize = 0x578; // 
                pub const m_flFogEnd: usize = 0x57C; // 
                pub const m_flFogMaxDensity: usize = 0x580; // 
                pub const m_bActive: usize = 0x584; // 
                pub const m_bUseScreenAspectRatio: usize = 0x585; // 
                pub const m_flAspectRatio: usize = 0x588; // 
                pub const m_bNoSky: usize = 0x58C; // 
                pub const m_fBrightness: usize = 0x590; // 
                pub const m_flZFar: usize = 0x594; // 
                pub const m_flZNear: usize = 0x598; // 
                pub const m_bCanHLTVUse: usize = 0x59C; // 
                pub const m_bAlignWithParent: usize = 0x59D; // 
                pub const m_bDofEnabled: usize = 0x59E; // 
                pub const m_flDofNearBlurry: usize = 0x5A0; // 
                pub const m_flDofNearCrisp: usize = 0x5A4; // 
                pub const m_flDofFarCrisp: usize = 0x5A8; // 
                pub const m_flDofFarBlurry: usize = 0x5AC; // 
                pub const m_flDofTiltToGround: usize = 0x5B0; // 
                pub const m_TargetFOV: usize = 0x5B4; // 
                pub const m_DegreesPerSecond: usize = 0x5B8; // 
                pub const m_bIsOn: usize = 0x5BC; // 
                pub const m_pNext: usize = 0x5C0; // 
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSPointScriptExtensions_weapon_cs_base {
            }
            // Parent: C_LightEntity
            // Field count: 0
            pub mod C_LightSpotEntity {
            }
            // Parent: C_BaseEntity
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_EnvWindShared (CEnvWindShared)
            pub mod C_EnvWind {
                pub const m_EnvWindShared: usize = 0x568; // 
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod C_GameRulesProxy {
            }
            // Parent: C_BaseFlex
            // Field count: 6
            //
            // Metadata:
            // MNetworkExcludeByUserGroup
            // NetworkVarNames: m_hMyWearables (CHandle<C_EconWearable>)
            pub mod C_BaseCombatCharacter {
                pub const m_hMyWearables: usize = 0x1120; // 
                pub const m_leftFootAttachment: usize = 0x1138; // 
                pub const m_rightFootAttachment: usize = 0x1139; // 
                pub const m_nWaterWakeMode: usize = 0x113C; // C_BaseCombatCharacter::WaterWakeMode_t
                pub const m_flWaterWorldZ: usize = 0x1140; // 
                pub const m_flWaterNextTraceTime: usize = 0x1144; // 
            }
            // Parent: C_BaseCSGrenadeProjectile
            // Field count: 0
            pub mod C_FlashbangProjectile {
            }
            // Parent: C_SoundEventEntity
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_flRadius (float)
            pub mod C_SoundEventSphereEntity {
                pub const m_flRadius: usize = 0x628; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 0
            pub mod CPlayer_ViewModelServices {
            }
            // Parent: CCSGO_WingmanIntroCharacterPosition
            // Field count: 0
            pub mod CCSGO_WingmanIntroCounterTerroristPosition {
            }
            // Parent: C_DynamicProp
            // Field count: 0
            pub mod C_DynamicPropAlias_cable_dynamic {
            }
            // Parent: C_PointEntity
            // Field count: 0
            pub mod C_InfoInstructorHintHostageRescueZone {
            }
            // Parent: C_PointEntity
            // Field count: 12
            //
            // Metadata:
            // NetworkVarNames: m_bIsPlayingBack (bool)
            // NetworkVarNames: m_bPaused (bool)
            // NetworkVarNames: m_bMultiplayer (bool)
            // NetworkVarNames: m_bAutogenerated (bool)
            // NetworkVarNames: m_flForceClientTime (float32)
            // NetworkVarNames: m_nSceneStringIndex (uint16)
            // NetworkVarNames: m_hActorList (CHandle<C_BaseFlex>)
            pub mod C_SceneEntity {
                pub const m_bIsPlayingBack: usize = 0x570; // 
                pub const m_bPaused: usize = 0x571; // 
                pub const m_bMultiplayer: usize = 0x572; // 
                pub const m_bAutogenerated: usize = 0x573; // 
                pub const m_flForceClientTime: usize = 0x574; // 
                pub const m_nSceneStringIndex: usize = 0x578; // 
                pub const m_bClientOnly: usize = 0x57A; // 
                pub const m_hOwner: usize = 0x57C; // 
                pub const m_hActorList: usize = 0x580; // 
                pub const m_bWasPlaying: usize = 0x598; // 
                pub const m_QueuedEvents: usize = 0x5A8; // 
                pub const m_flCurrentTime: usize = 0x5C0; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 20
            //
            // Metadata:
            // NetworkVarNames: m_vecCsViewPunchAngle (QAngle)
            // NetworkVarNames: m_nCsViewPunchAngleTick (GameTick_t)
            // NetworkVarNames: m_flCsViewPunchAngleTickRatio (float32)
            // NetworkVarNames: m_PlayerFog (fogplayerparams_t)
            // NetworkVarNames: m_hColorCorrectionCtrl (CHandle<CColorCorrection>)
            // NetworkVarNames: m_hViewEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hTonemapController (CHandle<CTonemapController2>)
            // NetworkVarNames: m_audio (audioparams_t)
            // NetworkVarNames: m_PostProcessingVolumes (CHandle<C_PostProcessingVolume>)
            pub mod CPlayer_CameraServices {
                pub const m_vecCsViewPunchAngle: usize = 0x40; // 
                pub const m_nCsViewPunchAngleTick: usize = 0x4C; // 
                pub const m_flCsViewPunchAngleTickRatio: usize = 0x50; // 
                pub const m_PlayerFog: usize = 0x58; // 
                pub const m_hColorCorrectionCtrl: usize = 0x98; // 
                pub const m_hViewEntity: usize = 0x9C; // 
                pub const m_hTonemapController: usize = 0xA0; // 
                pub const m_audio: usize = 0xA8; // 
                pub const m_PostProcessingVolumes: usize = 0x120; // 
                pub const m_flOldPlayerZ: usize = 0x138; // 
                pub const m_flOldPlayerViewOffsetZ: usize = 0x13C; // 
                pub const m_CurrentFog: usize = 0x140; // 
                pub const m_hOldFogController: usize = 0x1A8; // 
                pub const m_bOverrideFogColor: usize = 0x1AC; // 
                pub const m_OverrideFogColor: usize = 0x1B1; // 
                pub const m_bOverrideFogStartEnd: usize = 0x1C5; // 
                pub const m_fOverrideFogStart: usize = 0x1CC; // 
                pub const m_fOverrideFogEnd: usize = 0x1E0; // 
                pub const m_hActivePostProcessingVolume: usize = 0x1F4; // CHandle<C_PostProcessingVolume>
                pub const m_angDemoViewAngles: usize = 0x1F8; // 
            }
            // Parent: C_CSWeaponBase
            // Field count: 15
            //
            // Metadata:
            // NetworkVarNames: m_bRedraw (bool)
            // NetworkVarNames: m_bIsHeldByPlayer (bool)
            // NetworkVarNames: m_bPinPulled (bool)
            // NetworkVarNames: m_bJumpThrow (bool)
            // NetworkVarNames: m_bThrowAnimating (bool)
            // NetworkVarNames: m_fThrowTime (GameTime_t)
            // NetworkVarNames: m_flThrowStrength (float)
            // NetworkVarNames: m_flThrowStrengthApproach (float)
            // NetworkVarNames: m_fDropTime (GameTime_t)
            // NetworkVarNames: m_fPinPullTime (GameTime_t)
            // NetworkVarNames: m_bJustPulledPin (bool)
            // NetworkVarNames: m_nNextHoldTick (GameTick_t)
            // NetworkVarNames: m_flNextHoldFrac (float)
            // NetworkVarNames: m_hSwitchToWeaponAfterThrow (CHandle<CCSWeaponBase>)
            pub mod C_BaseCSGrenade {
                pub const m_bClientPredictDelete: usize = 0x1B10; // 
                pub const m_bRedraw: usize = 0x1B11; // 
                pub const m_bIsHeldByPlayer: usize = 0x1B12; // 
                pub const m_bPinPulled: usize = 0x1B13; // 
                pub const m_bJumpThrow: usize = 0x1B14; // 
                pub const m_bThrowAnimating: usize = 0x1B15; // 
                pub const m_fThrowTime: usize = 0x1B18; // 
                pub const m_flThrowStrength: usize = 0x1B1C; // 
                pub const m_flThrowStrengthApproach: usize = 0x1B20; // 
                pub const m_fDropTime: usize = 0x1B24; // 
                pub const m_fPinPullTime: usize = 0x1B28; // 
                pub const m_bJustPulledPin: usize = 0x1B2C; // 
                pub const m_nNextHoldTick: usize = 0x1B30; // 
                pub const m_flNextHoldFrac: usize = 0x1B34; // 
                pub const m_hSwitchToWeaponAfterThrow: usize = 0x1B38; // 
            }
            // Parent: C_FuncBrush
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_targetCamera (CUtlString)
            // NetworkVarNames: m_nResolutionEnum (int)
            // NetworkVarNames: m_bRenderShadows (bool)
            // NetworkVarNames: m_bUseUniqueColorTarget (bool)
            // NetworkVarNames: m_brushModelName (CUtlString)
            // NetworkVarNames: m_hTargetCamera (EHANDLE)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bDraw3DSkybox (bool)
            pub mod C_FuncMonitor {
                pub const m_targetCamera: usize = 0xD28; // 
                pub const m_nResolutionEnum: usize = 0xD30; // 
                pub const m_bRenderShadows: usize = 0xD34; // 
                pub const m_bUseUniqueColorTarget: usize = 0xD35; // 
                pub const m_brushModelName: usize = 0xD38; // 
                pub const m_hTargetCamera: usize = 0xD40; // 
                pub const m_bEnabled: usize = 0xD44; // 
                pub const m_bDraw3DSkybox: usize = 0xD45; // 
            }
            // Parent: C_BaseTrigger
            // Field count: 0
            pub mod C_TriggerMultiple {
            }
            // Parent: None
            // Field count: 1
            pub mod C_RopeKeyframe__CPhysicsDelegate {
                pub const m_pKeyframe: usize = 0x8; // 
            }
            // Parent: CEntityComponent
            // Field count: 2
            pub mod CBodyComponent {
                pub const m_pSceneNode: usize = 0x8; // 
                pub const __m_pChainEntity: usize = 0x20; // 
            }
            // Parent: C_SoundAreaEntityBase
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_flRadius (float)
            pub mod C_SoundAreaEntitySphere {
                pub const m_flRadius: usize = 0x590; // 
            }
            // Parent: CPlayer_MovementServices
            // Field count: 0
            pub mod CCSObserver_MovementServices {
            }
            // Parent: C_BaseModelEntity
            // Field count: 8
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_nHorizontalSize (uint32)
            // NetworkVarNames: m_nVerticalSize (uint32)
            // NetworkVarNames: m_nMinDist (uint32)
            // NetworkVarNames: m_nMaxDist (uint32)
            // NetworkVarNames: m_nOuterMaxDist (uint32)
            // NetworkVarNames: m_flGlowProxySize (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            pub mod C_LightGlow {
                pub const m_nHorizontalSize: usize = 0xD28; // 
                pub const m_nVerticalSize: usize = 0xD2C; // 
                pub const m_nMinDist: usize = 0xD30; // 
                pub const m_nMaxDist: usize = 0xD34; // 
                pub const m_nOuterMaxDist: usize = 0xD38; // 
                pub const m_flGlowProxySize: usize = 0xD3C; // 
                pub const m_flHDRColorScale: usize = 0xD40; // 
                pub const m_GlowOverlay: usize = 0xD48; // 
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_duration (float32)
            // NetworkVarNames: m_timestamp (GameTime_t)
            // NetworkVarNames: m_timescale (float32)
            // NetworkVarNames: m_nWorldGroupId (WorldGroupId_t)
            pub mod CountdownTimer {
                pub const m_duration: usize = 0x8; // 
                pub const m_timestamp: usize = 0xC; // 
                pub const m_timescale: usize = 0x10; // 
                pub const m_nWorldGroupId: usize = 0x14; // 
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hOwner (CEntityHandle)
            // NetworkVarNames: m_name (CUtlStringToken)
            pub mod CGameSceneNodeHandle {
                pub const m_hOwner: usize = 0x8; // 
                pub const m_name: usize = 0xC; // 
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: nTagTick (GameTick_t)
            // NetworkVarNames: flFlinchModSmall (float)
            // NetworkVarNames: flFlinchModLarge (float)
            // NetworkVarNames: flFriendlyFireDamageReductionRatio (float)
            pub mod PredictedDamageTag_t {
                pub const nTagTick: usize = 0x30; // 
                pub const flFlinchModSmall: usize = 0x34; // 
                pub const flFlinchModLarge: usize = 0x38; // 
                pub const flFriendlyFireDamageReductionRatio: usize = 0x3C; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 14
            pub mod C_ClientRagdoll {
                pub const m_bFadeOut: usize = 0xF88; // 
                pub const m_bImportant: usize = 0xF89; // 
                pub const m_flEffectTime: usize = 0xF8C; // 
                pub const m_gibDespawnTime: usize = 0xF90; // 
                pub const m_iCurrentFriction: usize = 0xF94; // 
                pub const m_iMinFriction: usize = 0xF98; // 
                pub const m_iMaxFriction: usize = 0xF9C; // 
                pub const m_iFrictionAnimState: usize = 0xFA0; // 
                pub const m_bReleaseRagdoll: usize = 0xFA4; // 
                pub const m_iEyeAttachment: usize = 0xFA5; // 
                pub const m_bFadingOut: usize = 0xFA6; // 
                pub const m_flScaleEnd: usize = 0xFA8; // 
                pub const m_flScaleTimeStart: usize = 0xFD0; // 
                pub const m_flScaleTimeEnd: usize = 0xFF8; // 
            }
            // Parent: C_BaseEntity
            // Field count: 1
            pub mod C_GlobalLight {
                pub const m_WindClothForceHandle: usize = 0xA30; // 
            }
            // Parent: C_BaseEntity
            // Field count: 10
            //
            // Metadata:
            // NetworkVarNames: m_bHostageAlive (bool)
            // NetworkVarNames: m_isHostageFollowingSomeone (bool)
            // NetworkVarNames: m_iHostageEntityIDs (CEntityIndex)
            // NetworkVarNames: m_bombsiteCenterA (Vector)
            // NetworkVarNames: m_bombsiteCenterB (Vector)
            // NetworkVarNames: m_hostageRescueX (int)
            // NetworkVarNames: m_hostageRescueY (int)
            // NetworkVarNames: m_hostageRescueZ (int)
            // NetworkVarNames: m_bEndMatchNextMapAllVoted (bool)
            pub mod C_CSPlayerResource {
                pub const m_bHostageAlive: usize = 0x568; // 
                pub const m_isHostageFollowingSomeone: usize = 0x574; // 
                pub const m_iHostageEntityIDs: usize = 0x580; // 
                pub const m_bombsiteCenterA: usize = 0x5B0; // 
                pub const m_bombsiteCenterB: usize = 0x5BC; // 
                pub const m_hostageRescueX: usize = 0x5C8; // 
                pub const m_hostageRescueY: usize = 0x5D8; // 
                pub const m_hostageRescueZ: usize = 0x5E8; // 
                pub const m_bEndMatchNextMapAllVoted: usize = 0x5F8; // 
                pub const m_foundGoalPositions: usize = 0x5F9; // 
            }
            // Parent: CGameSceneNode
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_modelState (CModelState)
            // NetworkVarNames: m_bIsAnimationEnabled (bool)
            // NetworkVarNames: m_bUseParentRenderBounds (bool)
            // NetworkVarNames: m_materialGroup (CUtlStringToken)
            // NetworkVarNames: m_nHitboxSet (uint8)
            pub mod CSkeletonInstance {
                pub const m_modelState: usize = 0x170; // 
                pub const m_bIsAnimationEnabled: usize = 0x3A0; // 
                pub const m_bUseParentRenderBounds: usize = 0x3A1; // 
                pub const m_bDisableSolidCollisionsForHierarchy: usize = 0x3A2; // 
                pub const m_bDirtyMotionType: usize = 0x0; // 
                pub const m_bIsGeneratingLatchedParentSpaceState: usize = 0x0; // 
                pub const m_materialGroup: usize = 0x3A4; // 
                pub const m_nHitboxSet: usize = 0x3A8; // 
            }
            // Parent: CBodyComponentSkeletonInstance
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_animationController (CBaseAnimGraphController)
            pub mod CBodyComponentBaseAnimGraph {
                pub const m_animationController: usize = 0x490; // 
            }
            // Parent: CBodyComponent
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_sceneNode (CGameSceneNode)
            pub mod CBodyComponentPoint {
                pub const m_sceneNode: usize = 0x50; // 
            }
            // Parent: C_BaseCSGrenadeProjectile
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nSmokeEffectTickBegin (int)
            // NetworkVarNames: m_bDidSmokeEffect (bool)
            // NetworkVarNames: m_nRandomSeed (int)
            // NetworkVarNames: m_vSmokeColor (Vector)
            // NetworkVarNames: m_vSmokeDetonationPos (Vector)
            // NetworkVarNames: m_VoxelFrameData (CUtlVector<uint8>)
            pub mod C_SmokeGrenadeProjectile {
                pub const m_nSmokeEffectTickBegin: usize = 0x1210; // 
                pub const m_bDidSmokeEffect: usize = 0x1214; // 
                pub const m_nRandomSeed: usize = 0x1218; // 
                pub const m_vSmokeColor: usize = 0x121C; // 
                pub const m_vSmokeDetonationPos: usize = 0x1228; // 
                pub const m_VoxelFrameData: usize = 0x1238; // 
                pub const m_bSmokeVolumeDataReceived: usize = 0x1250; // 
                pub const m_bSmokeEffectSpawned: usize = 0x1251; // 
            }
            // Parent: C_SoundEventEntity
            // Field count: 0
            pub mod C_SoundEventEntityAlias_snd_event_point {
            }
            // Parent: C_BaseEntity
            // Field count: 8
            pub mod C_CSGO_MapPreviewCameraPath {
                pub const m_flZFar: usize = 0x568; // 
                pub const m_flZNear: usize = 0x56C; // 
                pub const m_bLoop: usize = 0x570; // 
                pub const m_bVerticalFOV: usize = 0x571; // 
                pub const m_bConstantSpeed: usize = 0x572; // 
                pub const m_flDuration: usize = 0x574; // 
                pub const m_flPathLength: usize = 0x5B8; // 
                pub const m_flPathDuration: usize = 0x5BC; // 
            }
            // Parent: CCSGO_WingmanIntroCharacterPosition
            // Field count: 0
            pub mod CCSGO_WingmanIntroTerroristPosition {
            }
            // Parent: C_BaseModelEntity
            // Field count: 24
            //
            // Metadata:
            // NetworkVarNames: m_firePositions (Vector)
            // NetworkVarNames: m_fireParentPositions (Vector)
            // NetworkVarNames: m_bFireIsBurning (bool)
            // NetworkVarNames: m_BurnNormal (Vector)
            // NetworkVarNames: m_fireCount (int)
            // NetworkVarNames: m_nInfernoType (int)
            // NetworkVarNames: m_nFireLifetime (float)
            // NetworkVarNames: m_bInPostEffectTime (bool)
            // NetworkVarNames: m_nFireEffectTickBegin (int)
            pub mod C_Inferno {
                pub const m_nfxFireDamageEffect: usize = 0xD68; // 
                pub const m_hInfernoPointsSnapshot: usize = 0xD70; // 
                pub const m_hInfernoFillerPointsSnapshot: usize = 0xD78; // 
                pub const m_hInfernoOutlinePointsSnapshot: usize = 0xD80; // 
                pub const m_hInfernoClimbingOutlinePointsSnapshot: usize = 0xD88; // 
                pub const m_hInfernoDecalsSnapshot: usize = 0xD90; // 
                pub const m_firePositions: usize = 0xD98; // 
                pub const m_fireParentPositions: usize = 0x1098; // 
                pub const m_bFireIsBurning: usize = 0x1398; // 
                pub const m_BurnNormal: usize = 0x13D8; // 
                pub const m_fireCount: usize = 0x16D8; // 
                pub const m_nInfernoType: usize = 0x16DC; // 
                pub const m_nFireLifetime: usize = 0x16E0; // 
                pub const m_bInPostEffectTime: usize = 0x16E4; // 
                pub const m_lastFireCount: usize = 0x16E8; // 
                pub const m_nFireEffectTickBegin: usize = 0x16EC; // 
                pub const m_drawableCount: usize = 0x82F0; // 
                pub const m_blosCheck: usize = 0x82F4; // 
                pub const m_nlosperiod: usize = 0x82F8; // 
                pub const m_maxFireHalfWidth: usize = 0x82FC; // 
                pub const m_maxFireHeight: usize = 0x8300; // 
                pub const m_minBounds: usize = 0x8304; // 
                pub const m_maxBounds: usize = 0x8310; // 
                pub const m_flLastGrassBurnThink: usize = 0x831C; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 0
            pub mod C_HostageCarriableProp {
            }
            // Parent: C_CSGO_TeamPreviewCamera
            // Field count: 0
            pub mod C_CSGO_TerroristWingmanIntroCamera {
            }
            // Parent: C_BaseEntity
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_hPlayer (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_hPingedEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_iType (int)
            // NetworkVarNames: m_bUrgent (bool)
            // NetworkVarNames: m_szPlaceName (char)
            pub mod C_PlayerPing {
                pub const m_hPlayer: usize = 0x598; // 
                pub const m_hPingedEntity: usize = 0x59C; // 
                pub const m_iType: usize = 0x5A0; // 
                pub const m_bUrgent: usize = 0x5A4; // 
                pub const m_szPlaceName: usize = 0x5A5; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponMP7 {
            }
            // Parent: C_Team
            // Field count: 10
            //
            // Metadata:
            // NetworkVarNames: m_szTeamMatchStat (char)
            // NetworkVarNames: m_numMapVictories (int)
            // NetworkVarNames: m_bSurrendered (bool)
            // NetworkVarNames: m_scoreFirstHalf (int32)
            // NetworkVarNames: m_scoreSecondHalf (int32)
            // NetworkVarNames: m_scoreOvertime (int32)
            // NetworkVarNames: m_szClanTeamname (char)
            // NetworkVarNames: m_iClanID (uint32)
            // NetworkVarNames: m_szTeamFlagImage (char)
            // NetworkVarNames: m_szTeamLogoImage (char)
            pub mod C_CSTeam {
                pub const m_szTeamMatchStat: usize = 0x620; // 
                pub const m_numMapVictories: usize = 0x820; // 
                pub const m_bSurrendered: usize = 0x824; // 
                pub const m_scoreFirstHalf: usize = 0x828; // 
                pub const m_scoreSecondHalf: usize = 0x82C; // 
                pub const m_scoreOvertime: usize = 0x830; // 
                pub const m_szClanTeamname: usize = 0x834; // 
                pub const m_iClanID: usize = 0x8B8; // 
                pub const m_szTeamFlagImage: usize = 0x8BC; // 
                pub const m_szTeamLogoImage: usize = 0x8C4; // 
            }
            // Parent: C_BaseEntity
            // Field count: 10
            pub mod C_CSGO_MapPreviewCameraPathNode {
                pub const m_szParentPathUniqueID: usize = 0x568; // 
                pub const m_nPathIndex: usize = 0x570; // 
                pub const m_vInTangentLocal: usize = 0x574; // 
                pub const m_vOutTangentLocal: usize = 0x580; // 
                pub const m_flFOV: usize = 0x58C; // 
                pub const m_flCameraSpeed: usize = 0x590; // 
                pub const m_flEaseIn: usize = 0x594; // 
                pub const m_flEaseOut: usize = 0x598; // 
                pub const m_vInTangentWorld: usize = 0x59C; // 
                pub const m_vOutTangentWorld: usize = 0x5A8; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponMag7 {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_DEagle {
            }
            // Parent: C_CSPlayerPawnBase
            // Field count: 104
            //
            // Metadata:
            // NetworkVarNames: m_pBulletServices (CCSPlayer_BulletServices*)
            // NetworkVarNames: m_pHostageServices (CCSPlayer_HostageServices*)
            // NetworkVarNames: m_pBuyServices (CCSPlayer_BuyServices*)
            // NetworkVarNames: m_pGlowServices (CCSPlayer_GlowServices*)
            // NetworkVarNames: m_pActionTrackingServices (CCSPlayer_ActionTrackingServices*)
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_flHealthShotBoostExpirationTime (GameTime_t)
            // NetworkVarNames: m_bHasFemaleVoice (bool)
            // NetworkVarNames: m_szLastPlaceName (char)
            // NetworkVarNames: m_bInBuyZone (bool)
            // NetworkVarNames: m_aimPunchAngle (QAngle)
            // NetworkVarNames: m_aimPunchAngleVel (QAngle)
            // NetworkVarNames: m_aimPunchTickBase (int)
            // NetworkVarNames: m_aimPunchTickFraction (float)
            // NetworkVarNames: m_bInHostageRescueZone (bool)
            // NetworkVarNames: m_bInBombZone (bool)
            // NetworkVarNames: m_bIsBuyMenuOpen (bool)
            // NetworkVarNames: m_flTimeOfLastInjury (GameTime_t)
            // NetworkVarNames: m_flNextSprayDecalTime (GameTime_t)
            // NetworkVarNames: m_iRetakesOffering (int)
            // NetworkVarNames: m_iRetakesOfferingCard (int)
            // NetworkVarNames: m_bRetakesHasDefuseKit (bool)
            // NetworkVarNames: m_bRetakesMVPLastRound (bool)
            // NetworkVarNames: m_iRetakesMVPBoostItem (int)
            // NetworkVarNames: m_RetakesMVPBoostExtraUtility (loadout_slot_t)
            // NetworkVarNames: m_EconGloves (CEconItemView)
            // NetworkVarNames: m_nEconGlovesChanged (uint8)
            // NetworkVarNames: m_nRagdollDamageBone (int)
            // NetworkVarNames: m_vRagdollDamageForce (Vector)
            // NetworkVarNames: m_vRagdollDamagePosition (Vector)
            // NetworkVarNames: m_szRagdollDamageWeaponName (char)
            // NetworkVarNames: m_bRagdollDamageHeadshot (bool)
            // NetworkVarNames: m_vRagdollServerOrigin (Vector)
            // MNetworkReplayCompatField
            // NetworkVarNames: m_qDeathEyeAngles (QAngle)
            // NetworkVarNames: m_bLeftHanded (bool)
            // NetworkVarNames: m_fSwitchedHandednessTime (GameTime_t)
            // NetworkVarNames: m_flViewmodelOffsetX (float)
            // NetworkVarNames: m_flViewmodelOffsetY (float)
            // NetworkVarNames: m_flViewmodelOffsetZ (float)
            // NetworkVarNames: m_flViewmodelFOV (float)
            // NetworkVarNames: m_vecPlayerPatchEconIndices (uint32)
            // NetworkVarNames: m_GunGameImmunityColor (Color)
            // NetworkVarNames: m_bIsWalking (bool)
            // NetworkVarNames: m_entitySpottedState (EntitySpottedState_t)
            // NetworkVarNames: m_bIsScoped (bool)
            // NetworkVarNames: m_bResumeZoom (bool)
            // NetworkVarNames: m_bIsDefusing (bool)
            // NetworkVarNames: m_bIsGrabbingHostage (bool)
            // NetworkVarNames: m_iBlockingUseActionInProgress (CSPlayerBlockingUseAction_t)
            // NetworkVarNames: m_flEmitSoundTime (GameTime_t)
            // NetworkVarNames: m_bInNoDefuseArea (bool)
            // NetworkVarNames: m_nWhichBombZone (int)
            // NetworkVarNames: m_iShotsFired (int)
            // NetworkVarNames: m_flFlinchStack (float)
            // NetworkVarNames: m_flVelocityModifier (float)
            // NetworkVarNames: m_flHitHeading (float)
            // NetworkVarNames: m_nHitBodyPart (int)
            // NetworkVarNames: m_bWaitForNoAttack (bool)
            // NetworkVarNames: m_bKilledByHeadshot (bool)
            // NetworkVarNames: m_ArmorValue (int32)
            // NetworkVarNames: m_unCurrentEquipmentValue (uint16)
            // NetworkVarNames: m_unRoundStartEquipmentValue (uint16)
            // NetworkVarNames: m_unFreezetimeEndEquipmentValue (uint16)
            // NetworkVarNames: m_nLastKillerIndex (CEntityIndex)
            // NetworkVarNames: m_PredictedDamageTags (PredictedDamageTag_t)
            pub mod C_CSPlayerPawn {
                pub const m_pBulletServices: usize = 0x1518; // 
                pub const m_pHostageServices: usize = 0x1520; // 
                pub const m_pBuyServices: usize = 0x1528; // 
                pub const m_pGlowServices: usize = 0x1530; // 
                pub const m_pActionTrackingServices: usize = 0x1538; // CCSPlayer_ActionTrackingServices*
                pub const m_pDamageReactServices: usize = 0x1540; // 
                pub const m_flHealthShotBoostExpirationTime: usize = 0x1548; // 
                pub const m_flLastFiredWeaponTime: usize = 0x154C; // 
                pub const m_bHasFemaleVoice: usize = 0x1550; // 
                pub const m_flLandingTimeSeconds: usize = 0x1554; // 
                pub const m_flOldFallVelocity: usize = 0x1558; // 
                pub const m_szLastPlaceName: usize = 0x155C; // 
                pub const m_bPrevDefuser: usize = 0x156E; // 
                pub const m_bPrevHelmet: usize = 0x156F; // 
                pub const m_nPrevArmorVal: usize = 0x1570; // 
                pub const m_nPrevGrenadeAmmoCount: usize = 0x1574; // 
                pub const m_unPreviousWeaponHash: usize = 0x1578; // 
                pub const m_unWeaponHash: usize = 0x157C; // 
                pub const m_bInBuyZone: usize = 0x1580; // 
                pub const m_bPreviouslyInBuyZone: usize = 0x1581; // 
                pub const m_aimPunchAngle: usize = 0x1584; // 
                pub const m_aimPunchAngleVel: usize = 0x1590; // 
                pub const m_aimPunchTickBase: usize = 0x159C; // 
                pub const m_aimPunchTickFraction: usize = 0x15A0; // 
                pub const m_aimPunchCache: usize = 0x15A8; // 
                pub const m_bInLanding: usize = 0x15C8; // 
                pub const m_flLandingStartTime: usize = 0x15CC; // 
                pub const m_bInHostageRescueZone: usize = 0x15D0; // 
                pub const m_bInBombZone: usize = 0x15D1; // 
                pub const m_bIsBuyMenuOpen: usize = 0x15D2; // 
                pub const m_flTimeOfLastInjury: usize = 0x15D4; // 
                pub const m_flNextSprayDecalTime: usize = 0x15D8; // 
                pub const m_iRetakesOffering: usize = 0x1710; // 
                pub const m_iRetakesOfferingCard: usize = 0x1714; // 
                pub const m_bRetakesHasDefuseKit: usize = 0x1718; // 
                pub const m_bRetakesMVPLastRound: usize = 0x1719; // 
                pub const m_iRetakesMVPBoostItem: usize = 0x171C; // 
                pub const m_RetakesMVPBoostExtraUtility: usize = 0x1720; // 
                pub const m_bNeedToReApplyGloves: usize = 0x1740; // 
                pub const m_EconGloves: usize = 0x1748; // 
                pub const m_nEconGlovesChanged: usize = 0x1B90; // 
                pub const m_bMustSyncRagdollState: usize = 0x1B91; // 
                pub const m_nRagdollDamageBone: usize = 0x1B94; // 
                pub const m_vRagdollDamageForce: usize = 0x1B98; // 
                pub const m_vRagdollDamagePosition: usize = 0x1BA4; // 
                pub const m_szRagdollDamageWeaponName: usize = 0x1BB0; // 
                pub const m_bRagdollDamageHeadshot: usize = 0x1BF0; // 
                pub const m_vRagdollServerOrigin: usize = 0x1BF4; // 
                pub const m_bLastHeadBoneTransformIsValid: usize = 0x2278; // 
                pub const m_lastLandTime: usize = 0x227C; // 
                pub const m_bOnGroundLastTick: usize = 0x2280; // 
                pub const m_qDeathEyeAngles: usize = 0x229C; // 
                pub const m_bSkipOneHeadConstraintUpdate: usize = 0x22A8; // 
                pub const m_bLeftHanded: usize = 0x22A9; // 
                pub const m_fSwitchedHandednessTime: usize = 0x22AC; // 
                pub const m_flViewmodelOffsetX: usize = 0x22B0; // 
                pub const m_flViewmodelOffsetY: usize = 0x22B4; // 
                pub const m_flViewmodelOffsetZ: usize = 0x22B8; // 
                pub const m_flViewmodelFOV: usize = 0x22BC; // 
                pub const m_vecPlayerPatchEconIndices: usize = 0x22C0; // 
                pub const m_GunGameImmunityColor: usize = 0x22F8; // 
                pub const m_vecBulletHitModels: usize = 0x2348; // 
                pub const m_bIsWalking: usize = 0x2360; // 
                pub const m_thirdPersonHeading: usize = 0x2368; // 
                pub const m_flSlopeDropOffset: usize = 0x2380; // 
                pub const m_flSlopeDropHeight: usize = 0x2390; // 
                pub const m_vHeadConstraintOffset: usize = 0x23A0; // 
                pub const m_entitySpottedState: usize = 0x23B8; // 
                pub const m_bIsScoped: usize = 0x23D0; // 
                pub const m_bResumeZoom: usize = 0x23D1; // 
                pub const m_bIsDefusing: usize = 0x23D2; // 
                pub const m_bIsGrabbingHostage: usize = 0x23D3; // 
                pub const m_iBlockingUseActionInProgress: usize = 0x23D4; // 
                pub const m_flEmitSoundTime: usize = 0x23D8; // 
                pub const m_bInNoDefuseArea: usize = 0x23DC; // 
                pub const m_nWhichBombZone: usize = 0x23E0; // 
                pub const m_iShotsFired: usize = 0x23E4; // 
                pub const m_flFlinchStack: usize = 0x23E8; // 
                pub const m_flVelocityModifier: usize = 0x23EC; // 
                pub const m_flHitHeading: usize = 0x23F0; // 
                pub const m_nHitBodyPart: usize = 0x23F4; // 
                pub const m_bWaitForNoAttack: usize = 0x23F8; // 
                pub const m_ignoreLadderJumpTime: usize = 0x23FC; // 
                pub const m_bKilledByHeadshot: usize = 0x2401; // 
                pub const m_ArmorValue: usize = 0x2404; // 
                pub const m_unCurrentEquipmentValue: usize = 0x2408; // 
                pub const m_unRoundStartEquipmentValue: usize = 0x240A; // 
                pub const m_unFreezetimeEndEquipmentValue: usize = 0x240C; // 
                pub const m_nLastKillerIndex: usize = 0x2410; // 
                pub const m_bOldIsScoped: usize = 0x2414; // 
                pub const m_bHasDeathInfo: usize = 0x2415; // 
                pub const m_flDeathInfoTime: usize = 0x2418; // 
                pub const m_vecDeathInfoOrigin: usize = 0x241C; // 
                pub const m_grenadeParameterStashTime: usize = 0x2428; // 
                pub const m_bGrenadeParametersStashed: usize = 0x242C; // 
                pub const m_angStashedShootAngles: usize = 0x2430; // 
                pub const m_vecStashedGrenadeThrowPosition: usize = 0x243C; // 
                pub const m_vecStashedVelocity: usize = 0x2448; // 
                pub const m_angShootAngleHistory: usize = 0x2454; // 
                pub const m_vecThrowPositionHistory: usize = 0x246C; // 
                pub const m_vecVelocityHistory: usize = 0x2484; // 
                pub const m_PredictedDamageTags: usize = 0x24A0; // 
                pub const m_nPrevHighestReceivedDamageTagTick: usize = 0x24F0; // 
                pub const m_nHighestAppliedDamageTagTick: usize = 0x24F4; // 
            }
            // Parent: C_RagdollProp
            // Field count: 7
            //
            // Metadata:
            // NetworkVarNames: m_boneIndexAttached (uint32)
            // NetworkVarNames: m_ragdollAttachedObjectIndex (uint32)
            // NetworkVarNames: m_attachmentPointBoneSpace (Vector)
            // NetworkVarNames: m_attachmentPointRagdollSpace (Vector)
            pub mod C_RagdollPropAttached {
                pub const m_boneIndexAttached: usize = 0x1000; // 
                pub const m_ragdollAttachedObjectIndex: usize = 0x1004; // 
                pub const m_attachmentPointBoneSpace: usize = 0x1008; // 
                pub const m_attachmentPointRagdollSpace: usize = 0x1014; // 
                pub const m_vecOffset: usize = 0x1020; // 
                pub const m_parentTime: usize = 0x102C; // 
                pub const m_bHasParent: usize = 0x1030; // 
            }
            // Parent: C_BaseGrenade
            // Field count: 16
            //
            // Metadata:
            // NetworkVarNames: m_vInitialPosition (Vector)
            // NetworkVarNames: m_vInitialVelocity (Vector)
            // NetworkVarNames: m_nBounces (int)
            // NetworkVarNames: m_nExplodeEffectIndex (HParticleSystemDefinitionStrong)
            // NetworkVarNames: m_nExplodeEffectTickBegin (int)
            // NetworkVarNames: m_vecExplodeEffectOrigin (Vector)
            pub mod C_BaseCSGrenadeProjectile {
                pub const m_vInitialPosition: usize = 0x1170; // 
                pub const m_vInitialVelocity: usize = 0x117C; // 
                pub const m_nBounces: usize = 0x1188; // 
                pub const m_nExplodeEffectIndex: usize = 0x1190; // 
                pub const m_nExplodeEffectTickBegin: usize = 0x1198; // 
                pub const m_vecExplodeEffectOrigin: usize = 0x119C; // 
                pub const m_flSpawnTime: usize = 0x11A8; // 
                pub const vecLastTrailLinePos: usize = 0x11AC; // 
                pub const flNextTrailLineTime: usize = 0x11B8; // 
                pub const m_bExplodeEffectBegan: usize = 0x11BC; // 
                pub const m_bCanCreateGrenadeTrail: usize = 0x11BD; // 
                pub const m_nSnapshotTrajectoryEffectIndex: usize = 0x11C0; // 
                pub const m_hSnapshotTrajectoryParticleSnapshot: usize = 0x11C8; // 
                pub const m_arrTrajectoryTrailPoints: usize = 0x11D0; // 
                pub const m_arrTrajectoryTrailPointCreationTimes: usize = 0x11E8; // 
                pub const m_flTrajectoryTrailEffectCreationTime: usize = 0x1200; // 
            }
            // Parent: C_BaseToggle
            // Field count: 2
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_bClientSidePredicted (bool)
            pub mod C_BaseTrigger {
                pub const m_bDisabled: usize = 0xD28; // 
                pub const m_bClientSidePredicted: usize = 0xD29; // 
            }
            // Parent: C_CSWeaponBase
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_SequenceCompleteTimer (CountdownTimer)
            // NetworkVarNames: m_bRedraw (bool)
            pub mod C_WeaponBaseItem {
                pub const m_SequenceCompleteTimer: usize = 0x1B10; // 
                pub const m_bRedraw: usize = 0x1B28; // 
            }
            // Parent: None
            // Field count: 21
            //
            // Metadata:
            // NetworkVarNames: m_vOrigin (Vector)
            // NetworkVarNames: m_vStart (Vector)
            // NetworkVarNames: m_vNormal (Vector)
            // NetworkVarNames: m_vAngles (QAngle)
            // NetworkVarNames: m_hEntity (CEntityHandle)
            // NetworkVarNames: m_hOtherEntity (CEntityHandle)
            // NetworkVarNames: m_flScale (float32)
            // NetworkVarNames: m_flMagnitude (float32)
            // NetworkVarNames: m_flRadius (float32)
            // NetworkVarNames: m_nSurfaceProp (CUtlStringToken)
            // NetworkVarNames: m_nEffectIndex (HParticleSystemDefinition)
            // NetworkVarNames: m_nDamageType (uint32)
            // NetworkVarNames: m_nPenetrate (uint8)
            // NetworkVarNames: m_nMaterial (uint16)
            // NetworkVarNames: m_nHitBox (uint16)
            // NetworkVarNames: m_nColor (uint8)
            // NetworkVarNames: m_fFlags (uint8)
            // NetworkVarNames: m_nAttachmentIndex (AttachmentHandle_t)
            // NetworkVarNames: m_nAttachmentName (CUtlStringToken)
            // NetworkVarNames: m_iEffectName (uint16)
            // NetworkVarNames: m_nExplosionType (uint8)
            pub mod CEffectData {
                pub const m_vOrigin: usize = 0x8; // 
                pub const m_vStart: usize = 0x14; // 
                pub const m_vNormal: usize = 0x20; // 
                pub const m_vAngles: usize = 0x2C; // 
                pub const m_hEntity: usize = 0x38; // 
                pub const m_hOtherEntity: usize = 0x3C; // 
                pub const m_flScale: usize = 0x40; // 
                pub const m_flMagnitude: usize = 0x44; // 
                pub const m_flRadius: usize = 0x48; // 
                pub const m_nSurfaceProp: usize = 0x4C; // 
                pub const m_nEffectIndex: usize = 0x50; // 
                pub const m_nDamageType: usize = 0x58; // 
                pub const m_nPenetrate: usize = 0x5C; // 
                pub const m_nMaterial: usize = 0x5E; // 
                pub const m_nHitBox: usize = 0x60; // 
                pub const m_nColor: usize = 0x62; // 
                pub const m_fFlags: usize = 0x63; // 
                pub const m_nAttachmentIndex: usize = 0x64; // 
                pub const m_nAttachmentName: usize = 0x68; // 
                pub const m_iEffectName: usize = 0x6C; // 
                pub const m_nExplosionType: usize = 0x6E; // 
            }
            // Parent: C_BaseCSGrenadeProjectile
            // Field count: 0
            pub mod C_SensorGrenadeProjectile {
            }
            // Parent: C_BaseCSGrenade
            // Field count: 0
            pub mod C_MolotovGrenade {
            }
            // Parent: C_BaseToggle
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_glowEntity (CHandle<C_BaseModelEntity>)
            // NetworkVarNames: m_usable (bool)
            // NetworkVarNames: m_szDisplayText (string_t)
            pub mod C_BaseButton {
                pub const m_glowEntity: usize = 0xD28; // 
                pub const m_usable: usize = 0xD2C; // 
                pub const m_szDisplayText: usize = 0xD30; // 
            }
            // Parent: CPlayer_ViewModelServices
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_hViewModel (CHandle<CBaseViewModel>)
            pub mod CCSPlayer_ViewModelServices {
                pub const m_hViewModel: usize = 0x40; // 
            }
            // Parent: C_PointEntity
            // Field count: 0
            pub mod CInfoParticleTarget {
            }
            // Parent: CPlayer_MovementServices
            // Field count: 12
            //
            // Metadata:
            // NetworkVarNames: m_flFallVelocity (float32)
            // NetworkVarNames: m_bInCrouch (bool)
            // NetworkVarNames: m_nCrouchState (uint32)
            // NetworkVarNames: m_flCrouchTransitionStartTime (GameTime_t)
            // NetworkVarNames: m_bDucked (bool)
            // NetworkVarNames: m_bDucking (bool)
            // NetworkVarNames: m_bInDuckJump (bool)
            pub mod CPlayer_MovementServices_Humanoid {
                pub const m_flStepSoundTime: usize = 0x1D8; // 
                pub const m_flFallVelocity: usize = 0x1DC; // 
                pub const m_bInCrouch: usize = 0x1E0; // 
                pub const m_nCrouchState: usize = 0x1E4; // 
                pub const m_flCrouchTransitionStartTime: usize = 0x1E8; // 
                pub const m_bDucked: usize = 0x1EC; // 
                pub const m_bDucking: usize = 0x1ED; // 
                pub const m_bInDuckJump: usize = 0x1EE; // 
                pub const m_groundNormal: usize = 0x1F0; // 
                pub const m_flSurfaceFriction: usize = 0x1FC; // 
                pub const m_surfaceProps: usize = 0x200; // 
                pub const m_nStepside: usize = 0x210; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponAug {
            }
            // Parent: C_BaseClientUIEntity
            // Field count: 28
            //
            // Metadata:
            // NetworkVarNames: m_bIgnoreInput (bool)
            // NetworkVarNames: m_bLit (bool)
            // NetworkVarNames: m_bFollowPlayerAcrossTeleport (bool)
            // NetworkVarNames: m_flWidth (float)
            // NetworkVarNames: m_flHeight (float)
            // NetworkVarNames: m_flDPI (float)
            // NetworkVarNames: m_flInteractDistance (float)
            // NetworkVarNames: m_flDepthOffset (float)
            // NetworkVarNames: m_unOwnerContext (uint32)
            // NetworkVarNames: m_unHorizontalAlign (uint32)
            // NetworkVarNames: m_unVerticalAlign (uint32)
            // NetworkVarNames: m_unOrientation (uint32)
            // NetworkVarNames: m_bAllowInteractionFromAllSceneWorlds (bool)
            // NetworkVarNames: m_vecCSSClasses (string_t)
            // NetworkVarNames: m_bOpaque (bool)
            // NetworkVarNames: m_bNoDepth (bool)
            // NetworkVarNames: m_bRenderBackface (bool)
            // NetworkVarNames: m_bUseOffScreenIndicator (bool)
            // NetworkVarNames: m_bExcludeFromSaveGames (bool)
            // NetworkVarNames: m_bGrabbable (bool)
            // NetworkVarNames: m_bOnlyRenderToTexture (bool)
            // NetworkVarNames: m_bDisableMipGen (bool)
            // NetworkVarNames: m_nExplicitImageLayout (int32)
            pub mod C_PointClientUIWorldPanel {
                pub const m_bForceRecreateNextUpdate: usize = 0xD60; // 
                pub const m_bMoveViewToPlayerNextThink: usize = 0xD61; // 
                pub const m_bCheckCSSClasses: usize = 0xD62; // 
                pub const m_anchorDeltaTransform: usize = 0xD70; // 
                pub const m_pOffScreenIndicator: usize = 0xF08; // 
                pub const m_bIgnoreInput: usize = 0xF30; // 
                pub const m_bLit: usize = 0xF31; // 
                pub const m_bFollowPlayerAcrossTeleport: usize = 0xF32; // 
                pub const m_flWidth: usize = 0xF34; // 
                pub const m_flHeight: usize = 0xF38; // 
                pub const m_flDPI: usize = 0xF3C; // 
                pub const m_flInteractDistance: usize = 0xF40; // 
                pub const m_flDepthOffset: usize = 0xF44; // 
                pub const m_unOwnerContext: usize = 0xF48; // 
                pub const m_unHorizontalAlign: usize = 0xF4C; // 
                pub const m_unVerticalAlign: usize = 0xF50; // 
                pub const m_unOrientation: usize = 0xF54; // 
                pub const m_bAllowInteractionFromAllSceneWorlds: usize = 0xF58; // 
                pub const m_vecCSSClasses: usize = 0xF60; // 
                pub const m_bOpaque: usize = 0xF78; // 
                pub const m_bNoDepth: usize = 0xF79; // 
                pub const m_bRenderBackface: usize = 0xF7A; // 
                pub const m_bUseOffScreenIndicator: usize = 0xF7B; // 
                pub const m_bExcludeFromSaveGames: usize = 0xF7C; // 
                pub const m_bGrabbable: usize = 0xF7D; // 
                pub const m_bOnlyRenderToTexture: usize = 0xF7E; // 
                pub const m_bDisableMipGen: usize = 0xF7F; // 
                pub const m_nExplicitImageLayout: usize = 0xF80; // 
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod C_CSMinimapBoundary {
            }
            // Parent: CCSGameModeRules
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_flDMBonusStartTime (GameTime_t)
            // NetworkVarNames: m_flDMBonusTimeLength (float)
            // NetworkVarNames: m_sDMBonusWeapon (CUtlString)
            pub mod CCSGameModeRules_Deathmatch {
                pub const m_flDMBonusStartTime: usize = 0x30; // 
                pub const m_flDMBonusTimeLength: usize = 0x34; // 
                pub const m_sDMBonusWeapon: usize = 0x38; // 
            }
            // Parent: C_CSGO_TeamPreviewCharacterPosition
            // Field count: 0
            pub mod C_CSGO_EndOfMatchCharacterPosition {
            }
            // Parent: C_SoundOpvarSetPointEntity
            // Field count: 0
            pub mod C_SoundOpvarSetAABBEntity {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_nTotalPausedTicks (int)
            // NetworkVarNames: m_nPauseStartTick (int)
            // NetworkVarNames: m_bGamePaused (bool)
            pub mod C_GameRules {
                pub const __m_pChainEntity: usize = 0x8; // 
                pub const m_nTotalPausedTicks: usize = 0x30; // 
                pub const m_nPauseStartTick: usize = 0x34; // 
                pub const m_bGamePaused: usize = 0x38; // 
            }
            // Parent: C_BasePropDoor
            // Field count: 0
            pub mod C_PropDoorRotating {
            }
            // Parent: C_BaseEntity
            // Field count: 4
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_aPlayerControllers (CHandle<CBasePlayerController>)
            // NetworkVarNames: m_aPlayers (CHandle<C_BasePlayerPawn>)
            // NetworkVarNames: m_iScore (int32)
            // NetworkVarNames: m_szTeamname (char)
            pub mod C_Team {
                pub const m_aPlayerControllers: usize = 0x568; // 
                pub const m_aPlayers: usize = 0x580; // 
                pub const m_iScore: usize = 0x598; // 
                pub const m_szTeamname: usize = 0x59C; // 
            }
            // Parent: C_SoundAreaEntityBase
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_vMin (Vector)
            // NetworkVarNames: m_vMax (Vector)
            pub mod C_SoundAreaEntityOrientedBox {
                pub const m_vMin: usize = 0x590; // 
                pub const m_vMax: usize = 0x59C; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_bLoop (bool)
            // NetworkVarNames: m_flFPS (float)
            // NetworkVarNames: m_hPositionKeys (HRenderTextureStrong)
            // NetworkVarNames: m_hRotationKeys (HRenderTextureStrong)
            // NetworkVarNames: m_vAnimationBoundsMin (Vector)
            // NetworkVarNames: m_vAnimationBoundsMax (Vector)
            // NetworkVarNames: m_flStartTime (float)
            // NetworkVarNames: m_flStartFrame (float)
            pub mod C_TextureBasedAnimatable {
                pub const m_bLoop: usize = 0xD28; // 
                pub const m_flFPS: usize = 0xD2C; // 
                pub const m_hPositionKeys: usize = 0xD30; // 
                pub const m_hRotationKeys: usize = 0xD38; // 
                pub const m_vAnimationBoundsMin: usize = 0xD40; // 
                pub const m_vAnimationBoundsMax: usize = 0xD4C; // 
                pub const m_flStartTime: usize = 0xD58; // 
                pub const m_flStartFrame: usize = 0xD5C; // 
            }
            // Parent: C_SoundOpvarSetPointBase
            // Field count: 0
            pub mod C_SoundOpvarSetPointEntity {
            }
            // Parent: C_LightEntity
            // Field count: 0
            pub mod C_LightOrthoEntity {
            }
            // Parent: CPlayer_WaterServices
            // Field count: 3
            pub mod CCSPlayer_WaterServices {
                pub const m_flWaterJumpTime: usize = 0x40; // 
                pub const m_vecWaterJumpVel: usize = 0x44; // 
                pub const m_flSwimSoundTime: usize = 0x50; // 
            }
            // Parent: C_BaseEntity
            // Field count: 2
            pub mod CSkyboxReference {
                pub const m_worldGroupId: usize = 0x568; // 
                pub const m_hSkyCamera: usize = 0x56C; // 
            }
            // Parent: C_TonemapController2
            // Field count: 0
            pub mod C_TonemapController2Alias_env_tonemap_controller2 {
            }
            // Parent: None
            // Field count: 30
            //
            // Metadata:
            // NetworkVarNames: m_hTargetEntity (CHandle<C_BaseEntity>)
            // NetworkVarNames: m_bState (bool)
            // NetworkVarNames: m_bAlwaysUpdate (bool)
            // NetworkVarNames: m_flLightFOV (float32)
            // NetworkVarNames: m_bEnableShadows (bool)
            // NetworkVarNames: m_bSimpleProjection (bool)
            // NetworkVarNames: m_bLightOnlyTarget (bool)
            // NetworkVarNames: m_bLightWorld (bool)
            // NetworkVarNames: m_bCameraSpace (bool)
            // NetworkVarNames: m_flBrightnessScale (float32)
            // NetworkVarNames: m_LightColor (Color)
            // NetworkVarNames: m_flIntensity (float32)
            // NetworkVarNames: m_flLinearAttenuation (float32)
            // NetworkVarNames: m_flQuadraticAttenuation (float32)
            // NetworkVarNames: m_bVolumetric (bool)
            // NetworkVarNames: m_flVolumetricIntensity (float32)
            // NetworkVarNames: m_flNoiseStrength (float32)
            // NetworkVarNames: m_flFlashlightTime (float32)
            // NetworkVarNames: m_nNumPlanes (uint32)
            // NetworkVarNames: m_flPlaneOffset (float32)
            // NetworkVarNames: m_flColorTransitionTime (float32)
            // NetworkVarNames: m_flAmbient (float32)
            // NetworkVarNames: m_SpotlightTextureName (char)
            // NetworkVarNames: m_nSpotlightTextureFrame (int32)
            // NetworkVarNames: m_nShadowQuality (uint32)
            // NetworkVarNames: m_flNearZ (float32)
            // NetworkVarNames: m_flFarZ (float32)
            // NetworkVarNames: m_flProjectionSize (float32)
            // NetworkVarNames: m_flRotation (float32)
            // NetworkVarNames: m_bFlipHorizontal (bool)
            pub mod CProjectedTextureBase {
                pub const m_hTargetEntity: usize = 0xC; // 
                pub const m_bState: usize = 0x10; // 
                pub const m_bAlwaysUpdate: usize = 0x11; // 
                pub const m_flLightFOV: usize = 0x14; // 
                pub const m_bEnableShadows: usize = 0x18; // 
                pub const m_bSimpleProjection: usize = 0x19; // 
                pub const m_bLightOnlyTarget: usize = 0x1A; // 
                pub const m_bLightWorld: usize = 0x1B; // 
                pub const m_bCameraSpace: usize = 0x1C; // 
                pub const m_flBrightnessScale: usize = 0x20; // 
                pub const m_LightColor: usize = 0x24; // 
                pub const m_flIntensity: usize = 0x28; // 
                pub const m_flLinearAttenuation: usize = 0x2C; // 
                pub const m_flQuadraticAttenuation: usize = 0x30; // 
                pub const m_bVolumetric: usize = 0x34; // 
                pub const m_flVolumetricIntensity: usize = 0x38; // 
                pub const m_flNoiseStrength: usize = 0x3C; // 
                pub const m_flFlashlightTime: usize = 0x40; // 
                pub const m_nNumPlanes: usize = 0x44; // 
                pub const m_flPlaneOffset: usize = 0x48; // 
                pub const m_flColorTransitionTime: usize = 0x4C; // 
                pub const m_flAmbient: usize = 0x50; // 
                pub const m_SpotlightTextureName: usize = 0x54; // 
                pub const m_nSpotlightTextureFrame: usize = 0x254; // 
                pub const m_nShadowQuality: usize = 0x258; // 
                pub const m_flNearZ: usize = 0x25C; // 
                pub const m_flFarZ: usize = 0x260; // 
                pub const m_flProjectionSize: usize = 0x264; // 
                pub const m_flRotation: usize = 0x268; // 
                pub const m_bFlipHorizontal: usize = 0x26C; // 
            }
            // Parent: CPlayerControllerComponent
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_unMusicID (item_definition_index_t)
            // NetworkVarNames: m_rank (MedalRank_t)
            // NetworkVarNames: m_nPersonaDataPublicLevel (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsLeader (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsTeacher (int)
            // NetworkVarNames: m_nPersonaDataPublicCommendsFriendly (int)
            // NetworkVarNames: m_nPersonaDataXpTrailLevel (int)
            // NetworkVarNames: m_vecServerAuthoritativeWeaponSlots (ServerAuthoritativeWeaponSlot_t)
            pub mod CCSPlayerController_InventoryServices {
                pub const m_unMusicID: usize = 0x40; // 
                pub const m_rank: usize = 0x44; // 
                pub const m_nPersonaDataPublicLevel: usize = 0x5C; // 
                pub const m_nPersonaDataPublicCommendsLeader: usize = 0x60; // 
                pub const m_nPersonaDataPublicCommendsTeacher: usize = 0x64; // 
                pub const m_nPersonaDataPublicCommendsFriendly: usize = 0x68; // 
                pub const m_nPersonaDataXpTrailLevel: usize = 0x6C; // 
                pub const m_vecServerAuthoritativeWeaponSlots: usize = 0x70; // C_UtlVectorEmbeddedNetworkVar<ServerAuthoritativeWeaponSlot_t>
            }
            // Parent: C_PathParticleRope
            // Field count: 0
            pub mod C_PathParticleRopeAlias_path_particle_rope_clientside {
            }
            // Parent: CPlayerPawnComponent
            // Field count: 0
            pub mod CPlayer_UseServices {
            }
            // Parent: C_PredictedViewModel
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_bShouldIgnoreOffsetAndAccuracy (bool)
            pub mod C_CSGOViewModel {
                pub const m_bShouldIgnoreOffsetAndAccuracy: usize = 0x1021; // 
                pub const m_nLastKnownAssociatedWeaponEntIndex: usize = 0x1024; // 
                pub const m_bNeedToQueueHighResComposite: usize = 0x1028; // 
                pub const m_vLoweredWeaponOffset: usize = 0x1078; // 
            }
            // Parent: CEntityComponent
            // Field count: 1
            pub mod CScriptComponent {
                pub const m_scriptClassName: usize = 0x30; // 
            }
            // Parent: C_BaseEntity
            // Field count: 17
            //
            // Metadata:
            // NetworkVarNames: m_Entity_hLightProbeTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_hLightProbeDirectLightIndicesTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_hLightProbeDirectLightScalarsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_hLightProbeDirectLightShadowsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_vBoxMins (Vector)
            // NetworkVarNames: m_Entity_vBoxMaxs (Vector)
            // NetworkVarNames: m_Entity_bMoveable (bool)
            // NetworkVarNames: m_Entity_nHandshake (int)
            // NetworkVarNames: m_Entity_nPriority (int)
            // NetworkVarNames: m_Entity_bStartDisabled (bool)
            // NetworkVarNames: m_Entity_nLightProbeSizeX (int)
            // NetworkVarNames: m_Entity_nLightProbeSizeY (int)
            // NetworkVarNames: m_Entity_nLightProbeSizeZ (int)
            // NetworkVarNames: m_Entity_nLightProbeAtlasX (int)
            // NetworkVarNames: m_Entity_nLightProbeAtlasY (int)
            // NetworkVarNames: m_Entity_nLightProbeAtlasZ (int)
            // NetworkVarNames: m_Entity_bEnabled (bool)
            pub mod C_EnvLightProbeVolume {
                pub const m_Entity_hLightProbeTexture: usize = 0x1548; // 
                pub const m_Entity_hLightProbeDirectLightIndicesTexture: usize = 0x1550; // 
                pub const m_Entity_hLightProbeDirectLightScalarsTexture: usize = 0x1558; // 
                pub const m_Entity_hLightProbeDirectLightShadowsTexture: usize = 0x1560; // 
                pub const m_Entity_vBoxMins: usize = 0x1568; // 
                pub const m_Entity_vBoxMaxs: usize = 0x1574; // 
                pub const m_Entity_bMoveable: usize = 0x1580; // 
                pub const m_Entity_nHandshake: usize = 0x1584; // 
                pub const m_Entity_nPriority: usize = 0x1588; // 
                pub const m_Entity_bStartDisabled: usize = 0x158C; // 
                pub const m_Entity_nLightProbeSizeX: usize = 0x1590; // 
                pub const m_Entity_nLightProbeSizeY: usize = 0x1594; // 
                pub const m_Entity_nLightProbeSizeZ: usize = 0x1598; // 
                pub const m_Entity_nLightProbeAtlasX: usize = 0x159C; // 
                pub const m_Entity_nLightProbeAtlasY: usize = 0x15A0; // 
                pub const m_Entity_nLightProbeAtlasZ: usize = 0x15A4; // 
                pub const m_Entity_bEnabled: usize = 0x15B1; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 2
            pub mod C_ViewmodelAttachmentModel {
                pub const m_bShouldFrontFaceCullLeftHanded: usize = 0xF90; // 
                pub const m_bCreatedLeftHanded: usize = 0xF91; // 
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSPointScriptExtensions_player_controller {
            }
            // Parent: C_BaseCSGrenade
            // Field count: 0
            pub mod C_Flashbang {
            }
            // Parent: C_CSWeaponBase
            // Field count: 0
            pub mod CBumpMine {
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // NetworkVarNames: m_iKills (int)
            // NetworkVarNames: m_iDeaths (int)
            // NetworkVarNames: m_iAssists (int)
            // NetworkVarNames: m_iDamage (int)
            // NetworkVarNames: m_iEquipmentValue (int)
            // NetworkVarNames: m_iMoneySaved (int)
            // NetworkVarNames: m_iKillReward (int)
            // NetworkVarNames: m_iLiveTime (int)
            // NetworkVarNames: m_iHeadShotKills (int)
            // NetworkVarNames: m_iObjective (int)
            // NetworkVarNames: m_iCashEarned (int)
            // NetworkVarNames: m_iUtilityDamage (int)
            // NetworkVarNames: m_iEnemiesFlashed (int)
            pub mod CSPerRoundStats_t {
                pub const m_iKills: usize = 0x30; // 
                pub const m_iDeaths: usize = 0x34; // 
                pub const m_iAssists: usize = 0x38; // 
                pub const m_iDamage: usize = 0x3C; // 
                pub const m_iEquipmentValue: usize = 0x40; // 
                pub const m_iMoneySaved: usize = 0x44; // 
                pub const m_iKillReward: usize = 0x48; // 
                pub const m_iLiveTime: usize = 0x4C; // 
                pub const m_iHeadShotKills: usize = 0x50; // 
                pub const m_iObjective: usize = 0x54; // 
                pub const m_iCashEarned: usize = 0x58; // 
                pub const m_iUtilityDamage: usize = 0x5C; // 
                pub const m_iEnemiesFlashed: usize = 0x60; // 
            }
            // Parent: C_BaseEntity
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_iCurrentMaxRagdollCount (int8)
            pub mod CRagdollManager {
                pub const m_iCurrentMaxRagdollCount: usize = 0x568; // 
            }
            // Parent: C_BaseEntity
            // Field count: 30
            //
            // Metadata:
            // NetworkVarNames: m_bTimerPaused (bool)
            // NetworkVarNames: m_flTimeRemaining (float)
            // NetworkVarNames: m_flTimerEndTime (GameTime_t)
            // NetworkVarNames: m_bIsDisabled (bool)
            // NetworkVarNames: m_bShowInHUD (bool)
            // NetworkVarNames: m_nTimerLength (int)
            // NetworkVarNames: m_nTimerInitialLength (int)
            // NetworkVarNames: m_nTimerMaxLength (int)
            // NetworkVarNames: m_bAutoCountdown (bool)
            // NetworkVarNames: m_nSetupTimeLength (int)
            // NetworkVarNames: m_nState (int)
            // NetworkVarNames: m_bStartPaused (bool)
            // NetworkVarNames: m_bInCaptureWatchState (bool)
            // NetworkVarNames: m_flTotalTime (float)
            // NetworkVarNames: m_bStopWatchTimer (bool)
            pub mod C_TeamRoundTimer {
                pub const m_bTimerPaused: usize = 0x568; // 
                pub const m_flTimeRemaining: usize = 0x56C; // 
                pub const m_flTimerEndTime: usize = 0x570; // 
                pub const m_bIsDisabled: usize = 0x574; // 
                pub const m_bShowInHUD: usize = 0x575; // 
                pub const m_nTimerLength: usize = 0x578; // 
                pub const m_nTimerInitialLength: usize = 0x57C; // 
                pub const m_nTimerMaxLength: usize = 0x580; // 
                pub const m_bAutoCountdown: usize = 0x584; // 
                pub const m_nSetupTimeLength: usize = 0x588; // 
                pub const m_nState: usize = 0x58C; // 
                pub const m_bStartPaused: usize = 0x590; // 
                pub const m_bInCaptureWatchState: usize = 0x591; // 
                pub const m_flTotalTime: usize = 0x594; // 
                pub const m_bStopWatchTimer: usize = 0x598; // 
                pub const m_bFireFinished: usize = 0x599; // 
                pub const m_bFire5MinRemain: usize = 0x59A; // 
                pub const m_bFire4MinRemain: usize = 0x59B; // 
                pub const m_bFire3MinRemain: usize = 0x59C; // 
                pub const m_bFire2MinRemain: usize = 0x59D; // 
                pub const m_bFire1MinRemain: usize = 0x59E; // 
                pub const m_bFire30SecRemain: usize = 0x59F; // 
                pub const m_bFire10SecRemain: usize = 0x5A0; // 
                pub const m_bFire5SecRemain: usize = 0x5A1; // 
                pub const m_bFire4SecRemain: usize = 0x5A2; // 
                pub const m_bFire3SecRemain: usize = 0x5A3; // 
                pub const m_bFire2SecRemain: usize = 0x5A4; // 
                pub const m_bFire1SecRemain: usize = 0x5A5; // 
                pub const m_nOldTimerLength: usize = 0x5A8; // 
                pub const m_nOldTimerState: usize = 0x5AC; // 
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_nItemDefIndex (uint16)
            // NetworkVarNames: m_nCount (uint16)
            pub mod WeaponPurchaseCount_t {
                pub const m_nItemDefIndex: usize = 0x30; // 
                pub const m_nCount: usize = 0x32; // 
            }
            // Parent: C_BaseCSGrenade
            // Field count: 0
            pub mod C_SmokeGrenade {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponGalilAR {
            }
            // Parent: CBaseAnimGraph
            // Field count: 0
            pub mod C_WorldModelGloves {
            }
            // Parent: C_ModelPointEntity
            // Field count: 0
            pub mod C_EnvProjectedTexture {
            }
            // Parent: C_BaseEntity
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_pathString (CUtlString)
            pub mod CPathSimple {
                pub const m_pathString: usize = 0x5C0; // 
            }
            // Parent: CPlayer_CameraServices
            // Field count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iFOV (uint32)
            // NetworkVarNames: m_iFOVStart (uint32)
            // NetworkVarNames: m_flFOVTime (GameTime_t)
            // NetworkVarNames: m_flFOVRate (float32)
            // NetworkVarNames: m_hZoomOwner (CHandle<CBaseEntity>)
            pub mod CCSPlayerBase_CameraServices {
                pub const m_iFOV: usize = 0x210; // 
                pub const m_iFOVStart: usize = 0x214; // 
                pub const m_flFOVTime: usize = 0x218; // 
                pub const m_flFOVRate: usize = 0x21C; // 
                pub const m_hZoomOwner: usize = 0x220; // 
                pub const m_flLastShotFOV: usize = 0x224; // 
            }
            // Parent: C_PointClientUIWorldPanel
            // Field count: 4
            pub mod CPointOffScreenIndicatorUi {
                pub const m_bBeenEnabled: usize = 0xF90; // 
                pub const m_bHide: usize = 0xF91; // 
                pub const m_flSeenTargetTime: usize = 0xF94; // 
                pub const m_pTargetPanel: usize = 0xF98; // 
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSPointScriptExtensions_entity {
            }
            // Parent: CBodyComponentSkeletonInstance
            // Field count: 0
            pub mod CBodyComponentBaseModelEntity {
            }
            // Parent: C_CSWeaponBase
            // Field count: 6
            //
            // Metadata:
            // NetworkVarNames: m_zoomLevel (int)
            // NetworkVarNames: m_iBurstShotsRemaining (int)
            // NetworkVarNames: m_bNeedsBoltAction (bool)
            pub mod C_CSWeaponBaseGun {
                pub const m_zoomLevel: usize = 0x1B10; // 
                pub const m_iBurstShotsRemaining: usize = 0x1B14; // 
                pub const m_iSilencerBodygroup: usize = 0x1B18; // 
                pub const m_silencedModelIndex: usize = 0x1B28; // 
                pub const m_inPrecache: usize = 0x1B2C; // 
                pub const m_bNeedsBoltAction: usize = 0x1B2D; // 
            }
            // Parent: C_BaseEntity
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_skyboxData (sky3dparams_t)
            // NetworkVarNames: m_skyboxSlotToken (CUtlStringToken)
            pub mod C_SkyCamera {
                pub const m_skyboxData: usize = 0x568; // 
                pub const m_skyboxSlotToken: usize = 0x5F8; // 
                pub const m_bUseAngles: usize = 0x5FC; // 
                pub const m_pNext: usize = 0x600; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod C_World {
            }
            // Parent: C_BaseModelEntity
            // Field count: 13
            //
            // Metadata:
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_flFadeInStart (float32)
            // NetworkVarNames: m_flFadeInLength (float32)
            // NetworkVarNames: m_flFadeOutModelStart (float32)
            // NetworkVarNames: m_flFadeOutModelLength (float32)
            // NetworkVarNames: m_flFadeOutStart (float32)
            // NetworkVarNames: m_flFadeOutLength (float32)
            // NetworkVarNames: m_nDissolveType (EntityDisolveType_t)
            // NetworkVarNames: m_vDissolverOrigin (Vector)
            // NetworkVarNames: m_nMagnitude (uint32)
            pub mod C_EntityDissolve {
                pub const m_flStartTime: usize = 0xD30; // 
                pub const m_flFadeInStart: usize = 0xD34; // 
                pub const m_flFadeInLength: usize = 0xD38; // 
                pub const m_flFadeOutModelStart: usize = 0xD3C; // 
                pub const m_flFadeOutModelLength: usize = 0xD40; // 
                pub const m_flFadeOutStart: usize = 0xD44; // 
                pub const m_flFadeOutLength: usize = 0xD48; // 
                pub const m_flNextSparkTime: usize = 0xD4C; // 
                pub const m_nDissolveType: usize = 0xD50; // 
                pub const m_vDissolverOrigin: usize = 0xD54; // 
                pub const m_nMagnitude: usize = 0xD60; // 
                pub const m_bCoreExplode: usize = 0xD64; // 
                pub const m_bLinkedToServerEnt: usize = 0xD65; // 
            }
            // Parent: CPlayer_UseServices
            // Field count: 0
            pub mod CCSObserver_UseServices {
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // NetworkVarNames: m_hCtrl (CHandle<CFogController>)
            pub mod C_fogplayerparams_t {
                pub const m_hCtrl: usize = 0x8; // 
                pub const m_flTransitionTime: usize = 0xC; // 
                pub const m_OldColor: usize = 0x10; // 
                pub const m_flOldStart: usize = 0x14; // 
                pub const m_flOldEnd: usize = 0x18; // 
                pub const m_flOldMaxDensity: usize = 0x1C; // 
                pub const m_flOldHDRColorScale: usize = 0x20; // 
                pub const m_flOldFarZ: usize = 0x24; // 
                pub const m_NewColor: usize = 0x28; // 
                pub const m_flNewStart: usize = 0x2C; // 
                pub const m_flNewEnd: usize = 0x30; // 
                pub const m_flNewMaxDensity: usize = 0x34; // 
                pub const m_flNewHDRColorScale: usize = 0x38; // 
                pub const m_flNewFarZ: usize = 0x3C; // 
            }
            // Parent: C_BasePlayerPawn
            // Field count: 65
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_pPingServices (CCSPlayer_PingServices*)
            // NetworkVarNames: m_pViewModelServices (CPlayer_ViewModelServices*)
            // NetworkVarNames: m_iPlayerState (CSPlayerState)
            // NetworkVarNames: m_bIsRescuing (bool)
            // NetworkVarNames: m_fImmuneToGunGameDamageTime (GameTime_t)
            // NetworkVarNames: m_bGunGameImmunity (bool)
            // NetworkVarNames: m_bHasMovedSinceSpawn (bool)
            // NetworkVarNames: m_fMolotovUseTime (float)
            // NetworkVarNames: m_fMolotovDamageTime (float)
            // NetworkVarNames: m_iThrowGrenadeCounter (int)
            // NetworkVarNames: m_iProgressBarDuration (int)
            // NetworkVarNames: m_flProgressBarStartTime (float)
            // NetworkVarNames: m_flFlashMaxAlpha (float)
            // NetworkVarNames: m_flFlashDuration (float)
            // NetworkVarNames: m_cycleLatch (int)
            // NetworkVarNames: m_hOriginalController (CHandle<CCSPlayerController>)
            pub mod C_CSPlayerPawnBase {
                pub const m_pPingServices: usize = 0x1360; // 
                pub const m_pViewModelServices: usize = 0x1368; // 
                pub const m_fRenderingClipPlane: usize = 0x1370; // 
                pub const m_nLastClipPlaneSetupFrame: usize = 0x1380; // 
                pub const m_vecLastClipCameraPos: usize = 0x1384; // 
                pub const m_vecLastClipCameraForward: usize = 0x1390; // 
                pub const m_bClipHitStaticWorld: usize = 0x139C; // 
                pub const m_bCachedPlaneIsValid: usize = 0x139D; // 
                pub const m_pClippingWeapon: usize = 0x13A0; // 
                pub const m_previousPlayerState: usize = 0x13A8; // 
                pub const m_iPlayerState: usize = 0x13AC; // 
                pub const m_bIsRescuing: usize = 0x13B0; // 
                pub const m_fImmuneToGunGameDamageTime: usize = 0x13B4; // 
                pub const m_fImmuneToGunGameDamageTimeLast: usize = 0x13B8; // 
                pub const m_bGunGameImmunity: usize = 0x13BC; // 
                pub const m_bHasMovedSinceSpawn: usize = 0x13BD; // 
                pub const m_fMolotovUseTime: usize = 0x13C0; // 
                pub const m_fMolotovDamageTime: usize = 0x13C4; // 
                pub const m_iThrowGrenadeCounter: usize = 0x13C8; // 
                pub const m_flLastSpawnTimeIndex: usize = 0x13CC; // 
                pub const m_iProgressBarDuration: usize = 0x13D0; // 
                pub const m_flProgressBarStartTime: usize = 0x13D4; // 
                pub const m_vecIntroStartEyePosition: usize = 0x13D8; // 
                pub const m_vecIntroStartPlayerForward: usize = 0x13E4; // 
                pub const m_flClientDeathTime: usize = 0x13F0; // 
                pub const m_bScreenTearFrameCaptured: usize = 0x13F4; // 
                pub const m_flFlashBangTime: usize = 0x13F8; // 
                pub const m_flFlashScreenshotAlpha: usize = 0x13FC; // 
                pub const m_flFlashOverlayAlpha: usize = 0x1400; // 
                pub const m_bFlashBuildUp: usize = 0x1404; // 
                pub const m_bFlashDspHasBeenCleared: usize = 0x1405; // 
                pub const m_bFlashScreenshotHasBeenGrabbed: usize = 0x1406; // 
                pub const m_flFlashMaxAlpha: usize = 0x1408; // 
                pub const m_flFlashDuration: usize = 0x140C; // 
                pub const m_iHealthBarRenderMaskIndex: usize = 0x1410; // 
                pub const m_flHealthFadeValue: usize = 0x1414; // 
                pub const m_flHealthFadeAlpha: usize = 0x1418; // 
                pub const m_flDeathCCWeight: usize = 0x1428; // 
                pub const m_flPrevRoundEndTime: usize = 0x142C; // 
                pub const m_flPrevMatchEndTime: usize = 0x1430; // 
                pub const m_angEyeAngles: usize = 0x1438; // 
                pub const m_fNextThinkPushAway: usize = 0x1450; // 
                pub const m_bShouldAutobuyDMWeapons: usize = 0x1454; // 
                pub const m_bShouldAutobuyNow: usize = 0x1455; // 
                pub const m_iIDEntIndex: usize = 0x1458; // 
                pub const m_delayTargetIDTimer: usize = 0x1460; // 
                pub const m_iTargetItemEntIdx: usize = 0x1478; // 
                pub const m_iOldIDEntIndex: usize = 0x147C; // 
                pub const m_holdTargetIDTimer: usize = 0x1480; // 
                pub const m_flCurrentMusicStartTime: usize = 0x149C; // 
                pub const m_flMusicRoundStartTime: usize = 0x14A0; // 
                pub const m_bDeferStartMusicOnWarmup: usize = 0x14A4; // 
                pub const m_cycleLatch: usize = 0x14A8; // 
                pub const m_serverIntendedCycle: usize = 0x14AC; // 
                pub const m_flLastSmokeOverlayAlpha: usize = 0x14B0; // 
                pub const m_flLastSmokeAge: usize = 0x14B4; // 
                pub const m_vLastSmokeOverlayColor: usize = 0x14B8; // 
                pub const m_nPlayerSmokedFx: usize = 0x14C4; // 
                pub const m_nPlayerInfernoBodyFx: usize = 0x14C8; // 
                pub const m_nPlayerInfernoFootFx: usize = 0x14CC; // 
                pub const m_flNextMagDropTime: usize = 0x14D0; // 
                pub const m_nLastMagDropAttachmentIndex: usize = 0x14D4; // 
                pub const m_vecLastAliveLocalVelocity: usize = 0x14D8; // 
                pub const m_bGuardianShouldSprayCustomXMark: usize = 0x1500; // 
                pub const m_hOriginalController: usize = 0x1508; // 
            }
            // Parent: C_WeaponBaseItem
            // Field count: 0
            pub mod C_Item_Healthshot {
            }
            // Parent: None
            // Field count: 29
            //
            // Metadata:
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_iWindSeed (uint32)
            // NetworkVarNames: m_iMinWind (uint16)
            // NetworkVarNames: m_iMaxWind (uint16)
            // NetworkVarNames: m_windRadius (int32)
            // NetworkVarNames: m_iMinGust (uint16)
            // NetworkVarNames: m_iMaxGust (uint16)
            // NetworkVarNames: m_flMinGustDelay (float32)
            // NetworkVarNames: m_flMaxGustDelay (float32)
            // NetworkVarNames: m_flGustDuration (float32)
            // NetworkVarNames: m_iGustDirChange (uint16)
            // NetworkVarNames: m_location (Vector)
            // NetworkVarNames: m_iInitialWindDir (uint16)
            // NetworkVarNames: m_flInitialWindSpeed (float32)
            pub mod C_EnvWindShared {
                pub const m_flStartTime: usize = 0x8; // 
                pub const m_iWindSeed: usize = 0xC; // 
                pub const m_iMinWind: usize = 0x10; // 
                pub const m_iMaxWind: usize = 0x12; // 
                pub const m_windRadius: usize = 0x14; // 
                pub const m_iMinGust: usize = 0x18; // 
                pub const m_iMaxGust: usize = 0x1A; // 
                pub const m_flMinGustDelay: usize = 0x1C; // 
                pub const m_flMaxGustDelay: usize = 0x20; // 
                pub const m_flGustDuration: usize = 0x24; // 
                pub const m_iGustDirChange: usize = 0x28; // 
                pub const m_location: usize = 0x2C; // 
                pub const m_iszGustSound: usize = 0x38; // 
                pub const m_iWindDir: usize = 0x3C; // 
                pub const m_flWindSpeed: usize = 0x40; // 
                pub const m_currentWindVector: usize = 0x44; // 
                pub const m_CurrentSwayVector: usize = 0x50; // 
                pub const m_PrevSwayVector: usize = 0x5C; // 
                pub const m_iInitialWindDir: usize = 0x68; // 
                pub const m_flInitialWindSpeed: usize = 0x6C; // 
                pub const m_flVariationTime: usize = 0x70; // 
                pub const m_flSwayTime: usize = 0x74; // 
                pub const m_flSimTime: usize = 0x78; // 
                pub const m_flSwitchTime: usize = 0x7C; // 
                pub const m_flAveWindSpeed: usize = 0x80; // 
                pub const m_bGusting: usize = 0x84; // 
                pub const m_flWindAngleVariation: usize = 0x88; // 
                pub const m_flWindSpeedVariation: usize = 0x8C; // 
                pub const m_hEntOwner: usize = 0x90; // 
            }
            // Parent: C_BaseTrigger
            // Field count: 16
            //
            // Metadata:
            // NetworkVarNames: m_hPostSettings (HPostProcessingStrong)
            // NetworkVarNames: m_flFadeDuration (float)
            // NetworkVarNames: m_flMinLogExposure (float)
            // NetworkVarNames: m_flMaxLogExposure (float)
            // NetworkVarNames: m_flMinExposure (float)
            // NetworkVarNames: m_flMaxExposure (float)
            // NetworkVarNames: m_flExposureCompensation (float)
            // NetworkVarNames: m_flExposureFadeSpeedUp (float)
            // NetworkVarNames: m_flExposureFadeSpeedDown (float)
            // NetworkVarNames: m_flTonemapEVSmoothingRange (float)
            // NetworkVarNames: m_bMaster (bool)
            // NetworkVarNames: m_bExposureControl (bool)
            // NetworkVarNames: m_flRate (float)
            // NetworkVarNames: m_flTonemapPercentTarget (float)
            // NetworkVarNames: m_flTonemapPercentBrightPixels (float)
            // NetworkVarNames: m_flTonemapMinAvgLum (float)
            pub mod C_PostProcessingVolume {
                pub const m_hPostSettings: usize = 0xD40; // 
                pub const m_flFadeDuration: usize = 0xD48; // 
                pub const m_flMinLogExposure: usize = 0xD4C; // 
                pub const m_flMaxLogExposure: usize = 0xD50; // 
                pub const m_flMinExposure: usize = 0xD54; // 
                pub const m_flMaxExposure: usize = 0xD58; // 
                pub const m_flExposureCompensation: usize = 0xD5C; // 
                pub const m_flExposureFadeSpeedUp: usize = 0xD60; // 
                pub const m_flExposureFadeSpeedDown: usize = 0xD64; // 
                pub const m_flTonemapEVSmoothingRange: usize = 0xD68; // 
                pub const m_bMaster: usize = 0xD6C; // 
                pub const m_bExposureControl: usize = 0xD6D; // 
                pub const m_flRate: usize = 0xD70; // 
                pub const m_flTonemapPercentTarget: usize = 0xD74; // 
                pub const m_flTonemapPercentBrightPixels: usize = 0xD78; // 
                pub const m_flTonemapMinAvgLum: usize = 0xD7C; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_hPlayerPing (CHandle<CBaseEntity>)
            pub mod CCSPlayer_PingServices {
                pub const m_hPlayerPing: usize = 0x40; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 0
            pub mod CPlayer_FlashlightServices {
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod CServerOnlyModelEntity {
            }
            // Parent: CAttributeManager
            // Field count: 3
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_Item (CEconItemView)
            pub mod C_AttributeContainer {
                pub const m_Item: usize = 0x50; // 
                pub const m_iExternalItemProviderRegisteredToken: usize = 0x498; // 
                pub const m_ullRegisteredAsItemID: usize = 0x4A0; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            //
            // Metadata:
            // MNetworkOverride
            // MNetworkOverride
            pub mod C_FuncRotating {
            }
            // Parent: C_BaseEntity
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_flScale (float32)
            // NetworkVarNames: m_flStartScale (float32)
            // NetworkVarNames: m_flScaleTime (float)
            // NetworkVarNames: m_nFlags (uint32)
            pub mod C_BaseFire {
                pub const m_flScale: usize = 0x568; // 
                pub const m_flStartScale: usize = 0x56C; // 
                pub const m_flScaleTime: usize = 0x570; // 
                pub const m_nFlags: usize = 0x574; // 
            }
            // Parent: C_BaseEntity
            // Field count: 16
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // MNetworkIncludeByUserGroup
            // NetworkVarNames: m_nTickBase (uint32)
            // NetworkVarNames: m_hPawn (CHandle<CBasePlayerPawn>)
            // NetworkVarNames: m_bKnownTeamMismatch (bool)
            // NetworkVarNames: m_iConnected (PlayerConnectedState)
            // NetworkVarNames: m_iszPlayerName (char)
            // NetworkVarNames: m_steamID (uint64)
            // NetworkVarNames: m_iDesiredFOV (uint32)
            // MNetworkReplayCompatField
            pub mod CBasePlayerController {
                pub const m_nFinalPredictedTick: usize = 0x570; // 
                pub const m_CommandContext: usize = 0x578; // 
                pub const m_nInButtonsWhichAreToggles: usize = 0x620; // 
                pub const m_nTickBase: usize = 0x628; // 
                pub const m_hPawn: usize = 0x62C; // 
                pub const m_bKnownTeamMismatch: usize = 0x630; // 
                pub const m_hPredictedPawn: usize = 0x634; // 
                pub const m_nSplitScreenSlot: usize = 0x638; // 
                pub const m_hSplitOwner: usize = 0x63C; // CHandle<CBasePlayerController>
                pub const m_hSplitScreenPlayers: usize = 0x640; // CUtlVector<CHandle<CBasePlayerController>>
                pub const m_bIsHLTV: usize = 0x658; // 
                pub const m_iConnected: usize = 0x65C; // 
                pub const m_iszPlayerName: usize = 0x660; // 
                pub const m_steamID: usize = 0x6E8; // 
                pub const m_bIsLocalPlayerController: usize = 0x6F0; // 
                pub const m_iDesiredFOV: usize = 0x6F4; // 
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CPointTemplateAPI {
            }
            // Parent: C_BaseCSGrenadeProjectile
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_nDecoyShotTick (int)
            pub mod C_DecoyProjectile {
                pub const m_nDecoyShotTick: usize = 0x1208; // 
                pub const m_nClientLastKnownDecoyShotTick: usize = 0x120C; // 
                pub const m_flTimeParticleEffectSpawn: usize = 0x1230; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 0
            pub mod C_WaterBullet {
            }
            // Parent: C_CSPlayerPawn
            // Field count: 3
            pub mod C_CSGO_PreviewPlayer {
                pub const m_animgraph: usize = 0x3A30; // 
                pub const m_animgraphCharacterModeString: usize = 0x3A38; // 
                pub const m_flInitialModelScale: usize = 0x3A40; // 
            }
            // Parent: CSkeletonAnimationController
            // Field count: 14
            //
            // Metadata:
            // NetworkVarNames: m_animGraphNetworkedVars (CAnimGraphNetworkedVariables)
            // NetworkVarNames: m_hSequence (HSequence)
            // NetworkVarNames: m_flSeqStartTime (GameTime_t)
            // NetworkVarNames: m_flSeqFixedCycle (float)
            // NetworkVarNames: m_nAnimLoopMode (AnimLoopMode_t)
            pub mod CBaseAnimGraphController {
                pub const m_animGraphNetworkedVars: usize = 0x18; // 
                pub const m_bSequenceFinished: usize = 0x14A8; // 
                pub const m_flSoundSyncTime: usize = 0x14AC; // 
                pub const m_nActiveIKChainMask: usize = 0x14B0; // 
                pub const m_hSequence: usize = 0x14B4; // 
                pub const m_flSeqStartTime: usize = 0x14B8; // 
                pub const m_flSeqFixedCycle: usize = 0x14BC; // 
                pub const m_nAnimLoopMode: usize = 0x14C0; // 
                pub const m_flPlaybackRate: usize = 0x14C4; // 
                pub const m_nNotifyState: usize = 0x14D0; // 
                pub const m_bNetworkedAnimationInputsChanged: usize = 0x14D2; // 
                pub const m_bNetworkedSequenceChanged: usize = 0x14D3; // 
                pub const m_bLastUpdateSkipped: usize = 0x14D4; // 
                pub const m_flPrevAnimUpdateTime: usize = 0x14D8; // 
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod CCSPointScriptEntity {
            }
            // Parent: C_CSWeaponBase
            // Field count: 11
            //
            // Metadata:
            // NetworkVarNames: m_bStartedArming (bool)
            // NetworkVarNames: m_fArmedTime (GameTime_t)
            // NetworkVarNames: m_bBombPlacedAnimation (bool)
            // NetworkVarNames: m_bIsPlantingViaUse (bool)
            // NetworkVarNames: m_entitySpottedState (EntitySpottedState_t)
            pub mod C_C4 {
                pub const m_szScreenText: usize = 0x1B10; // 
                pub const m_activeLightParticleIndex: usize = 0x1B30; // 
                pub const m_eActiveLightEffect: usize = 0x1B34; // 
                pub const m_bStartedArming: usize = 0x1B38; // 
                pub const m_fArmedTime: usize = 0x1B3C; // 
                pub const m_bBombPlacedAnimation: usize = 0x1B40; // 
                pub const m_bIsPlantingViaUse: usize = 0x1B41; // 
                pub const m_entitySpottedState: usize = 0x1B48; // 
                pub const m_nSpotRules: usize = 0x1B60; // 
                pub const m_bPlayedArmingBeeps: usize = 0x1B64; // 
                pub const m_bBombPlanted: usize = 0x1B6B; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 13
            //
            // Metadata:
            // MNetworkIncludeByName
            // NetworkVarNames: m_bInitiallyPopulateInterpHistory (bool)
            // NetworkVarNames: m_bAnimGraphUpdateEnabled (bool)
            // NetworkVarNames: m_vecForce (Vector)
            // NetworkVarNames: m_nForceBone (int32)
            // NetworkVarNames: m_RagdollPose (PhysicsRagdollPose_t)
            // NetworkVarNames: m_bRagdollClientSide (bool)
            pub mod CBaseAnimGraph {
                pub const m_bInitiallyPopulateInterpHistory: usize = 0xDA0; // 
                pub const m_bSuppressAnimEventSounds: usize = 0xDA2; // 
                pub const m_bAnimGraphUpdateEnabled: usize = 0xDB0; // 
                pub const m_flMaxSlopeDistance: usize = 0xDB4; // 
                pub const m_vLastSlopeCheckPos: usize = 0xDB8; // 
                pub const m_bAnimationUpdateScheduled: usize = 0xDC4; // 
                pub const m_vecForce: usize = 0xDC8; // 
                pub const m_nForceBone: usize = 0xDD4; // 
                pub const m_pClientsideRagdoll: usize = 0xDD8; // 
                pub const m_bBuiltRagdoll: usize = 0xDE0; // 
                pub const m_RagdollPose: usize = 0xDF8; // 
                pub const m_bRagdollClientSide: usize = 0xE40; // 
                pub const m_bHasAnimatedMaterialAttributes: usize = 0xE50; // 
            }
            // Parent: C_CSWeaponBase
            // Field count: 0
            pub mod C_Melee {
            }
            // Parent: C_BaseGrenade
            // Field count: 0
            pub mod CBreachChargeProjectile {
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod C_PointEntity {
            }
            // Parent: C_GameRules
            // Field count: 0
            pub mod C_MultiplayRules {
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CBasePlayerControllerAPI {
            }
            // Parent: C_BaseEntity
            // Field count: 16
            //
            // Metadata:
            // NetworkVarNames: m_flParticleSpacing (float)
            // NetworkVarNames: m_flSlack (float)
            // NetworkVarNames: m_flRadius (float)
            // NetworkVarNames: m_ColorTint (Color)
            // NetworkVarNames: m_nEffectState (int)
            // NetworkVarNames: m_iEffectIndex (HParticleSystemDefinitionStrong)
            // NetworkVarNames: m_PathNodes_Position (Vector)
            // NetworkVarNames: m_PathNodes_TangentIn (Vector)
            // NetworkVarNames: m_PathNodes_TangentOut (Vector)
            // NetworkVarNames: m_PathNodes_Color (Vector)
            // NetworkVarNames: m_PathNodes_PinEnabled (bool)
            // NetworkVarNames: m_PathNodes_RadiusScale (float)
            pub mod C_PathParticleRope {
                pub const m_bStartActive: usize = 0x570; // 
                pub const m_flMaxSimulationTime: usize = 0x574; // 
                pub const m_iszEffectName: usize = 0x578; // 
                pub const m_PathNodes_Name: usize = 0x580; // 
                pub const m_flParticleSpacing: usize = 0x598; // 
                pub const m_flSlack: usize = 0x59C; // 
                pub const m_flRadius: usize = 0x5A0; // 
                pub const m_ColorTint: usize = 0x5A4; // 
                pub const m_nEffectState: usize = 0x5A8; // 
                pub const m_iEffectIndex: usize = 0x5B0; // 
                pub const m_PathNodes_Position: usize = 0x5B8; // C_NetworkUtlVectorBase<Vector>
                pub const m_PathNodes_TangentIn: usize = 0x5D0; // C_NetworkUtlVectorBase<Vector>
                pub const m_PathNodes_TangentOut: usize = 0x5E8; // C_NetworkUtlVectorBase<Vector>
                pub const m_PathNodes_Color: usize = 0x600; // 
                pub const m_PathNodes_PinEnabled: usize = 0x618; // 
                pub const m_PathNodes_RadiusScale: usize = 0x630; // 
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSPointScriptExtensions_CCSWeaponBaseVData {
            }
            // Parent: C_SoundEventEntity
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_vMins (Vector)
            // NetworkVarNames: m_vMaxs (Vector)
            pub mod C_SoundEventAABBEntity {
                pub const m_vMins: usize = 0x628; // 
                pub const m_vMaxs: usize = 0x634; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponP90 {
            }
            // Parent: C_EconEntity
            // Field count: 7
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkIncludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkUserGroupProxy
            // NetworkVarNames: m_nNextPrimaryAttackTick (GameTick_t)
            // NetworkVarNames: m_flNextPrimaryAttackTickRatio (float32)
            // NetworkVarNames: m_nNextSecondaryAttackTick (GameTick_t)
            // NetworkVarNames: m_flNextSecondaryAttackTickRatio (float32)
            // NetworkVarNames: m_iClip1 (int32)
            // NetworkVarNames: m_iClip2 (int32)
            // NetworkVarNames: m_pReserveAmmo (int)
            pub mod C_BasePlayerWeapon {
                pub const m_nNextPrimaryAttackTick: usize = 0x1668; // 
                pub const m_flNextPrimaryAttackTickRatio: usize = 0x166C; // 
                pub const m_nNextSecondaryAttackTick: usize = 0x1670; // 
                pub const m_flNextSecondaryAttackTickRatio: usize = 0x1674; // 
                pub const m_iClip1: usize = 0x1678; // 
                pub const m_iClip2: usize = 0x167C; // 
                pub const m_pReserveAmmo: usize = 0x1680; // 
            }
            // Parent: CEntitySubclassVDataBase
            // Field count: 22
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CBasePlayerWeaponVData {
                pub const m_szWorldModel: usize = 0x28; // 
                pub const m_sToolsOnlyOwnerModelName: usize = 0x108; // 
                pub const m_bBuiltRightHanded: usize = 0x1E8; // 
                pub const m_bAllowFlipping: usize = 0x1E9; // 
                pub const m_sMuzzleAttachment: usize = 0x1F0; // CAttachmentNameSymbolWithStorage
                pub const m_szMuzzleFlashParticle: usize = 0x210; // 
                pub const m_bLinkedCooldowns: usize = 0x2F0; // 
                pub const m_iFlags: usize = 0x2F1; // 
                pub const m_nPrimaryAmmoType: usize = 0x2F2; // 
                pub const m_nSecondaryAmmoType: usize = 0x2F3; // 
                pub const m_iMaxClip1: usize = 0x2F4; // 
                pub const m_iMaxClip2: usize = 0x2F8; // 
                pub const m_iDefaultClip1: usize = 0x2FC; // 
                pub const m_iDefaultClip2: usize = 0x300; // 
                pub const m_bReserveAmmoAsClips: usize = 0x304; // 
                pub const m_iWeight: usize = 0x308; // 
                pub const m_bAutoSwitchTo: usize = 0x30C; // 
                pub const m_bAutoSwitchFrom: usize = 0x30D; // 
                pub const m_iRumbleEffect: usize = 0x310; // 
                pub const m_iSlot: usize = 0x314; // 
                pub const m_iPosition: usize = 0x318; // 
                pub const m_aShootSounds: usize = 0x320; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 23
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_poolOrigin (Vector)
            // NetworkVarNames: m_waterLevel (float32)
            // NetworkVarNames: m_x (float32)
            // NetworkVarNames: m_y (float32)
            // NetworkVarNames: m_z (float32)
            // NetworkVarNames: m_angle (float32)
            pub mod C_Fish {
                pub const m_pos: usize = 0xF88; // 
                pub const m_vel: usize = 0xF94; // 
                pub const m_angles: usize = 0xFA0; // 
                pub const m_localLifeState: usize = 0xFAC; // 
                pub const m_deathDepth: usize = 0xFB0; // 
                pub const m_deathAngle: usize = 0xFB4; // 
                pub const m_buoyancy: usize = 0xFB8; // 
                pub const m_wiggleTimer: usize = 0xFC0; // 
                pub const m_wigglePhase: usize = 0xFD8; // 
                pub const m_wiggleRate: usize = 0xFDC; // 
                pub const m_actualPos: usize = 0xFE0; // 
                pub const m_actualAngles: usize = 0xFEC; // 
                pub const m_poolOrigin: usize = 0xFF8; // 
                pub const m_waterLevel: usize = 0x1004; // 
                pub const m_gotUpdate: usize = 0x1008; // 
                pub const m_x: usize = 0x100C; // 
                pub const m_y: usize = 0x1010; // 
                pub const m_z: usize = 0x1014; // 
                pub const m_angle: usize = 0x1018; // 
                pub const m_errorHistory: usize = 0x101C; // 
                pub const m_errorHistoryIndex: usize = 0x106C; // 
                pub const m_errorHistoryCount: usize = 0x1070; // 
                pub const m_averageError: usize = 0x1074; // 
            }
            // Parent: IntervalTimer
            // Field count: 7
            //
            // Metadata:
            // NetworkVarNames: m_flValues (float)
            // NetworkVarNames: m_nValueCounts (int)
            // NetworkVarNames: m_nBucketCount (int)
            // NetworkVarNames: m_flInterval (float)
            // NetworkVarNames: m_flFinalValue (float)
            // NetworkVarNames: m_nCompressionType (TimelineCompression_t)
            // NetworkVarNames: m_bStopped (bool)
            pub mod CTimeline {
                pub const m_flValues: usize = 0x10; // 
                pub const m_nValueCounts: usize = 0x110; // 
                pub const m_nBucketCount: usize = 0x210; // 
                pub const m_flInterval: usize = 0x214; // 
                pub const m_flFinalValue: usize = 0x218; // 
                pub const m_nCompressionType: usize = 0x21C; // 
                pub const m_bStopped: usize = 0x220; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponAWP {
            }
            // Parent: C_BaseCombatCharacter
            // Field count: 28
            //
            // Metadata:
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // MNetworkExcludeByUserGroup
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByUserGroup
            // MNetworkOverride
            // MNetworkOverride
            // MNetworkOverride
            // MNetworkOverride
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_pWeaponServices (CPlayer_WeaponServices*)
            // NetworkVarNames: m_pItemServices (CPlayer_ItemServices*)
            // NetworkVarNames: m_pAutoaimServices (CPlayer_AutoaimServices*)
            // NetworkVarNames: m_pObserverServices (CPlayer_ObserverServices*)
            // NetworkVarNames: m_pWaterServices (CPlayer_WaterServices*)
            // NetworkVarNames: m_pUseServices (CPlayer_UseServices*)
            // NetworkVarNames: m_pFlashlightServices (CPlayer_FlashlightServices*)
            // NetworkVarNames: m_pCameraServices (CPlayer_CameraServices*)
            // NetworkVarNames: m_pMovementServices (CPlayer_MovementServices*)
            // NetworkVarNames: m_ServerViewAngleChanges (ViewAngleServerChange_t)
            // NetworkVarNames: m_iHideHUD (uint32)
            // NetworkVarNames: m_skybox3d (sky3dparams_t)
            // NetworkVarNames: m_flDeathTime (GameTime_t)
            // NetworkVarNames: m_hController (CHandle<CBasePlayerController>)
            pub mod C_BasePlayerPawn {
                pub const m_pWeaponServices: usize = 0x11A8; // 
                pub const m_pItemServices: usize = 0x11B0; // 
                pub const m_pAutoaimServices: usize = 0x11B8; // 
                pub const m_pObserverServices: usize = 0x11C0; // 
                pub const m_pWaterServices: usize = 0x11C8; // 
                pub const m_pUseServices: usize = 0x11D0; // 
                pub const m_pFlashlightServices: usize = 0x11D8; // 
                pub const m_pCameraServices: usize = 0x11E0; // 
                pub const m_pMovementServices: usize = 0x11E8; // 
                pub const m_ServerViewAngleChanges: usize = 0x11F8; // 
                pub const m_nHighestConsumedServerViewAngleChangeIndex: usize = 0x1248; // 
                pub const v_angle: usize = 0x124C; // 
                pub const v_anglePrevious: usize = 0x1258; // 
                pub const m_iHideHUD: usize = 0x1264; // 
                pub const m_skybox3d: usize = 0x1268; // 
                pub const m_flDeathTime: usize = 0x12F8; // 
                pub const m_vecPredictionError: usize = 0x12FC; // 
                pub const m_flPredictionErrorTime: usize = 0x1308; // 
                pub const m_vecLastCameraSetupLocalOrigin: usize = 0x130C; // 
                pub const m_flLastCameraSetupTime: usize = 0x1318; // 
                pub const m_flFOVSensitivityAdjust: usize = 0x131C; // 
                pub const m_flMouseSensitivity: usize = 0x1320; // 
                pub const m_vOldOrigin: usize = 0x1324; // 
                pub const m_flOldSimulationTime: usize = 0x1330; // 
                pub const m_nLastExecutedCommandNumber: usize = 0x1334; // 
                pub const m_nLastExecutedCommandTick: usize = 0x1338; // 
                pub const m_hController: usize = 0x133C; // CHandle<CBasePlayerController>
                pub const m_bIsSwappingToPredictableController: usize = 0x1340; // 
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iReapplyProvisionParity (int)
            // NetworkVarNames: m_hOuter (EHANDLE)
            // NetworkVarNames: m_ProviderType (attributeprovidertypes_t)
            pub mod CAttributeManager {
                pub const m_Providers: usize = 0x8; // CUtlVector<CHandle<C_BaseEntity>>
                pub const m_iReapplyProvisionParity: usize = 0x20; // 
                pub const m_hOuter: usize = 0x24; // 
                pub const m_bPreventLoopback: usize = 0x28; // 
                pub const m_ProviderType: usize = 0x2C; // 
                pub const m_CachedResults: usize = 0x30; // 
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod CLogicalEntity {
            }
            // Parent: None
            // Field count: 24
            //
            // Metadata:
            // NetworkVarNames: m_PredNetBoolVariables (uint32)
            // NetworkVarNames: m_PredNetByteVariables (byte)
            // NetworkVarNames: m_PredNetUInt16Variables (uint16)
            // NetworkVarNames: m_PredNetIntVariables (int32)
            // NetworkVarNames: m_PredNetUInt32Variables (uint32)
            // NetworkVarNames: m_PredNetUInt64Variables (uint64)
            // NetworkVarNames: m_PredNetFloatVariables (float)
            // NetworkVarNames: m_PredNetVectorVariables (Vector)
            // NetworkVarNames: m_PredNetQuaternionVariables (Quaternion)
            // NetworkVarNames: m_PredNetGlobalSymbolVariables (CGlobalSymbol)
            // NetworkVarNames: m_OwnerOnlyPredNetBoolVariables (uint32)
            // NetworkVarNames: m_OwnerOnlyPredNetByteVariables (byte)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt16Variables (uint16)
            // NetworkVarNames: m_OwnerOnlyPredNetIntVariables (int32)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt32Variables (uint32)
            // NetworkVarNames: m_OwnerOnlyPredNetUInt64Variables (uint64)
            // NetworkVarNames: m_OwnerOnlyPredNetFloatVariables (float)
            // NetworkVarNames: m_OwnerOnlyPredNetVectorVariables (Vector)
            // NetworkVarNames: m_OwnerOnlyPredNetQuaternionVariables (Quaternion)
            // NetworkVarNames: m_OwnerOnlyPredNetGlobalSymbolVariables (CGlobalSymbol)
            // NetworkVarNames: m_nBoolVariablesCount (int)
            // NetworkVarNames: m_nOwnerOnlyBoolVariablesCount (int)
            // NetworkVarNames: m_nRandomSeedOffset (int)
            // NetworkVarNames: m_flLastTeleportTime (float)
            pub mod CAnimGraphNetworkedVariables {
                pub const m_PredNetBoolVariables: usize = 0x8; // C_NetworkUtlVectorBase<uint32>
                pub const m_PredNetByteVariables: usize = 0x20; // 
                pub const m_PredNetUInt16Variables: usize = 0x38; // C_NetworkUtlVectorBase<uint16>
                pub const m_PredNetIntVariables: usize = 0x50; // 
                pub const m_PredNetUInt32Variables: usize = 0x68; // C_NetworkUtlVectorBase<uint32>
                pub const m_PredNetUInt64Variables: usize = 0x80; // C_NetworkUtlVectorBase<uint64>
                pub const m_PredNetFloatVariables: usize = 0x98; // C_NetworkUtlVectorBase<float32>
                pub const m_PredNetVectorVariables: usize = 0xB0; // 
                pub const m_PredNetQuaternionVariables: usize = 0xC8; // C_NetworkUtlVectorBase<Quaternion>
                pub const m_PredNetGlobalSymbolVariables: usize = 0xE0; // 
                pub const m_OwnerOnlyPredNetBoolVariables: usize = 0xF8; // C_NetworkUtlVectorBase<uint32>
                pub const m_OwnerOnlyPredNetByteVariables: usize = 0x110; // 
                pub const m_OwnerOnlyPredNetUInt16Variables: usize = 0x128; // 
                pub const m_OwnerOnlyPredNetIntVariables: usize = 0x140; // 
                pub const m_OwnerOnlyPredNetUInt32Variables: usize = 0x158; // C_NetworkUtlVectorBase<uint32>
                pub const m_OwnerOnlyPredNetUInt64Variables: usize = 0x170; // C_NetworkUtlVectorBase<uint64>
                pub const m_OwnerOnlyPredNetFloatVariables: usize = 0x188; // C_NetworkUtlVectorBase<float32>
                pub const m_OwnerOnlyPredNetVectorVariables: usize = 0x1A0; // C_NetworkUtlVectorBase<Vector>
                pub const m_OwnerOnlyPredNetQuaternionVariables: usize = 0x1B8; // C_NetworkUtlVectorBase<Quaternion>
                pub const m_OwnerOnlyPredNetGlobalSymbolVariables: usize = 0x1D0; // C_NetworkUtlVectorBase<CGlobalSymbol>
                pub const m_nBoolVariablesCount: usize = 0x1E8; // 
                pub const m_nOwnerOnlyBoolVariablesCount: usize = 0x1EC; // 
                pub const m_nRandomSeedOffset: usize = 0x1F0; // 
                pub const m_flLastTeleportTime: usize = 0x1F4; // 
            }
            // Parent: C_BaseTrigger
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_flFluidDensity (float)
            pub mod C_TriggerBuoyancy {
                pub const m_BuoyancyHelper: usize = 0xD30; // 
                pub const m_flFluidDensity: usize = 0xDB0; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 41
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_flScrollSpeed (float32)
            // NetworkVarNames: m_RopeFlags (uint16)
            // NetworkVarNames: m_iRopeMaterialModelIndex (HMaterialStrong)
            // NetworkVarNames: m_nSegments (uint8)
            // NetworkVarNames: m_hStartPoint (CHandle<C_BaseEntity>)
            // NetworkVarNames: m_hEndPoint (CHandle<C_BaseEntity>)
            // NetworkVarNames: m_iStartAttachment (AttachmentHandle_t)
            // NetworkVarNames: m_iEndAttachment (AttachmentHandle_t)
            // NetworkVarNames: m_Subdiv (uint8)
            // NetworkVarNames: m_RopeLength (int16)
            // NetworkVarNames: m_Slack (int16)
            // NetworkVarNames: m_TextureScale (float32)
            // NetworkVarNames: m_fLockedPoints (uint8)
            // NetworkVarNames: m_nChangeCount (uint8)
            // NetworkVarNames: m_Width (float32)
            // NetworkVarNames: m_bConstrainBetweenEndpoints (bool)
            pub mod C_RopeKeyframe {
                pub const m_LinksTouchingSomething: usize = 0xD30; // 
                pub const m_nLinksTouchingSomething: usize = 0xD34; // 
                pub const m_bApplyWind: usize = 0xD38; // 
                pub const m_fPrevLockedPoints: usize = 0xD3C; // 
                pub const m_iForcePointMoveCounter: usize = 0xD40; // 
                pub const m_bPrevEndPointPos: usize = 0xD44; // 
                pub const m_vPrevEndPointPos: usize = 0xD48; // 
                pub const m_flCurScroll: usize = 0xD60; // 
                pub const m_flScrollSpeed: usize = 0xD64; // 
                pub const m_RopeFlags: usize = 0xD68; // 
                pub const m_iRopeMaterialModelIndex: usize = 0xD70; // 
                pub const m_LightValues: usize = 0xFE8; // 
                pub const m_nSegments: usize = 0x1060; // 
                pub const m_hStartPoint: usize = 0x1064; // 
                pub const m_hEndPoint: usize = 0x1068; // 
                pub const m_iStartAttachment: usize = 0x106C; // 
                pub const m_iEndAttachment: usize = 0x106D; // 
                pub const m_Subdiv: usize = 0x106E; // 
                pub const m_RopeLength: usize = 0x1070; // 
                pub const m_Slack: usize = 0x1072; // 
                pub const m_TextureScale: usize = 0x1074; // 
                pub const m_fLockedPoints: usize = 0x1078; // 
                pub const m_nChangeCount: usize = 0x1079; // 
                pub const m_Width: usize = 0x107C; // 
                pub const m_PhysicsDelegate: usize = 0x1080; // C_RopeKeyframe::CPhysicsDelegate
                pub const m_hMaterial: usize = 0x1090; // 
                pub const m_TextureHeight: usize = 0x1098; // 
                pub const m_vecImpulse: usize = 0x109C; // 
                pub const m_vecPreviousImpulse: usize = 0x10A8; // 
                pub const m_flCurrentGustTimer: usize = 0x10B4; // 
                pub const m_flCurrentGustLifetime: usize = 0x10B8; // 
                pub const m_flTimeToNextGust: usize = 0x10BC; // 
                pub const m_vWindDir: usize = 0x10C0; // 
                pub const m_vColorMod: usize = 0x10CC; // 
                pub const m_vCachedEndPointAttachmentPos: usize = 0x10D8; // 
                pub const m_vCachedEndPointAttachmentAngle: usize = 0x10F0; // 
                pub const m_bConstrainBetweenEndpoints: usize = 0x1108; // 
                pub const m_bEndPointAttachmentPositionsDirty: usize = 0x0; // 
                pub const m_bEndPointAttachmentAnglesDirty: usize = 0x0; // 
                pub const m_bNewDataThisFrame: usize = 0x0; // 
                pub const m_bPhysicsInitted: usize = 0x0; // 
            }
            // Parent: C_BaseEntity
            // Field count: 16
            //
            // Metadata:
            // NetworkVarNames: m_hGradientFogTexture (HRenderTextureStrong)
            // NetworkVarNames: m_flFogStartDistance (float)
            // NetworkVarNames: m_flFogEndDistance (float)
            // NetworkVarNames: m_bHeightFogEnabled (bool)
            // NetworkVarNames: m_flFogStartHeight (float)
            // NetworkVarNames: m_flFogEndHeight (float)
            // NetworkVarNames: m_flFarZ (float)
            // NetworkVarNames: m_flFogMaxOpacity (float)
            // NetworkVarNames: m_flFogFalloffExponent (float)
            // NetworkVarNames: m_flFogVerticalExponent (float)
            // NetworkVarNames: m_fogColor (Color)
            // NetworkVarNames: m_flFogStrength (float)
            // NetworkVarNames: m_flFadeTime (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bIsEnabled (bool)
            pub mod C_GradientFog {
                pub const m_hGradientFogTexture: usize = 0x568; // 
                pub const m_flFogStartDistance: usize = 0x570; // 
                pub const m_flFogEndDistance: usize = 0x574; // 
                pub const m_bHeightFogEnabled: usize = 0x578; // 
                pub const m_flFogStartHeight: usize = 0x57C; // 
                pub const m_flFogEndHeight: usize = 0x580; // 
                pub const m_flFarZ: usize = 0x584; // 
                pub const m_flFogMaxOpacity: usize = 0x588; // 
                pub const m_flFogFalloffExponent: usize = 0x58C; // 
                pub const m_flFogVerticalExponent: usize = 0x590; // 
                pub const m_fogColor: usize = 0x594; // 
                pub const m_flFogStrength: usize = 0x598; // 
                pub const m_flFadeTime: usize = 0x59C; // 
                pub const m_bStartDisabled: usize = 0x5A0; // 
                pub const m_bIsEnabled: usize = 0x5A1; // 
                pub const m_bGradientFogNeedsTextures: usize = 0x5A2; // 
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_bSpotted (bool)
            // NetworkVarNames: m_bSpottedByMask (uint32)
            pub mod EntitySpottedState_t {
                pub const m_bSpotted: usize = 0x8; // 
                pub const m_bSpottedByMask: usize = 0xC; // 
            }
            // Parent: C_CSGO_MapPreviewCameraPath
            // Field count: 7
            pub mod C_CSGO_TeamPreviewCamera {
                pub const m_nVariant: usize = 0x5D0; // 
                pub const m_bDofEnabled: usize = 0x5D4; // 
                pub const m_flDofNearBlurry: usize = 0x5D8; // 
                pub const m_flDofNearCrisp: usize = 0x5DC; // 
                pub const m_flDofFarCrisp: usize = 0x5E0; // 
                pub const m_flDofFarBlurry: usize = 0x5E4; // 
                pub const m_flDofTiltToGround: usize = 0x5E8; // 
            }
            // Parent: CCSPlayerBase_CameraServices
            // Field count: 2
            pub mod CCSPlayer_CameraServices {
                pub const m_flDeathCamTilt: usize = 0x228; // 
                pub const m_vClientScopeInaccuracy: usize = 0x230; // 
            }
            // Parent: C_CSGO_TeamSelectCharacterPosition
            // Field count: 0
            pub mod C_CSGO_TeamSelectCounterTerroristPosition {
            }
            // Parent: C_EconEntity
            // Field count: 2
            pub mod C_EconWearable {
                pub const m_nForceSkin: usize = 0x1668; // 
                pub const m_bAlwaysAllow: usize = 0x166C; // 
            }
            // Parent: C_CSGO_TeamIntroCharacterPosition
            // Field count: 0
            pub mod CCSGO_WingmanIntroCharacterPosition {
            }
            // Parent: CSPerRoundStats_t
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iEnemy5Ks (int)
            // NetworkVarNames: m_iEnemy4Ks (int)
            // NetworkVarNames: m_iEnemy3Ks (int)
            // NetworkVarNames: m_iEnemyKnifeKills (int)
            // NetworkVarNames: m_iEnemyTaserKills (int)
            pub mod CSMatchStats_t {
                pub const m_iEnemy5Ks: usize = 0x68; // 
                pub const m_iEnemy4Ks: usize = 0x6C; // 
                pub const m_iEnemy3Ks: usize = 0x70; // 
                pub const m_iEnemyKnifeKills: usize = 0x74; // 
                pub const m_iEnemyTaserKills: usize = 0x78; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod C_BaseToggle {
            }
            // Parent: C_BaseEntity
            // Field count: 16
            //
            // Metadata:
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_flStrength (float)
            // NetworkVarNames: m_nFalloffShape (int)
            // NetworkVarNames: m_flFalloffExponent (float)
            // NetworkVarNames: m_flHeightFogDepth (float)
            // NetworkVarNames: m_fHeightFogEdgeWidth (float)
            // NetworkVarNames: m_fIndirectLightStrength (float)
            // NetworkVarNames: m_fSunLightStrength (float)
            // NetworkVarNames: m_fNoiseStrength (float)
            // NetworkVarNames: m_bOverrideIndirectLightStrength (bool)
            // NetworkVarNames: m_bOverrideSunLightStrength (bool)
            // NetworkVarNames: m_bOverrideNoiseStrength (bool)
            // NetworkVarNames: m_bAllowLPVIndirect (bool)
            pub mod C_EnvVolumetricFogVolume {
                pub const m_bActive: usize = 0x568; // 
                pub const m_vBoxMins: usize = 0x56C; // 
                pub const m_vBoxMaxs: usize = 0x578; // 
                pub const m_bStartDisabled: usize = 0x584; // 
                pub const m_flStrength: usize = 0x588; // 
                pub const m_nFalloffShape: usize = 0x58C; // 
                pub const m_flFalloffExponent: usize = 0x590; // 
                pub const m_flHeightFogDepth: usize = 0x594; // 
                pub const m_fHeightFogEdgeWidth: usize = 0x598; // 
                pub const m_fIndirectLightStrength: usize = 0x59C; // 
                pub const m_fSunLightStrength: usize = 0x5A0; // 
                pub const m_fNoiseStrength: usize = 0x5A4; // 
                pub const m_bOverrideIndirectLightStrength: usize = 0x5A8; // 
                pub const m_bOverrideSunLightStrength: usize = 0x5A9; // 
                pub const m_bOverrideNoiseStrength: usize = 0x5AA; // 
                pub const m_bAllowLPVIndirect: usize = 0x5AB; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod CWaterSplasher {
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod C_ModelPointEntity {
            }
            // Parent: CPlayer_ViewModelServices
            // Field count: 0
            pub mod CCSObserver_ViewModelServices {
            }
            // Parent: C_BaseFire
            // Field count: 13
            //
            // Metadata:
            // MNetworkOverride
            // MNetworkOverride
            // NetworkVarNames: m_nFlameModelIndex (int32)
            // NetworkVarNames: m_nFlameFromAboveModelIndex (int32)
            pub mod C_FireSmoke {
                pub const m_nFlameModelIndex: usize = 0x578; // 
                pub const m_nFlameFromAboveModelIndex: usize = 0x57C; // 
                pub const m_flScaleRegister: usize = 0x580; // 
                pub const m_flScaleStart: usize = 0x584; // 
                pub const m_flScaleEnd: usize = 0x588; // 
                pub const m_flScaleTimeStart: usize = 0x58C; // 
                pub const m_flScaleTimeEnd: usize = 0x590; // 
                pub const m_flChildFlameSpread: usize = 0x594; // 
                pub const m_flClipPerc: usize = 0x5A8; // 
                pub const m_bClipTested: usize = 0x5AC; // 
                pub const m_bFadingOut: usize = 0x5AD; // 
                pub const m_tParticleSpawn: usize = 0x5B0; // 
                pub const m_pFireOverlay: usize = 0x5B8; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponRevolver {
            }
            // Parent: C_Sprite
            // Field count: 0
            pub mod C_FireFromAboveSprite {
            }
            // Parent: C_BaseEntity
            // Field count: 24
            //
            // Metadata:
            // NetworkVarNames: m_Entity_Color (Color)
            // NetworkVarNames: m_Entity_flBrightness (float)
            // NetworkVarNames: m_Entity_hCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_bCustomCubemapTexture (bool)
            // NetworkVarNames: m_Entity_hLightProbeTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_hLightProbeDirectLightIndicesTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_hLightProbeDirectLightScalarsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_hLightProbeDirectLightShadowsTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_vBoxMins (Vector)
            // NetworkVarNames: m_Entity_vBoxMaxs (Vector)
            // NetworkVarNames: m_Entity_bMoveable (bool)
            // NetworkVarNames: m_Entity_nHandshake (int)
            // NetworkVarNames: m_Entity_nEnvCubeMapArrayIndex (int)
            // NetworkVarNames: m_Entity_nPriority (int)
            // NetworkVarNames: m_Entity_bStartDisabled (bool)
            // NetworkVarNames: m_Entity_flEdgeFadeDist (float)
            // NetworkVarNames: m_Entity_vEdgeFadeDists (Vector)
            // NetworkVarNames: m_Entity_nLightProbeSizeX (int)
            // NetworkVarNames: m_Entity_nLightProbeSizeY (int)
            // NetworkVarNames: m_Entity_nLightProbeSizeZ (int)
            // NetworkVarNames: m_Entity_nLightProbeAtlasX (int)
            // NetworkVarNames: m_Entity_nLightProbeAtlasY (int)
            // NetworkVarNames: m_Entity_nLightProbeAtlasZ (int)
            // NetworkVarNames: m_Entity_bEnabled (bool)
            pub mod C_EnvCombinedLightProbeVolume {
                pub const m_Entity_Color: usize = 0x15C8; // 
                pub const m_Entity_flBrightness: usize = 0x15CC; // 
                pub const m_Entity_hCubemapTexture: usize = 0x15D0; // 
                pub const m_Entity_bCustomCubemapTexture: usize = 0x15D8; // 
                pub const m_Entity_hLightProbeTexture: usize = 0x15E0; // 
                pub const m_Entity_hLightProbeDirectLightIndicesTexture: usize = 0x15E8; // 
                pub const m_Entity_hLightProbeDirectLightScalarsTexture: usize = 0x15F0; // 
                pub const m_Entity_hLightProbeDirectLightShadowsTexture: usize = 0x15F8; // 
                pub const m_Entity_vBoxMins: usize = 0x1600; // 
                pub const m_Entity_vBoxMaxs: usize = 0x160C; // 
                pub const m_Entity_bMoveable: usize = 0x1618; // 
                pub const m_Entity_nHandshake: usize = 0x161C; // 
                pub const m_Entity_nEnvCubeMapArrayIndex: usize = 0x1620; // 
                pub const m_Entity_nPriority: usize = 0x1624; // 
                pub const m_Entity_bStartDisabled: usize = 0x1628; // 
                pub const m_Entity_flEdgeFadeDist: usize = 0x162C; // 
                pub const m_Entity_vEdgeFadeDists: usize = 0x1630; // 
                pub const m_Entity_nLightProbeSizeX: usize = 0x163C; // 
                pub const m_Entity_nLightProbeSizeY: usize = 0x1640; // 
                pub const m_Entity_nLightProbeSizeZ: usize = 0x1644; // 
                pub const m_Entity_nLightProbeAtlasX: usize = 0x1648; // 
                pub const m_Entity_nLightProbeAtlasY: usize = 0x164C; // 
                pub const m_Entity_nLightProbeAtlasZ: usize = 0x1650; // 
                pub const m_Entity_bEnabled: usize = 0x1669; // 
            }
            // Parent: C_SoundOpvarSetPointBase
            // Field count: 0
            pub mod C_SoundOpvarSetOBBWindEntity {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_Handle (ModelConfigHandle_t)
            // NetworkVarNames: m_Name (string_t)
            // NetworkVarNames: m_AssociatedEntities (CHandle<C_BaseModelEntity>)
            // NetworkVarNames: m_AssociatedEntityNames (string_t)
            pub mod ActiveModelConfig_t {
                pub const m_Handle: usize = 0x28; // 
                pub const m_Name: usize = 0x30; // 
                pub const m_AssociatedEntities: usize = 0x38; // 
                pub const m_AssociatedEntityNames: usize = 0x50; // C_NetworkUtlVectorBase<CUtlSymbolLarge>
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponMP5SD {
            }
            // Parent: C_DynamicProp
            // Field count: 0
            pub mod C_DynamicPropAlias_dynamic_prop {
            }
            // Parent: CPlayerPawnComponent
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_totalHitsOnServer (int32)
            pub mod CCSPlayer_BulletServices {
                pub const m_totalHitsOnServer: usize = 0x40; // 
            }
            // Parent: CLogicalEntity
            // Field count: 7
            pub mod CLogicRelay {
                pub const m_OnTrigger: usize = 0x568; // 
                pub const m_OnSpawn: usize = 0x590; // 
                pub const m_bDisabled: usize = 0x5B8; // 
                pub const m_bWaitForRefire: usize = 0x5B9; // 
                pub const m_bTriggerOnce: usize = 0x5BA; // 
                pub const m_bFastRetrigger: usize = 0x5BB; // 
                pub const m_bPassthoughCaller: usize = 0x5BC; // 
            }
            // Parent: C_PointCamera
            // Field count: 1
            pub mod C_PointCameraVFOV {
                pub const m_flVerticalFOV: usize = 0x5C8; // 
            }
            // Parent: CPlayer_ItemServices
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_bHasDefuser (bool)
            // NetworkVarNames: m_bHasHelmet (bool)
            // NetworkVarNames: m_bHasHeavyArmor (bool)
            pub mod CCSPlayer_ItemServices {
                pub const m_bHasDefuser: usize = 0x40; // 
                pub const m_bHasHelmet: usize = 0x41; // 
                pub const m_bHasHeavyArmor: usize = 0x42; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 73
            //
            // Metadata:
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_nColorMode (int)
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_flColorTemperature (float)
            // NetworkVarNames: m_flBrightness (float)
            // NetworkVarNames: m_flBrightnessScale (float)
            // NetworkVarNames: m_nDirectLight (int)
            // NetworkVarNames: m_nBakedShadowIndex (int)
            // NetworkVarNames: m_nLuminaireShape (int)
            // NetworkVarNames: m_flLuminaireSize (float)
            // NetworkVarNames: m_flLuminaireAnisotropy (float)
            // NetworkVarNames: m_LightStyleString (CUtlString)
            // NetworkVarNames: m_flLightStyleStartTime (GameTime_t)
            // NetworkVarNames: m_QueuedLightStyleStrings (CUtlString)
            // NetworkVarNames: m_LightStyleEvents (CUtlString)
            // NetworkVarNames: m_LightStyleTargets (CHandle<C_BaseModelEntity>)
            // NetworkVarNames: m_hLightCookie (HRenderTextureStrong)
            // NetworkVarNames: m_flShape (float)
            // NetworkVarNames: m_flSoftX (float)
            // NetworkVarNames: m_flSoftY (float)
            // NetworkVarNames: m_flSkirt (float)
            // NetworkVarNames: m_flSkirtNear (float)
            // NetworkVarNames: m_vSizeParams (Vector)
            // NetworkVarNames: m_flRange (float)
            // NetworkVarNames: m_vShear (Vector)
            // NetworkVarNames: m_nBakeSpecularToCubemaps (int)
            // NetworkVarNames: m_vBakeSpecularToCubemapsSize (Vector)
            // NetworkVarNames: m_nCastShadows (int)
            // NetworkVarNames: m_nShadowMapSize (int)
            // NetworkVarNames: m_nShadowPriority (int)
            // NetworkVarNames: m_bContactShadow (bool)
            // NetworkVarNames: m_nBounceLight (int)
            // NetworkVarNames: m_flBounceScale (float)
            // NetworkVarNames: m_flMinRoughness (float)
            // NetworkVarNames: m_vAlternateColor (Vector)
            // NetworkVarNames: m_fAlternateColorBrightness (float)
            // NetworkVarNames: m_nFog (int)
            // NetworkVarNames: m_flFogStrength (float)
            // NetworkVarNames: m_nFogShadows (int)
            // NetworkVarNames: m_flFogScale (float)
            // NetworkVarNames: m_bFogMixedShadows (bool)
            // NetworkVarNames: m_flFadeSizeStart (float)
            // NetworkVarNames: m_flFadeSizeEnd (float)
            // NetworkVarNames: m_flShadowFadeSizeStart (float)
            // NetworkVarNames: m_flShadowFadeSizeEnd (float)
            // NetworkVarNames: m_bPrecomputedFieldsValid (bool)
            // NetworkVarNames: m_vPrecomputedBoundsMins (Vector)
            // NetworkVarNames: m_vPrecomputedBoundsMaxs (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent (Vector)
            // NetworkVarNames: m_nPrecomputedSubFrusta (int)
            // NetworkVarNames: m_vPrecomputedOBBOrigin0 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles0 (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent0 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin1 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles1 (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent1 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin2 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles2 (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent2 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin3 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles3 (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent3 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin4 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles4 (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent4 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin5 (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles5 (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent5 (Vector)
            // NetworkVarNames: m_VisClusters (uint16)
            pub mod C_BarnLight {
                pub const m_bEnabled: usize = 0xD28; // 
                pub const m_nColorMode: usize = 0xD2C; // 
                pub const m_Color: usize = 0xD30; // 
                pub const m_flColorTemperature: usize = 0xD34; // 
                pub const m_flBrightness: usize = 0xD38; // 
                pub const m_flBrightnessScale: usize = 0xD3C; // 
                pub const m_nDirectLight: usize = 0xD40; // 
                pub const m_nBakedShadowIndex: usize = 0xD44; // 
                pub const m_nLuminaireShape: usize = 0xD48; // 
                pub const m_flLuminaireSize: usize = 0xD4C; // 
                pub const m_flLuminaireAnisotropy: usize = 0xD50; // 
                pub const m_LightStyleString: usize = 0xD58; // 
                pub const m_flLightStyleStartTime: usize = 0xD60; // 
                pub const m_QueuedLightStyleStrings: usize = 0xD68; // C_NetworkUtlVectorBase<CUtlString>
                pub const m_LightStyleEvents: usize = 0xD80; // C_NetworkUtlVectorBase<CUtlString>
                pub const m_LightStyleTargets: usize = 0xD98; // 
                pub const m_StyleEvent: usize = 0xDB0; // 
                pub const m_hLightCookie: usize = 0xE50; // 
                pub const m_flShape: usize = 0xE58; // 
                pub const m_flSoftX: usize = 0xE5C; // 
                pub const m_flSoftY: usize = 0xE60; // 
                pub const m_flSkirt: usize = 0xE64; // 
                pub const m_flSkirtNear: usize = 0xE68; // 
                pub const m_vSizeParams: usize = 0xE6C; // 
                pub const m_flRange: usize = 0xE78; // 
                pub const m_vShear: usize = 0xE7C; // 
                pub const m_nBakeSpecularToCubemaps: usize = 0xE88; // 
                pub const m_vBakeSpecularToCubemapsSize: usize = 0xE8C; // 
                pub const m_nCastShadows: usize = 0xE98; // 
                pub const m_nShadowMapSize: usize = 0xE9C; // 
                pub const m_nShadowPriority: usize = 0xEA0; // 
                pub const m_bContactShadow: usize = 0xEA4; // 
                pub const m_nBounceLight: usize = 0xEA8; // 
                pub const m_flBounceScale: usize = 0xEAC; // 
                pub const m_flMinRoughness: usize = 0xEB0; // 
                pub const m_vAlternateColor: usize = 0xEB4; // 
                pub const m_fAlternateColorBrightness: usize = 0xEC0; // 
                pub const m_nFog: usize = 0xEC4; // 
                pub const m_flFogStrength: usize = 0xEC8; // 
                pub const m_nFogShadows: usize = 0xECC; // 
                pub const m_flFogScale: usize = 0xED0; // 
                pub const m_bFogMixedShadows: usize = 0xED4; // 
                pub const m_flFadeSizeStart: usize = 0xED8; // 
                pub const m_flFadeSizeEnd: usize = 0xEDC; // 
                pub const m_flShadowFadeSizeStart: usize = 0xEE0; // 
                pub const m_flShadowFadeSizeEnd: usize = 0xEE4; // 
                pub const m_bPrecomputedFieldsValid: usize = 0xEE8; // 
                pub const m_vPrecomputedBoundsMins: usize = 0xEEC; // 
                pub const m_vPrecomputedBoundsMaxs: usize = 0xEF8; // 
                pub const m_vPrecomputedOBBOrigin: usize = 0xF04; // 
                pub const m_vPrecomputedOBBAngles: usize = 0xF10; // 
                pub const m_vPrecomputedOBBExtent: usize = 0xF1C; // 
                pub const m_nPrecomputedSubFrusta: usize = 0xF28; // 
                pub const m_vPrecomputedOBBOrigin0: usize = 0xF2C; // 
                pub const m_vPrecomputedOBBAngles0: usize = 0xF38; // 
                pub const m_vPrecomputedOBBExtent0: usize = 0xF44; // 
                pub const m_vPrecomputedOBBOrigin1: usize = 0xF50; // 
                pub const m_vPrecomputedOBBAngles1: usize = 0xF5C; // 
                pub const m_vPrecomputedOBBExtent1: usize = 0xF68; // 
                pub const m_vPrecomputedOBBOrigin2: usize = 0xF74; // 
                pub const m_vPrecomputedOBBAngles2: usize = 0xF80; // 
                pub const m_vPrecomputedOBBExtent2: usize = 0xF8C; // 
                pub const m_vPrecomputedOBBOrigin3: usize = 0xF98; // 
                pub const m_vPrecomputedOBBAngles3: usize = 0xFA4; // 
                pub const m_vPrecomputedOBBExtent3: usize = 0xFB0; // 
                pub const m_vPrecomputedOBBOrigin4: usize = 0xFBC; // 
                pub const m_vPrecomputedOBBAngles4: usize = 0xFC8; // 
                pub const m_vPrecomputedOBBExtent4: usize = 0xFD4; // 
                pub const m_vPrecomputedOBBOrigin5: usize = 0xFE0; // 
                pub const m_vPrecomputedOBBAngles5: usize = 0xFEC; // 
                pub const m_vPrecomputedOBBExtent5: usize = 0xFF8; // 
                pub const m_bInitialBoneSetup: usize = 0x1048; // 
                pub const m_VisClusters: usize = 0x1050; // C_NetworkUtlVectorBase<uint16>
            }
            // Parent: C_BaseEntity
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_flAutoExposureMin (float)
            // NetworkVarNames: m_flAutoExposureMax (float)
            // NetworkVarNames: m_flTonemapPercentTarget (float)
            // NetworkVarNames: m_flTonemapPercentBrightPixels (float)
            // NetworkVarNames: m_flTonemapMinAvgLum (float)
            // NetworkVarNames: m_flExposureAdaptationSpeedUp (float)
            // NetworkVarNames: m_flExposureAdaptationSpeedDown (float)
            // NetworkVarNames: m_flTonemapEVSmoothingRange (float)
            pub mod C_TonemapController2 {
                pub const m_flAutoExposureMin: usize = 0x568; // 
                pub const m_flAutoExposureMax: usize = 0x56C; // 
                pub const m_flTonemapPercentTarget: usize = 0x570; // 
                pub const m_flTonemapPercentBrightPixels: usize = 0x574; // 
                pub const m_flTonemapMinAvgLum: usize = 0x578; // 
                pub const m_flExposureAdaptationSpeedUp: usize = 0x57C; // 
                pub const m_flExposureAdaptationSpeedDown: usize = 0x580; // 
                pub const m_flTonemapEVSmoothingRange: usize = 0x584; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponG3SG1 {
            }
            // Parent: C_BaseModelEntity
            // Field count: 1
            pub mod CFuncWater {
                pub const m_BuoyancyHelper: usize = 0xD28; // 
            }
            // Parent: C_SoundOpvarSetPointEntity
            // Field count: 0
            pub mod C_SoundOpvarSetAutoRoomEntity {
            }
            // Parent: C_BaseFlex
            // Field count: 20
            //
            // Metadata:
            // NetworkVarNames: m_AttributeManager (CAttributeContainer)
            // NetworkVarNames: m_OriginalOwnerXuidLow (uint32)
            // NetworkVarNames: m_OriginalOwnerXuidHigh (uint32)
            // NetworkVarNames: m_nFallbackPaintKit (int)
            // NetworkVarNames: m_nFallbackSeed (int)
            // NetworkVarNames: m_flFallbackWear (float)
            // NetworkVarNames: m_nFallbackStatTrak (int)
            pub mod C_EconEntity {
                pub const m_flFlexDelayTime: usize = 0x1130; // 
                pub const m_flFlexDelayedWeight: usize = 0x1138; // 
                pub const m_bAttributesInitialized: usize = 0x1140; // 
                pub const m_AttributeManager: usize = 0x1148; // 
                pub const m_OriginalOwnerXuidLow: usize = 0x15F0; // 
                pub const m_OriginalOwnerXuidHigh: usize = 0x15F4; // 
                pub const m_nFallbackPaintKit: usize = 0x15F8; // 
                pub const m_nFallbackSeed: usize = 0x15FC; // 
                pub const m_flFallbackWear: usize = 0x1600; // 
                pub const m_nFallbackStatTrak: usize = 0x1604; // 
                pub const m_bClientside: usize = 0x1608; // 
                pub const m_bParticleSystemsCreated: usize = 0x1609; // 
                pub const m_vecAttachedParticles: usize = 0x1610; // 
                pub const m_hViewmodelAttachment: usize = 0x1628; // 
                pub const m_iOldTeam: usize = 0x162C; // 
                pub const m_bAttachmentDirty: usize = 0x1630; // 
                pub const m_nUnloadedModelIndex: usize = 0x1634; // 
                pub const m_iNumOwnerValidationRetries: usize = 0x1638; // 
                pub const m_hOldProvidee: usize = 0x1648; // 
                pub const m_vecAttachedModels: usize = 0x1650; // CUtlVector<C_EconEntity::AttachedModelData_t>
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponSG556 {
            }
            // Parent: C_BarnLight
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bShowLight (bool)
            pub mod C_RectLight {
                pub const m_bShowLight: usize = 0x1070; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 24
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_szSnapshotFileName (char)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bFrozen (bool)
            // NetworkVarNames: m_flFreezeTransitionDuration (float)
            // NetworkVarNames: m_nStopType (int)
            // NetworkVarNames: m_bAnimateDuringGameplayPause (bool)
            // NetworkVarNames: m_iEffectIndex (HParticleSystemDefinitionStrong)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_flPreSimTime (float32)
            // NetworkVarNames: m_vServerControlPoints (Vector)
            // NetworkVarNames: m_iServerControlPointAssignments (uint8)
            // NetworkVarNames: m_hControlPointEnts (CHandle<CBaseEntity>)
            // NetworkVarNames: m_bNoSave (bool)
            // NetworkVarNames: m_bNoFreeze (bool)
            // NetworkVarNames: m_bNoRamp (bool)
            pub mod C_ParticleSystem {
                pub const m_szSnapshotFileName: usize = 0xD28; // 
                pub const m_bActive: usize = 0xF28; // 
                pub const m_bFrozen: usize = 0xF29; // 
                pub const m_flFreezeTransitionDuration: usize = 0xF2C; // 
                pub const m_nStopType: usize = 0xF30; // 
                pub const m_bAnimateDuringGameplayPause: usize = 0xF34; // 
                pub const m_iEffectIndex: usize = 0xF38; // 
                pub const m_flStartTime: usize = 0xF40; // 
                pub const m_flPreSimTime: usize = 0xF44; // 
                pub const m_vServerControlPoints: usize = 0xF48; // 
                pub const m_iServerControlPointAssignments: usize = 0xF78; // 
                pub const m_hControlPointEnts: usize = 0xF7C; // 
                pub const m_bNoSave: usize = 0x107C; // 
                pub const m_bNoFreeze: usize = 0x107D; // 
                pub const m_bNoRamp: usize = 0x107E; // 
                pub const m_bStartActive: usize = 0x107F; // 
                pub const m_iszEffectName: usize = 0x1080; // 
                pub const m_iszControlPointNames: usize = 0x1088; // 
                pub const m_nDataCP: usize = 0x1288; // 
                pub const m_vecDataCPValue: usize = 0x128C; // 
                pub const m_nTintCP: usize = 0x1298; // 
                pub const m_clrTint: usize = 0x129C; // 
                pub const m_bOldActive: usize = 0x12C0; // 
                pub const m_bOldFrozen: usize = 0x12C1; // 
            }
            // Parent: C_CSGO_PreviewPlayer
            // Field count: 0
            pub mod C_CSGO_TeamPreviewModel {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponGlock {
            }
            // Parent: C_GameRulesProxy
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_pGameRules (C_CSGameRules*)
            pub mod C_CSGameRulesProxy {
                pub const m_pGameRules: usize = 0x568; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 6
            //
            // Metadata:
            // NetworkVarNames: m_iObserverMode (uint8)
            // NetworkVarNames: m_hObserverTarget (CHandle<CBaseEntity>)
            pub mod CPlayer_ObserverServices {
                pub const m_iObserverMode: usize = 0x40; // 
                pub const m_hObserverTarget: usize = 0x44; // 
                pub const m_iObserverLastMode: usize = 0x48; // 
                pub const m_bForcedObserverMode: usize = 0x4C; // 
                pub const m_flObserverChaseDistance: usize = 0x50; // 
                pub const m_flObserverChaseDistanceCalcTime: usize = 0x54; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod CWeaponZoneRepulsor {
            }
            // Parent: CPlayerPawnComponent
            // Field count: 0
            pub mod CPlayer_WaterServices {
            }
            // Parent: C_CSGO_EndOfMatchLineupEndpoint
            // Field count: 0
            pub mod C_CSGO_EndOfMatchLineupEnd {
            }
            // Parent: None
            // Field count: 6
            pub mod SequenceHistory_t {
                pub const m_hSequence: usize = 0x0; // 
                pub const m_flSeqStartTime: usize = 0x4; // 
                pub const m_flSeqFixedCycle: usize = 0x8; // 
                pub const m_nSeqLoopMode: usize = 0xC; // 
                pub const m_flPlaybackRate: usize = 0x10; // 
                pub const m_flCyclesPerSecond: usize = 0x14; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 1
            pub mod C_Multimeter {
                pub const m_hTargetC4: usize = 0xF90; // 
            }
            // Parent: C_BaseEntity
            // Field count: 2
            pub mod C_CsmFovOverride {
                pub const m_cameraName: usize = 0x568; // 
                pub const m_flCsmFovOverrideValue: usize = 0x570; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 9
            //
            // Metadata:
            // NetworkVarNames: m_hDecalMaterial (HMaterialStrong)
            // NetworkVarNames: m_flWidth (float)
            // NetworkVarNames: m_flHeight (float)
            // NetworkVarNames: m_flDepth (float)
            // NetworkVarNames: m_nRenderOrder (uint32)
            // NetworkVarNames: m_bProjectOnWorld (bool)
            // NetworkVarNames: m_bProjectOnCharacters (bool)
            // NetworkVarNames: m_bProjectOnWater (bool)
            // NetworkVarNames: m_flDepthSortBias (float)
            pub mod C_EnvDecal {
                pub const m_hDecalMaterial: usize = 0xD28; // 
                pub const m_flWidth: usize = 0xD30; // 
                pub const m_flHeight: usize = 0xD34; // 
                pub const m_flDepth: usize = 0xD38; // 
                pub const m_nRenderOrder: usize = 0xD3C; // 
                pub const m_bProjectOnWorld: usize = 0xD40; // 
                pub const m_bProjectOnCharacters: usize = 0xD41; // 
                pub const m_bProjectOnWater: usize = 0xD42; // 
                pub const m_flDepthSortBias: usize = 0xD44; // 
            }
            // Parent: CCSGameModeRules
            // Field count: 0
            pub mod CCSGameModeRules_Noop {
            }
            // Parent: C_BaseEntity
            // Field count: 18
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // NetworkVarNames: m_MinFalloff (float32)
            // NetworkVarNames: m_MaxFalloff (float32)
            // NetworkVarNames: m_flFadeInDuration (float32)
            // NetworkVarNames: m_flFadeOutDuration (float32)
            // NetworkVarNames: m_flMaxWeight (float32)
            // NetworkVarNames: m_flCurWeight (float32)
            // NetworkVarNames: m_netlookupFilename (char)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bMaster (bool)
            // NetworkVarNames: m_bClientSide (bool)
            // NetworkVarNames: m_bExclusive (bool)
            pub mod C_ColorCorrection {
                pub const m_vecOrigin: usize = 0x568; // 
                pub const m_MinFalloff: usize = 0x574; // 
                pub const m_MaxFalloff: usize = 0x578; // 
                pub const m_flFadeInDuration: usize = 0x57C; // 
                pub const m_flFadeOutDuration: usize = 0x580; // 
                pub const m_flMaxWeight: usize = 0x584; // 
                pub const m_flCurWeight: usize = 0x588; // 
                pub const m_netlookupFilename: usize = 0x58C; // 
                pub const m_bEnabled: usize = 0x78C; // 
                pub const m_bMaster: usize = 0x78D; // 
                pub const m_bClientSide: usize = 0x78E; // 
                pub const m_bExclusive: usize = 0x78F; // 
                pub const m_bEnabledOnClient: usize = 0x790; // 
                pub const m_flCurWeightOnClient: usize = 0x794; // 
                pub const m_bFadingIn: usize = 0x798; // 
                pub const m_flFadeStartWeight: usize = 0x79C; // 
                pub const m_flFadeStartTime: usize = 0x7A0; // 
                pub const m_flFadeDuration: usize = 0x7A4; // 
            }
            // Parent: C_BaseTrigger
            // Field count: 0
            pub mod CHostageRescueZoneShim {
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // NetworkVarNames: m_hModel (HModelStrong)
            // NetworkVarNames: m_bClientClothCreationSuppressed (bool)
            // NetworkVarNames: m_MeshGroupMask (MeshGroupMask_t)
            // NetworkVarNames: m_nIdealMotionType (int8)
            pub mod CModelState {
                pub const m_hModel: usize = 0xA0; // 
                pub const m_ModelName: usize = 0xA8; // 
                pub const m_bClientClothCreationSuppressed: usize = 0xE8; // 
                pub const m_MeshGroupMask: usize = 0x198; // 
                pub const m_nIdealMotionType: usize = 0x21A; // 
                pub const m_nForceLOD: usize = 0x21B; // 
                pub const m_nClothUpdateFlags: usize = 0x21C; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponP250 {
            }
            // Parent: None
            // Field count: 1
            pub mod C_EconEntity__AttachedModelData_t {
                pub const m_iModelDisplayFlags: usize = 0x0; // 
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_Attributes (CEconItemAttribute)
            pub mod CAttributeList {
                pub const m_Attributes: usize = 0x8; // 
                pub const m_pManager: usize = 0x58; // 
            }
            // Parent: C_MultiplayRules
            // Field count: 0
            pub mod C_TeamplayRules {
            }
            // Parent: C_CSGO_TeamPreviewCamera
            // Field count: 0
            pub mod C_CSGO_CounterTerroristWingmanIntroCamera {
            }
            // Parent: C_CSWeaponBase
            // Field count: 0
            pub mod C_WeaponNOVA {
            }
            // Parent: C_BaseCSGrenadeProjectile
            // Field count: 0
            pub mod C_HEGrenadeProjectile {
            }
            // Parent: C_BaseToggle
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bIsUsable (bool)
            pub mod C_BaseDoor {
                pub const m_bIsUsable: usize = 0xD28; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 2
            pub mod CGrenadeTracer {
                pub const m_flTracerDuration: usize = 0xD40; // 
                pub const m_nType: usize = 0xD44; // 
            }
            // Parent: C_BaseTrigger
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bBombPlantedHere (bool)
            pub mod CBombTarget {
                pub const m_bBombPlantedHere: usize = 0xD30; // 
            }
            // Parent: C_BaseClientUIEntity
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hActivator (EHANDLE)
            pub mod C_PointClientUIDialog {
                pub const m_hActivator: usize = 0xD58; // 
                pub const m_bStartEnabled: usize = 0xD5C; // 
            }
            // Parent: CCSGameModeRules
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_WeaponSequence (CUtlString)
            pub mod CCSGameModeRules_ArmsRace {
                pub const m_WeaponSequence: usize = 0x30; // 
            }
            // Parent: C_PointEntity
            // Field count: 1
            pub mod CPointChildModifier {
                pub const m_bOrphanInsteadOfDeletingChildrenOnRemove: usize = 0x568; // 
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // NetworkVarNames: m_PlayerDamager (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_PlayerRecipient (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_hPlayerControllerDamager (CHandle<CCSPlayerController>)
            // NetworkVarNames: m_hPlayerControllerRecipient (CHandle<CCSPlayerController>)
            // NetworkVarNames: m_szPlayerDamagerName (CUtlString)
            // NetworkVarNames: m_szPlayerRecipientName (CUtlString)
            // NetworkVarNames: m_DamagerXuid (uint64)
            // NetworkVarNames: m_RecipientXuid (uint64)
            // NetworkVarNames: m_iDamage (int)
            // NetworkVarNames: m_iActualHealthRemoved (int)
            // NetworkVarNames: m_iNumHits (int)
            // NetworkVarNames: m_iLastBulletUpdate (int)
            // NetworkVarNames: m_bIsOtherEnemy (bool)
            // NetworkVarNames: m_killType (EKillTypes_t)
            pub mod CDamageRecord {
                pub const m_PlayerDamager: usize = 0x28; // 
                pub const m_PlayerRecipient: usize = 0x2C; // 
                pub const m_hPlayerControllerDamager: usize = 0x30; // 
                pub const m_hPlayerControllerRecipient: usize = 0x34; // 
                pub const m_szPlayerDamagerName: usize = 0x38; // 
                pub const m_szPlayerRecipientName: usize = 0x40; // 
                pub const m_DamagerXuid: usize = 0x48; // 
                pub const m_RecipientXuid: usize = 0x50; // 
                pub const m_iBulletsDamage: usize = 0x58; // 
                pub const m_iDamage: usize = 0x5C; // 
                pub const m_iActualHealthRemoved: usize = 0x60; // 
                pub const m_iNumHits: usize = 0x64; // 
                pub const m_iLastBulletUpdate: usize = 0x68; // 
                pub const m_bIsOtherEnemy: usize = 0x6C; // 
                pub const m_killType: usize = 0x6D; // 
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod C_InfoLadderDismount {
            }
            // Parent: CPlayerControllerComponent
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_nSendUpdate (int)
            // NetworkVarNames: m_DamageList (CDamageRecord)
            pub mod CCSPlayerController_DamageServices {
                pub const m_nSendUpdate: usize = 0x40; // 
                pub const m_DamageList: usize = 0x48; // C_UtlVectorEmbeddedNetworkVar<CDamageRecord>
            }
            // Parent: CEntityComponent
            // Field count: 5
            pub mod CRenderComponent {
                pub const __m_pChainEntity: usize = 0x10; // 
                pub const m_bIsRenderingWithViewModels: usize = 0x50; // 
                pub const m_nSplitscreenFlags: usize = 0x54; // 
                pub const m_bEnableRendering: usize = 0x60; // 
                pub const m_bInterpolationReadyToDraw: usize = 0xB0; // 
            }
            // Parent: C_SoundEventEntity
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_vMins (Vector)
            // NetworkVarNames: m_vMaxs (Vector)
            pub mod C_SoundEventOBBEntity {
                pub const m_vMins: usize = 0x628; // 
                pub const m_vMaxs: usize = 0x634; // 
            }
            // Parent: CEntityComponent
            // Field count: 66
            //
            // Metadata:
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_SecondaryColor (Color)
            // NetworkVarNames: m_flBrightness (float)
            // NetworkVarNames: m_flBrightnessScale (float)
            // NetworkVarNames: m_flBrightnessMult (float)
            // NetworkVarNames: m_flRange (float)
            // NetworkVarNames: m_flFalloff (float)
            // NetworkVarNames: m_flAttenuation0 (float)
            // NetworkVarNames: m_flAttenuation1 (float)
            // NetworkVarNames: m_flAttenuation2 (float)
            // NetworkVarNames: m_flTheta (float)
            // NetworkVarNames: m_flPhi (float)
            // NetworkVarNames: m_hLightCookie (HRenderTextureStrong)
            // NetworkVarNames: m_nCascades (int)
            // NetworkVarNames: m_nCastShadows (int)
            // NetworkVarNames: m_nShadowWidth (int)
            // NetworkVarNames: m_nShadowHeight (int)
            // NetworkVarNames: m_bRenderDiffuse (bool)
            // NetworkVarNames: m_nRenderSpecular (int)
            // NetworkVarNames: m_bRenderTransmissive (bool)
            // NetworkVarNames: m_flOrthoLightWidth (float)
            // NetworkVarNames: m_flOrthoLightHeight (float)
            // NetworkVarNames: m_nStyle (int)
            // NetworkVarNames: m_Pattern (CUtlString)
            // NetworkVarNames: m_nCascadeRenderStaticObjects (int)
            // NetworkVarNames: m_flShadowCascadeCrossFade (float)
            // NetworkVarNames: m_flShadowCascadeDistanceFade (float)
            // NetworkVarNames: m_flShadowCascadeDistance0 (float)
            // NetworkVarNames: m_flShadowCascadeDistance1 (float)
            // NetworkVarNames: m_flShadowCascadeDistance2 (float)
            // NetworkVarNames: m_flShadowCascadeDistance3 (float)
            // NetworkVarNames: m_nShadowCascadeResolution0 (int)
            // NetworkVarNames: m_nShadowCascadeResolution1 (int)
            // NetworkVarNames: m_nShadowCascadeResolution2 (int)
            // NetworkVarNames: m_nShadowCascadeResolution3 (int)
            // NetworkVarNames: m_bUsesBakedShadowing (bool)
            // NetworkVarNames: m_nShadowPriority (int)
            // NetworkVarNames: m_nBakedShadowIndex (int)
            // NetworkVarNames: m_bRenderToCubemaps (bool)
            // NetworkVarNames: m_nDirectLight (int)
            // NetworkVarNames: m_nIndirectLight (int)
            // NetworkVarNames: m_flFadeMinDist (float)
            // NetworkVarNames: m_flFadeMaxDist (float)
            // NetworkVarNames: m_flShadowFadeMinDist (float)
            // NetworkVarNames: m_flShadowFadeMaxDist (float)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bFlicker (bool)
            // NetworkVarNames: m_bPrecomputedFieldsValid (bool)
            // NetworkVarNames: m_vPrecomputedBoundsMins (Vector)
            // NetworkVarNames: m_vPrecomputedBoundsMaxs (Vector)
            // NetworkVarNames: m_vPrecomputedOBBOrigin (Vector)
            // NetworkVarNames: m_vPrecomputedOBBAngles (QAngle)
            // NetworkVarNames: m_vPrecomputedOBBExtent (Vector)
            // NetworkVarNames: m_flPrecomputedMaxRange (float)
            // NetworkVarNames: m_nFogLightingMode (int)
            // NetworkVarNames: m_flFogContributionStength (float)
            // NetworkVarNames: m_flNearClipPlane (float)
            // NetworkVarNames: m_SkyColor (Color)
            // NetworkVarNames: m_flSkyIntensity (float)
            // NetworkVarNames: m_SkyAmbientBounce (Color)
            // NetworkVarNames: m_bUseSecondaryColor (bool)
            // NetworkVarNames: m_bMixedShadows (bool)
            // NetworkVarNames: m_flLightStyleStartTime (GameTime_t)
            // NetworkVarNames: m_flCapsuleLength (float)
            // NetworkVarNames: m_flMinRoughness (float)
            pub mod CLightComponent {
                pub const __m_pChainEntity: usize = 0x38; // 
                pub const m_Color: usize = 0x75; // 
                pub const m_SecondaryColor: usize = 0x79; // 
                pub const m_flBrightness: usize = 0x80; // 
                pub const m_flBrightnessScale: usize = 0x84; // 
                pub const m_flBrightnessMult: usize = 0x88; // 
                pub const m_flRange: usize = 0x8C; // 
                pub const m_flFalloff: usize = 0x90; // 
                pub const m_flAttenuation0: usize = 0x94; // 
                pub const m_flAttenuation1: usize = 0x98; // 
                pub const m_flAttenuation2: usize = 0x9C; // 
                pub const m_flTheta: usize = 0xA0; // 
                pub const m_flPhi: usize = 0xA4; // 
                pub const m_hLightCookie: usize = 0xA8; // 
                pub const m_nCascades: usize = 0xB0; // 
                pub const m_nCastShadows: usize = 0xB4; // 
                pub const m_nShadowWidth: usize = 0xB8; // 
                pub const m_nShadowHeight: usize = 0xBC; // 
                pub const m_bRenderDiffuse: usize = 0xC0; // 
                pub const m_nRenderSpecular: usize = 0xC4; // 
                pub const m_bRenderTransmissive: usize = 0xC8; // 
                pub const m_flOrthoLightWidth: usize = 0xCC; // 
                pub const m_flOrthoLightHeight: usize = 0xD0; // 
                pub const m_nStyle: usize = 0xD4; // 
                pub const m_Pattern: usize = 0xD8; // 
                pub const m_nCascadeRenderStaticObjects: usize = 0xE0; // 
                pub const m_flShadowCascadeCrossFade: usize = 0xE4; // 
                pub const m_flShadowCascadeDistanceFade: usize = 0xE8; // 
                pub const m_flShadowCascadeDistance0: usize = 0xEC; // 
                pub const m_flShadowCascadeDistance1: usize = 0xF0; // 
                pub const m_flShadowCascadeDistance2: usize = 0xF4; // 
                pub const m_flShadowCascadeDistance3: usize = 0xF8; // 
                pub const m_nShadowCascadeResolution0: usize = 0xFC; // 
                pub const m_nShadowCascadeResolution1: usize = 0x100; // 
                pub const m_nShadowCascadeResolution2: usize = 0x104; // 
                pub const m_nShadowCascadeResolution3: usize = 0x108; // 
                pub const m_bUsesBakedShadowing: usize = 0x10C; // 
                pub const m_nShadowPriority: usize = 0x110; // 
                pub const m_nBakedShadowIndex: usize = 0x114; // 
                pub const m_bRenderToCubemaps: usize = 0x118; // 
                pub const m_nDirectLight: usize = 0x11C; // 
                pub const m_nIndirectLight: usize = 0x120; // 
                pub const m_flFadeMinDist: usize = 0x124; // 
                pub const m_flFadeMaxDist: usize = 0x128; // 
                pub const m_flShadowFadeMinDist: usize = 0x12C; // 
                pub const m_flShadowFadeMaxDist: usize = 0x130; // 
                pub const m_bEnabled: usize = 0x134; // 
                pub const m_bFlicker: usize = 0x135; // 
                pub const m_bPrecomputedFieldsValid: usize = 0x136; // 
                pub const m_vPrecomputedBoundsMins: usize = 0x138; // 
                pub const m_vPrecomputedBoundsMaxs: usize = 0x144; // 
                pub const m_vPrecomputedOBBOrigin: usize = 0x150; // 
                pub const m_vPrecomputedOBBAngles: usize = 0x15C; // 
                pub const m_vPrecomputedOBBExtent: usize = 0x168; // 
                pub const m_flPrecomputedMaxRange: usize = 0x174; // 
                pub const m_nFogLightingMode: usize = 0x178; // 
                pub const m_flFogContributionStength: usize = 0x17C; // 
                pub const m_flNearClipPlane: usize = 0x180; // 
                pub const m_SkyColor: usize = 0x184; // 
                pub const m_flSkyIntensity: usize = 0x188; // 
                pub const m_SkyAmbientBounce: usize = 0x18C; // 
                pub const m_bUseSecondaryColor: usize = 0x190; // 
                pub const m_bMixedShadows: usize = 0x191; // 
                pub const m_flLightStyleStartTime: usize = 0x194; // 
                pub const m_flCapsuleLength: usize = 0x198; // 
                pub const m_flMinRoughness: usize = 0x19C; // 
            }
            // Parent: C_CSGO_TeamPreviewCamera
            // Field count: 0
            pub mod C_CSGO_CounterTerroristTeamIntroCamera {
            }
            // Parent: None
            // Field count: 0
            pub mod CEntityComponent {
            }
            // Parent: IEconItemInterface
            // Field count: 29
            //
            // Metadata:
            // NetworkVarNames: m_iItemDefinitionIndex (item_definition_index_t)
            // NetworkVarNames: m_iEntityQuality (int)
            // NetworkVarNames: m_iEntityLevel (uint32)
            // NetworkVarNames: m_iItemIDHigh (uint32)
            // NetworkVarNames: m_iItemIDLow (uint32)
            // NetworkVarNames: m_iAccountID (uint32)
            // NetworkVarNames: m_iInventoryPosition (uint32)
            // NetworkVarNames: m_bInitialized (bool)
            // NetworkVarNames: m_AttributeList (CAttributeList)
            // NetworkVarNames: m_NetworkedDynamicAttributes (CAttributeList)
            // NetworkVarNames: m_szCustomName (char)
            pub mod C_EconItemView {
                pub const m_bInventoryImageRgbaRequested: usize = 0x60; // 
                pub const m_bInventoryImageTriedCache: usize = 0x61; // 
                pub const m_nInventoryImageRgbaWidth: usize = 0x80; // 
                pub const m_nInventoryImageRgbaHeight: usize = 0x84; // 
                pub const m_szCurrentLoadCachedFileName: usize = 0x88; // 
                pub const m_bRestoreCustomMaterialAfterPrecache: usize = 0x1B8; // 
                pub const m_iItemDefinitionIndex: usize = 0x1BA; // 
                pub const m_iEntityQuality: usize = 0x1BC; // 
                pub const m_iEntityLevel: usize = 0x1C0; // 
                pub const m_iItemID: usize = 0x1C8; // 
                pub const m_iItemIDHigh: usize = 0x1D0; // 
                pub const m_iItemIDLow: usize = 0x1D4; // 
                pub const m_iAccountID: usize = 0x1D8; // 
                pub const m_iInventoryPosition: usize = 0x1DC; // 
                pub const m_bInitialized: usize = 0x1E8; // 
                pub const m_bDisallowSOC: usize = 0x1E9; // 
                pub const m_bIsStoreItem: usize = 0x1EA; // 
                pub const m_bIsTradeItem: usize = 0x1EB; // 
                pub const m_iEntityQuantity: usize = 0x1EC; // 
                pub const m_iRarityOverride: usize = 0x1F0; // 
                pub const m_iQualityOverride: usize = 0x1F4; // 
                pub const m_iOriginOverride: usize = 0x1F8; // 
                pub const m_unClientFlags: usize = 0x1FC; // 
                pub const m_unOverrideStyle: usize = 0x1FD; // 
                pub const m_AttributeList: usize = 0x210; // 
                pub const m_NetworkedDynamicAttributes: usize = 0x270; // 
                pub const m_szCustomName: usize = 0x2D0; // 
                pub const m_szCustomNameOverride: usize = 0x371; // 
                pub const m_bInitializedTags: usize = 0x440; // 
            }
            // Parent: C_BaseEntity
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_Handle (CHandle<CBaseEntity>)
            // NetworkVarNames: m_bSendHandle (bool)
            pub mod C_HandleTest {
                pub const m_Handle: usize = 0x568; // 
                pub const m_bSendHandle: usize = 0x56C; // 
            }
            // Parent: C_BaseEntity
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_EnvWindShared (CEnvWindShared)
            pub mod C_EnvWindClientside {
                pub const m_EnvWindShared: usize = 0x568; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponMAC10 {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_flDisplayHealth (float)
            pub mod C_WeaponShield {
                pub const m_flDisplayHealth: usize = 0x1B30; // 
            }
            // Parent: C_PointClientUIWorldPanel
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_messageText (char)
            pub mod C_PointClientUIWorldTextPanel {
                pub const m_messageText: usize = 0xF90; // 
            }
            // Parent: CPlayer_UseServices
            // Field count: 0
            pub mod CCSPlayer_UseServices {
            }
            // Parent: CPlayerPawnComponent
            // Field count: 15
            //
            // Metadata:
            // NetworkVarNames: m_nToggleButtonDownMask (ButtonBitMask_t)
            // NetworkVarNames: m_flMaxspeed (float32)
            // NetworkVarNames: m_arrForceSubtickMoveWhen (float32)
            pub mod CPlayer_MovementServices {
                pub const m_nImpulse: usize = 0x40; // 
                pub const m_nButtons: usize = 0x48; // 
                pub const m_nQueuedButtonDownMask: usize = 0x68; // 
                pub const m_nQueuedButtonChangeMask: usize = 0x70; // 
                pub const m_nButtonDoublePressed: usize = 0x78; // 
                pub const m_pButtonPressedCmdNumber: usize = 0x80; // 
                pub const m_nLastCommandNumberProcessed: usize = 0x180; // 
                pub const m_nToggleButtonDownMask: usize = 0x188; // 
                pub const m_flMaxspeed: usize = 0x198; // 
                pub const m_arrForceSubtickMoveWhen: usize = 0x19C; // 
                pub const m_flForwardMove: usize = 0x1AC; // 
                pub const m_flLeftMove: usize = 0x1B0; // 
                pub const m_flUpMove: usize = 0x1B4; // 
                pub const m_vecLastMovementImpulses: usize = 0x1B8; // 
                pub const m_vecOldViewAngles: usize = 0x1C4; // 
            }
            // Parent: CEntitySubclassVDataBase
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CBasePlayerVData {
                pub const m_sModelName: usize = 0x28; // 
                pub const m_flHeadDamageMultiplier: usize = 0x108; // 
                pub const m_flChestDamageMultiplier: usize = 0x118; // 
                pub const m_flStomachDamageMultiplier: usize = 0x128; // 
                pub const m_flArmDamageMultiplier: usize = 0x138; // 
                pub const m_flLegDamageMultiplier: usize = 0x148; // 
                pub const m_flHoldBreathTime: usize = 0x158; // 
                pub const m_flDrowningDamageInterval: usize = 0x15C; // 
                pub const m_nDrowningDamageInitial: usize = 0x160; // 
                pub const m_nDrowningDamageMax: usize = 0x164; // 
                pub const m_nWaterSpeed: usize = 0x168; // 
                pub const m_flUseRange: usize = 0x16C; // 
                pub const m_flUseAngleTolerance: usize = 0x170; // 
                pub const m_flCrouchTime: usize = 0x174; // 
            }
            // Parent: CPlayerControllerComponent
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_iAccount (int)
            // NetworkVarNames: m_iStartAccount (int)
            // NetworkVarNames: m_iTotalCashSpent (int)
            // NetworkVarNames: m_iCashSpentThisRound (int)
            pub mod CCSPlayerController_InGameMoneyServices {
                pub const m_iAccount: usize = 0x40; // 
                pub const m_iStartAccount: usize = 0x44; // 
                pub const m_iTotalCashSpent: usize = 0x48; // 
                pub const m_iCashSpentThisRound: usize = 0x4C; // 
            }
            // Parent: None
            // Field count: 2
            pub mod C_EnvWindShared__WindVariationEvent_t {
                pub const m_flWindAngleVariation: usize = 0x0; // 
                pub const m_flWindSpeedVariation: usize = 0x4; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponUSPSilencer {
            }
            // Parent: C_Sprite
            // Field count: 0
            pub mod CSpriteOriented {
            }
            // Parent: C_CSGO_EndOfMatchLineupEndpoint
            // Field count: 0
            pub mod C_CSGO_EndOfMatchLineupStart {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponFamas {
            }
            // Parent: C_ParticleSystem
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_flAlphaScale (float32)
            // NetworkVarNames: m_flRadiusScale (float32)
            // NetworkVarNames: m_flSelfIllumScale (float32)
            // NetworkVarNames: m_ColorTint (Color)
            // NetworkVarNames: m_hTextureOverride (HRenderTextureStrong)
            pub mod C_EnvParticleGlow {
                pub const m_flAlphaScale: usize = 0x12D8; // 
                pub const m_flRadiusScale: usize = 0x12DC; // 
                pub const m_flSelfIllumScale: usize = 0x12E0; // 
                pub const m_ColorTint: usize = 0x12E4; // 
                pub const m_hTextureOverride: usize = 0x12E8; // 
            }
            // Parent: C_BaseEntity
            // Field count: 15
            pub mod C_SoundEventEntity {
                pub const m_bStartOnSpawn: usize = 0x568; // 
                pub const m_bToLocalPlayer: usize = 0x569; // 
                pub const m_bStopOnNew: usize = 0x56A; // 
                pub const m_bSaveRestore: usize = 0x56B; // 
                pub const m_bSavedIsPlaying: usize = 0x56C; // 
                pub const m_flSavedElapsedTime: usize = 0x570; // 
                pub const m_iszSourceEntityName: usize = 0x578; // 
                pub const m_iszAttachmentName: usize = 0x580; // 
                pub const m_onGUIDChanged: usize = 0x588; // 
                pub const m_onSoundFinished: usize = 0x5B0; // 
                pub const m_flClientCullRadius: usize = 0x5D8; // 
                pub const m_iszSoundName: usize = 0x608; // 
                pub const m_hSource: usize = 0x618; // 
                pub const m_nEntityIndexSelection: usize = 0x61C; // 
                pub const m_bClientSideOnly: usize = 0x0; // 
            }
            // Parent: C_CSGO_TeamPreviewCharacterPosition
            // Field count: 0
            pub mod C_CSGO_TeamIntroCharacterPosition {
            }
            // Parent: C_CSGO_PreviewModel
            // Field count: 0
            pub mod C_CSGO_PreviewModelAlias_csgo_item_previewmodel {
            }
            // Parent: C_BaseModelEntity
            // Field count: 18
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkExcludeByName
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // NetworkVarNames: m_vDirection (Vector)
            // NetworkVarNames: m_iszEffectName (string_t)
            // NetworkVarNames: m_iszSSEffectName (string_t)
            // NetworkVarNames: m_clrOverlay (Color)
            // NetworkVarNames: m_bOn (bool)
            // NetworkVarNames: m_bmaxColor (bool)
            // NetworkVarNames: m_flSize (float32)
            // NetworkVarNames: m_flHazeScale (float32)
            // NetworkVarNames: m_flRotation (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            // NetworkVarNames: m_flAlphaHaze (float32)
            // NetworkVarNames: m_flAlphaScale (float32)
            // NetworkVarNames: m_flAlphaHdr (float32)
            // NetworkVarNames: m_flFarZScale (float32)
            pub mod C_Sun {
                pub const m_fxSSSunFlareEffectIndex: usize = 0xD28; // 
                pub const m_fxSunFlareEffectIndex: usize = 0xD2C; // 
                pub const m_fdistNormalize: usize = 0xD30; // 
                pub const m_vSunPos: usize = 0xD34; // 
                pub const m_vDirection: usize = 0xD40; // 
                pub const m_iszEffectName: usize = 0xD50; // 
                pub const m_iszSSEffectName: usize = 0xD58; // 
                pub const m_clrOverlay: usize = 0xD60; // 
                pub const m_bOn: usize = 0xD64; // 
                pub const m_bmaxColor: usize = 0xD65; // 
                pub const m_flSize: usize = 0xD68; // 
                pub const m_flHazeScale: usize = 0xD6C; // 
                pub const m_flRotation: usize = 0xD70; // 
                pub const m_flHDRColorScale: usize = 0xD74; // 
                pub const m_flAlphaHaze: usize = 0xD78; // 
                pub const m_flAlphaScale: usize = 0xD7C; // 
                pub const m_flAlphaHdr: usize = 0xD80; // 
                pub const m_flFarZScale: usize = 0xD84; // 
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod C_CSGO_EndOfMatchLineupEndpoint {
            }
            // Parent: CPlayerControllerComponent
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_perRoundStats (CSPerRoundStats_t)
            // NetworkVarNames: m_matchStats (CSMatchStats_t)
            // NetworkVarNames: m_iNumRoundKills (int)
            // NetworkVarNames: m_iNumRoundKillsHeadshots (int)
            // NetworkVarNames: m_unTotalRoundDamageDealt (uint32)
            pub mod CCSPlayerController_ActionTrackingServices {
                pub const m_perRoundStats: usize = 0x40; // 
                pub const m_matchStats: usize = 0x90; // 
                pub const m_iNumRoundKills: usize = 0x110; // 
                pub const m_iNumRoundKillsHeadshots: usize = 0x114; // 
                pub const m_unTotalRoundDamageDealt: usize = 0x118; // 
            }
            // Parent: CEntityComponent
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bvDisabledHitGroups (uint32)
            pub mod CHitboxComponent {
                pub const m_bvDisabledHitGroups: usize = 0x24; // 
            }
            // Parent: C_BaseTrigger
            // Field count: 9
            //
            // Metadata:
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_MaxWeight (float)
            // NetworkVarNames: m_FadeDuration (float)
            // NetworkVarNames: m_Weight (float)
            // NetworkVarNames: m_lookupFilename (char)
            pub mod C_ColorCorrectionVolume {
                pub const m_LastEnterWeight: usize = 0xD30; // 
                pub const m_LastEnterTime: usize = 0xD34; // 
                pub const m_LastExitWeight: usize = 0xD38; // 
                pub const m_LastExitTime: usize = 0xD3C; // 
                pub const m_bEnabled: usize = 0xD40; // 
                pub const m_MaxWeight: usize = 0xD44; // 
                pub const m_FadeDuration: usize = 0xD48; // 
                pub const m_Weight: usize = 0xD4C; // 
                pub const m_lookupFilename: usize = 0xD50; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod C_PrecipitationBlocker {
            }
            // Parent: CBaseAnimGraph
            // Field count: 6
            pub mod C_BulletHitModel {
                pub const m_matLocal: usize = 0xF88; // 
                pub const m_iBoneIndex: usize = 0xFB8; // 
                pub const m_hPlayerParent: usize = 0xFBC; // 
                pub const m_bIsHit: usize = 0xFC0; // 
                pub const m_flTimeCreated: usize = 0xFC4; // 
                pub const m_vecStartPos: usize = 0xFC8; // 
            }
            // Parent: C_BaseEntity
            // Field count: 17
            //
            // Metadata:
            // NetworkVarNames: m_nDraftType (int)
            // NetworkVarNames: m_nTeamWinningCoinToss (int)
            // NetworkVarNames: m_nTeamWithFirstChoice (int)
            // NetworkVarNames: m_nVoteMapIdsList (int)
            // NetworkVarNames: m_nAccountIDs (int)
            // NetworkVarNames: m_nMapId0 (int)
            // NetworkVarNames: m_nMapId1 (int)
            // NetworkVarNames: m_nMapId2 (int)
            // NetworkVarNames: m_nMapId3 (int)
            // NetworkVarNames: m_nMapId4 (int)
            // NetworkVarNames: m_nMapId5 (int)
            // NetworkVarNames: m_nStartingSide0 (int)
            // NetworkVarNames: m_nCurrentPhase (int)
            // NetworkVarNames: m_nPhaseStartTick (int)
            // NetworkVarNames: m_nPhaseDurationTicks (int)
            pub mod C_MapVetoPickController {
                pub const m_nDraftType: usize = 0x578; // 
                pub const m_nTeamWinningCoinToss: usize = 0x57C; // 
                pub const m_nTeamWithFirstChoice: usize = 0x580; // 
                pub const m_nVoteMapIdsList: usize = 0x680; // 
                pub const m_nAccountIDs: usize = 0x69C; // 
                pub const m_nMapId0: usize = 0x79C; // 
                pub const m_nMapId1: usize = 0x89C; // 
                pub const m_nMapId2: usize = 0x99C; // 
                pub const m_nMapId3: usize = 0xA9C; // 
                pub const m_nMapId4: usize = 0xB9C; // 
                pub const m_nMapId5: usize = 0xC9C; // 
                pub const m_nStartingSide0: usize = 0xD9C; // 
                pub const m_nCurrentPhase: usize = 0xE9C; // 
                pub const m_nPhaseStartTick: usize = 0xEA0; // 
                pub const m_nPhaseDurationTicks: usize = 0xEA4; // 
                pub const m_nPostDataUpdateTick: usize = 0xEA8; // 
                pub const m_bDisabledHud: usize = 0xEAC; // 
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGapTypeQueriesForScopeSingleton
            pub mod CSharedGapTypeQueryRegistration {
            }
            // Parent: C_BarnLight
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_flInnerAngle (float)
            // NetworkVarNames: m_flOuterAngle (float)
            // NetworkVarNames: m_bShowLight (bool)
            pub mod C_OmniLight {
                pub const m_flInnerAngle: usize = 0x1070; // 
                pub const m_flOuterAngle: usize = 0x1074; // 
                pub const m_bShowLight: usize = 0x1078; // 
            }
            // Parent: C_Inferno
            // Field count: 0
            pub mod C_FireCrackerBlast {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponMP9 {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_weaponPurchases (WeaponPurchaseCount_t)
            pub mod WeaponPurchaseTracker_t {
                pub const m_weaponPurchases: usize = 0x8; // 
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_nMatchSeed (int)
            // NetworkVarNames: m_bBlockersPresent (bool)
            // NetworkVarNames: m_bRoundInProgress (bool)
            // NetworkVarNames: m_iFirstSecondHalfRound (int)
            // NetworkVarNames: m_iBombSite (int)
            pub mod C_RetakeGameRules {
                pub const m_nMatchSeed: usize = 0xF8; // 
                pub const m_bBlockersPresent: usize = 0xFC; // 
                pub const m_bRoundInProgress: usize = 0xFD; // 
                pub const m_iFirstSecondHalfRound: usize = 0x100; // 
                pub const m_iBombSite: usize = 0x104; // 
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_duration (float32)
            // NetworkVarNames: m_timestamp (float32)
            // NetworkVarNames: m_timescale (float32)
            pub mod EngineCountdownTimer {
                pub const m_duration: usize = 0x8; // 
                pub const m_timestamp: usize = 0xC; // 
                pub const m_timescale: usize = 0x10; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 0
            pub mod CCSPlayer_GlowServices {
            }
            // Parent: None
            // Field count: 5
            pub mod C_BaseFlex__Emphasized_Phoneme {
                pub const m_sClassName: usize = 0x0; // 
                pub const m_flAmount: usize = 0x18; // 
                pub const m_bRequired: usize = 0x1C; // 
                pub const m_bBasechecked: usize = 0x1D; // 
                pub const m_bValid: usize = 0x1E; // 
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_timestamp (GameTime_t)
            // NetworkVarNames: m_nWorldGroupId (WorldGroupId_t)
            pub mod IntervalTimer {
                pub const m_timestamp: usize = 0x8; // 
                pub const m_nWorldGroupId: usize = 0xC; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponSSG08 {
            }
            // Parent: C_BaseModelEntity
            // Field count: 9
            //
            // Metadata:
            // NetworkVarNames: m_vecLadderDir (Vector)
            // NetworkVarNames: m_vecPlayerMountPositionTop (Vector)
            // NetworkVarNames: m_vecPlayerMountPositionBottom (Vector)
            // NetworkVarNames: m_flAutoRideSpeed (float)
            // NetworkVarNames: m_bFakeLadder (bool)
            pub mod C_FuncLadder {
                pub const m_vecLadderDir: usize = 0xD28; // 
                pub const m_Dismounts: usize = 0xD38; // 
                pub const m_vecLocalTop: usize = 0xD50; // 
                pub const m_vecPlayerMountPositionTop: usize = 0xD5C; // 
                pub const m_vecPlayerMountPositionBottom: usize = 0xD68; // 
                pub const m_flAutoRideSpeed: usize = 0xD74; // 
                pub const m_bDisabled: usize = 0xD78; // 
                pub const m_bFakeLadder: usize = 0xD79; // 
                pub const m_bHasSlack: usize = 0xD7A; // 
            }
            // Parent: C_SoundOpvarSetAABBEntity
            // Field count: 0
            pub mod C_SoundOpvarSetOBBEntity {
            }
            // Parent: CPlayerPawnComponent
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_hMyWeapons (CHandle<C_BasePlayerWeapon>)
            // NetworkVarNames: m_hActiveWeapon (CHandle<CBasePlayerWeapon>)
            // NetworkVarNames: m_hLastWeapon (CHandle<CBasePlayerWeapon>)
            // NetworkVarNames: m_iAmmo (uint16)
            pub mod CPlayer_WeaponServices {
                pub const m_hMyWeapons: usize = 0x40; // 
                pub const m_hActiveWeapon: usize = 0x58; // 
                pub const m_hLastWeapon: usize = 0x5C; // 
                pub const m_iAmmo: usize = 0x60; // 
            }
            // Parent: None
            // Field count: 3
            pub mod CAttributeManager__cached_attribute_float_t {
                pub const flIn: usize = 0x0; // 
                pub const iAttribHook: usize = 0x8; // 
                pub const flOut: usize = 0x10; // 
            }
            // Parent: C_BaseFlex
            // Field count: 12
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_bIsLive (bool)
            // NetworkVarNames: m_DmgRadius (float32)
            // NetworkVarNames: m_flDetonateTime (GameTime_t)
            // NetworkVarNames: m_flDamage (float32)
            // NetworkVarNames: m_hThrower (CHandle<CCSPlayerPawn>)
            pub mod C_BaseGrenade {
                pub const m_bHasWarnedAI: usize = 0x1120; // 
                pub const m_bIsSmokeGrenade: usize = 0x1121; // 
                pub const m_bIsLive: usize = 0x1122; // 
                pub const m_DmgRadius: usize = 0x1124; // 
                pub const m_flDetonateTime: usize = 0x1128; // 
                pub const m_flWarnAITime: usize = 0x112C; // 
                pub const m_flDamage: usize = 0x1130; // 
                pub const m_iszBounceSound: usize = 0x1138; // 
                pub const m_ExplosionSound: usize = 0x1140; // 
                pub const m_hThrower: usize = 0x114C; // 
                pub const m_flNextAttack: usize = 0x1164; // 
                pub const m_hOriginalThrower: usize = 0x1168; // 
            }
            // Parent: None
            // Field count: 1
            pub mod CCSGameModeRules {
                pub const __m_pChainEntity: usize = 0x8; // 
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nInteractsAs (uint64)
            // NetworkVarNames: m_nInteractsWith (uint64)
            // NetworkVarNames: m_nInteractsExclude (uint64)
            // NetworkVarNames: m_nEntityId (uint32)
            // NetworkVarNames: m_nOwnerId (uint32)
            // NetworkVarNames: m_nHierarchyId (uint16)
            // NetworkVarNames: m_nCollisionGroup (uint8)
            // NetworkVarNames: m_nCollisionFunctionMask (uint8)
            pub mod VPhysicsCollisionAttribute_t {
                pub const m_nInteractsAs: usize = 0x8; // 
                pub const m_nInteractsWith: usize = 0x10; // 
                pub const m_nInteractsExclude: usize = 0x18; // 
                pub const m_nEntityId: usize = 0x20; // 
                pub const m_nOwnerId: usize = 0x24; // 
                pub const m_nHierarchyId: usize = 0x28; // 
                pub const m_nCollisionGroup: usize = 0x2A; // 
                pub const m_nCollisionFunctionMask: usize = 0x2B; // 
            }
            // Parent: C_BaseEntity
            // Field count: 34
            //
            // Metadata:
            // NetworkVarNames: m_flScattering (float)
            // NetworkVarNames: m_flAnisotropy (float)
            // NetworkVarNames: m_flFadeSpeed (float)
            // NetworkVarNames: m_flDrawDistance (float)
            // NetworkVarNames: m_flFadeInStart (float)
            // NetworkVarNames: m_flFadeInEnd (float)
            // NetworkVarNames: m_flIndirectStrength (float)
            // NetworkVarNames: m_nVolumeDepth (int)
            // NetworkVarNames: m_fFirstVolumeSliceThickness (float)
            // NetworkVarNames: m_nIndirectTextureDimX (int)
            // NetworkVarNames: m_nIndirectTextureDimY (int)
            // NetworkVarNames: m_nIndirectTextureDimZ (int)
            // NetworkVarNames: m_vBoxMins (Vector)
            // NetworkVarNames: m_vBoxMaxs (Vector)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_flStartAnisoTime (GameTime_t)
            // NetworkVarNames: m_flStartScatterTime (GameTime_t)
            // NetworkVarNames: m_flStartDrawDistanceTime (GameTime_t)
            // NetworkVarNames: m_flStartAnisotropy (float)
            // NetworkVarNames: m_flStartScattering (float)
            // NetworkVarNames: m_flStartDrawDistance (float)
            // NetworkVarNames: m_flDefaultAnisotropy (float)
            // NetworkVarNames: m_flDefaultScattering (float)
            // NetworkVarNames: m_flDefaultDrawDistance (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bEnableIndirect (bool)
            // NetworkVarNames: m_bIndirectUseLPVs (bool)
            // NetworkVarNames: m_bIsMaster (bool)
            // NetworkVarNames: m_hFogIndirectTexture (HRenderTextureStrong)
            // NetworkVarNames: m_nForceRefreshCount (int)
            // NetworkVarNames: m_fNoiseSpeed (float)
            // NetworkVarNames: m_fNoiseStrength (float)
            // NetworkVarNames: m_vNoiseScale (Vector)
            pub mod C_EnvVolumetricFogController {
                pub const m_flScattering: usize = 0x568; // 
                pub const m_flAnisotropy: usize = 0x56C; // 
                pub const m_flFadeSpeed: usize = 0x570; // 
                pub const m_flDrawDistance: usize = 0x574; // 
                pub const m_flFadeInStart: usize = 0x578; // 
                pub const m_flFadeInEnd: usize = 0x57C; // 
                pub const m_flIndirectStrength: usize = 0x580; // 
                pub const m_nVolumeDepth: usize = 0x584; // 
                pub const m_fFirstVolumeSliceThickness: usize = 0x588; // 
                pub const m_nIndirectTextureDimX: usize = 0x58C; // 
                pub const m_nIndirectTextureDimY: usize = 0x590; // 
                pub const m_nIndirectTextureDimZ: usize = 0x594; // 
                pub const m_vBoxMins: usize = 0x598; // 
                pub const m_vBoxMaxs: usize = 0x5A4; // 
                pub const m_bActive: usize = 0x5B0; // 
                pub const m_flStartAnisoTime: usize = 0x5B4; // 
                pub const m_flStartScatterTime: usize = 0x5B8; // 
                pub const m_flStartDrawDistanceTime: usize = 0x5BC; // 
                pub const m_flStartAnisotropy: usize = 0x5C0; // 
                pub const m_flStartScattering: usize = 0x5C4; // 
                pub const m_flStartDrawDistance: usize = 0x5C8; // 
                pub const m_flDefaultAnisotropy: usize = 0x5CC; // 
                pub const m_flDefaultScattering: usize = 0x5D0; // 
                pub const m_flDefaultDrawDistance: usize = 0x5D4; // 
                pub const m_bStartDisabled: usize = 0x5D8; // 
                pub const m_bEnableIndirect: usize = 0x5D9; // 
                pub const m_bIndirectUseLPVs: usize = 0x5DA; // 
                pub const m_bIsMaster: usize = 0x5DB; // 
                pub const m_hFogIndirectTexture: usize = 0x5E0; // 
                pub const m_nForceRefreshCount: usize = 0x5E8; // 
                pub const m_fNoiseSpeed: usize = 0x5EC; // 
                pub const m_fNoiseStrength: usize = 0x5F0; // 
                pub const m_vNoiseScale: usize = 0x5F4; // 
                pub const m_bFirstTime: usize = 0x600; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponTec9 {
            }
            // Parent: CBaseAnimGraph
            // Field count: 13
            //
            // Metadata:
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_flStartTimeInCommentary (float32)
            // NetworkVarNames: m_iszCommentaryFile (string_t)
            // NetworkVarNames: m_iszTitle (string_t)
            // NetworkVarNames: m_iszSpeakers (string_t)
            // NetworkVarNames: m_iNodeNumber (int)
            // NetworkVarNames: m_iNodeNumberMax (int)
            // NetworkVarNames: m_bListenedTo (bool)
            // NetworkVarNames: m_hViewPosition (CHandle<C_BaseEntity>)
            pub mod C_PointCommentaryNode {
                pub const m_bActive: usize = 0xF90; // 
                pub const m_bWasActive: usize = 0xF91; // 
                pub const m_flEndTime: usize = 0xF94; // 
                pub const m_flStartTime: usize = 0xF98; // 
                pub const m_flStartTimeInCommentary: usize = 0xF9C; // 
                pub const m_iszCommentaryFile: usize = 0xFA0; // 
                pub const m_iszTitle: usize = 0xFA8; // 
                pub const m_iszSpeakers: usize = 0xFB0; // 
                pub const m_iNodeNumber: usize = 0xFB8; // 
                pub const m_iNodeNumberMax: usize = 0xFBC; // 
                pub const m_bListenedTo: usize = 0xFC0; // 
                pub const m_hViewPosition: usize = 0xFD0; // 
                pub const m_bRestartAfterRestore: usize = 0xFD4; // 
            }
            // Parent: CCSPlayerBase_CameraServices
            // Field count: 0
            pub mod CCSObserver_CameraServices {
            }
            // Parent: CCSPointScriptEntity
            // Field count: 0
            pub mod CCSClientPointScriptEntity {
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: localSound (Vector)
            // NetworkVarNames: soundscapeIndex (int32)
            // NetworkVarNames: localBits (uint8)
            // NetworkVarNames: soundscapeEntityListIndex (int)
            // NetworkVarNames: soundEventHash (uint32)
            pub mod audioparams_t {
                pub const localSound: usize = 0x8; // 
                pub const soundscapeIndex: usize = 0x68; // 
                pub const localBits: usize = 0x6C; // 
                pub const soundscapeEntityListIndex: usize = 0x70; // 
                pub const soundEventHash: usize = 0x74; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 0
            pub mod C_CS2WeaponModuleBase {
            }
            // Parent: C_BaseEntity
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_nMode (int)
            // NetworkVarNames: m_vBoxSize (Vector)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod C_InfoVisibilityBox {
                pub const m_nMode: usize = 0x56C; // 
                pub const m_vBoxSize: usize = 0x570; // 
                pub const m_bEnabled: usize = 0x57C; // 
            }
            // Parent: C_Sprite
            // Field count: 2
            pub mod C_FireSprite {
                pub const m_vecMoveDir: usize = 0xE38; // 
                pub const m_bFadeFromAbove: usize = 0xE44; // 
            }
            // Parent: C_CSGO_TeamIntroCharacterPosition
            // Field count: 0
            pub mod C_CSGO_TeamIntroTerroristPosition {
            }
            // Parent: C_BaseTrigger
            // Field count: 8
            pub mod C_Precipitation {
                pub const m_flDensity: usize = 0xD30; // 
                pub const m_flParticleInnerDist: usize = 0xD40; // 
                pub const m_pParticleDef: usize = 0xD48; // 
                pub const m_tParticlePrecipTraceTimer: usize = 0xD70; // 
                pub const m_bActiveParticlePrecipEmitter: usize = 0xD78; // 
                pub const m_bParticlePrecipInitialized: usize = 0xD79; // 
                pub const m_bHasSimulatedSinceLastSceneObjectUpdate: usize = 0xD7A; // 
                pub const m_nAvailableSheetSequencesMaxIndex: usize = 0xD7C; // 
            }
            // Parent: CEntitySubclassVDataBase
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPrecipitationVData {
                pub const m_szParticlePrecipitationEffect: usize = 0x28; // 
                pub const m_flInnerDistance: usize = 0x108; // 
                pub const m_nAttachType: usize = 0x10C; // 
                pub const m_bBatchSameVolumeType: usize = 0x110; // 
                pub const m_nRTEnvCP: usize = 0x114; // 
                pub const m_nRTEnvCPComponent: usize = 0x118; // 
                pub const m_szModifier: usize = 0x120; // 
            }
            // Parent: None
            // Field count: 6
            pub mod CBuoyancyHelper {
                pub const m_nFluidType: usize = 0x18; // 
                pub const m_flFluidDensity: usize = 0x1C; // 
                pub const m_vecFractionOfWheelSubmergedForWheelFriction: usize = 0x20; // 
                pub const m_vecWheelFrictionScales: usize = 0x38; // 
                pub const m_vecFractionOfWheelSubmergedForWheelDrag: usize = 0x50; // 
                pub const m_vecWheelDrag: usize = 0x68; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_DialogXMLName (string_t)
            // NetworkVarNames: m_PanelClassName (string_t)
            // NetworkVarNames: m_PanelID (string_t)
            pub mod C_BaseClientUIEntity {
                pub const m_bEnabled: usize = 0xD30; // 
                pub const m_DialogXMLName: usize = 0xD38; // 
                pub const m_PanelClassName: usize = 0xD40; // 
                pub const m_PanelID: usize = 0xD48; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 3
            pub mod C_FuncTrackTrain {
                pub const m_nLongAxis: usize = 0xD28; // 
                pub const m_flRadius: usize = 0xD2C; // 
                pub const m_flLineLength: usize = 0xD30; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_bIsRescuing (bool)
            // NetworkVarNames: m_weaponPurchasesThisMatch (WeaponPurchaseTracker_t)
            // NetworkVarNames: m_weaponPurchasesThisRound (WeaponPurchaseTracker_t)
            pub mod CCSPlayer_ActionTrackingServices {
                pub const m_hLastWeaponBeforeC4AutoSwitch: usize = 0x40; // 
                pub const m_bIsRescuing: usize = 0x44; // 
                pub const m_weaponPurchasesThisMatch: usize = 0x48; // 
                pub const m_weaponPurchasesThisRound: usize = 0xA0; // 
            }
            // Parent: None
            // Field count: 13
            //
            // Metadata:
            // NetworkVarNames: m_nModelID (int32)
            // NetworkVarNames: m_hMaterialBase (HMaterialStrong)
            // NetworkVarNames: m_hMaterialDamageOverlay (HMaterialStrong)
            // NetworkVarNames: m_solid (ShardSolid_t)
            // NetworkVarNames: m_vecPanelSize (Vector2D)
            // NetworkVarNames: m_vecStressPositionA (Vector2D)
            // NetworkVarNames: m_vecStressPositionB (Vector2D)
            // NetworkVarNames: m_vecPanelVertices (Vector2D)
            // NetworkVarNames: m_vInitialPanelVertices (Vector4D)
            // NetworkVarNames: m_flGlassHalfThickness (float)
            // NetworkVarNames: m_bHasParent (bool)
            // NetworkVarNames: m_bParentFrozen (bool)
            // NetworkVarNames: m_SurfacePropStringToken (CUtlStringToken)
            pub mod shard_model_desc_t {
                pub const m_nModelID: usize = 0x8; // 
                pub const m_hMaterialBase: usize = 0x10; // 
                pub const m_hMaterialDamageOverlay: usize = 0x18; // 
                pub const m_solid: usize = 0x20; // 
                pub const m_vecPanelSize: usize = 0x24; // 
                pub const m_vecStressPositionA: usize = 0x2C; // 
                pub const m_vecStressPositionB: usize = 0x34; // 
                pub const m_vecPanelVertices: usize = 0x40; // C_NetworkUtlVectorBase<Vector2D>
                pub const m_vInitialPanelVertices: usize = 0x58; // C_NetworkUtlVectorBase<Vector4D>
                pub const m_flGlassHalfThickness: usize = 0x70; // 
                pub const m_bHasParent: usize = 0x74; // 
                pub const m_bParentFrozen: usize = 0x75; // 
                pub const m_SurfacePropStringToken: usize = 0x78; // 
            }
            // Parent: None
            // Field count: 1
            pub mod C_SceneEntity__QueuedEvents_t {
                pub const starttime: usize = 0x0; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 4
            pub mod CBaseProp {
                pub const m_bModelOverrodeBlockLOS: usize = 0xF88; // 
                pub const m_iShapeType: usize = 0xF8C; // 
                pub const m_bConformToCollisionBounds: usize = 0xF90; // 
                pub const m_mPreferredCatchTransform: usize = 0xF94; // 
            }
            // Parent: C_CSGO_PreviewPlayer
            // Field count: 0
            pub mod C_CSGO_PreviewPlayerAlias_csgo_player_previewmodel {
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSPointScriptExtensions_observer {
            }
            // Parent: CLogicalEntity
            // Field count: 12
            pub mod CPointTemplate {
                pub const m_iszWorldName: usize = 0x568; // 
                pub const m_iszSource2EntityLumpName: usize = 0x570; // 
                pub const m_iszEntityFilterName: usize = 0x578; // 
                pub const m_flTimeoutInterval: usize = 0x580; // 
                pub const m_bAsynchronouslySpawnEntities: usize = 0x584; // 
                pub const m_pOutputOnSpawned: usize = 0x588; // 
                pub const m_clientOnlyEntityBehavior: usize = 0x5B0; // PointTemplateClientOnlyEntityBehavior_t
                pub const m_ownerSpawnGroupType: usize = 0x5B4; // PointTemplateOwnerSpawnGroupType_t
                pub const m_createdSpawnGroupHandles: usize = 0x5B8; // 
                pub const m_SpawnedEntityHandles: usize = 0x5D0; // 
                pub const m_ScriptSpawnCallback: usize = 0x5E8; // 
                pub const m_ScriptCallbackScope: usize = 0x5F0; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponHKP2000 {
            }
            // Parent: C_BaseCSGrenade
            // Field count: 0
            pub mod CTripWireFire {
            }
            // Parent: C_BaseFlex
            // Field count: 6
            pub mod C_CSGO_PreviewModel {
                pub const m_animgraph: usize = 0x1120; // 
                pub const m_animgraphCharacterModeString: usize = 0x1128; // 
                pub const m_defaultAnim: usize = 0x1130; // 
                pub const m_nDefaultAnimLoopMode: usize = 0x1138; // 
                pub const m_flInitialModelScale: usize = 0x113C; // 
                pub const m_sInitialWeaponState: usize = 0x1140; // 
            }
            // Parent: C_PointEntity
            // Field count: 5
            pub mod CInfoDynamicShadowHint {
                pub const m_bDisabled: usize = 0x568; // 
                pub const m_flRange: usize = 0x56C; // 
                pub const m_nImportance: usize = 0x570; // 
                pub const m_nLightChoice: usize = 0x574; // 
                pub const m_hLight: usize = 0x578; // 
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CTakeDamageInfoAPI {
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod C_BaseEntityAPI {
            }
            // Parent: C_CSWeaponBase
            // Field count: 0
            pub mod C_WeaponXM1014 {
            }
            // Parent: C_ParticleSystem
            // Field count: 0
            pub mod C_MapPreviewParticleSystem {
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // NetworkVarNames: m_iGlowType (int32)
            // NetworkVarNames: m_iGlowTeam (int32)
            // NetworkVarNames: m_nGlowRange (int32)
            // NetworkVarNames: m_nGlowRangeMin (int32)
            // NetworkVarNames: m_glowColorOverride (Color)
            // NetworkVarNames: m_bFlashing (bool)
            // NetworkVarNames: m_flGlowTime (float)
            // NetworkVarNames: m_flGlowStartTime (float)
            // NetworkVarNames: m_bEligibleForScreenHighlight (bool)
            pub mod CGlowProperty {
                pub const m_fGlowColor: usize = 0x8; // 
                pub const m_iGlowType: usize = 0x30; // 
                pub const m_iGlowTeam: usize = 0x34; // 
                pub const m_nGlowRange: usize = 0x38; // 
                pub const m_nGlowRangeMin: usize = 0x3C; // 
                pub const m_glowColorOverride: usize = 0x40; // 
                pub const m_bFlashing: usize = 0x44; // 
                pub const m_flGlowTime: usize = 0x48; // 
                pub const m_flGlowStartTime: usize = 0x4C; // 
                pub const m_bEligibleForScreenHighlight: usize = 0x50; // 
                pub const m_bGlowing: usize = 0x51; // 
            }
            // Parent: C_BaseTrigger
            // Field count: 13
            //
            // Metadata:
            // NetworkVarNames: m_gravityScale (float)
            // NetworkVarNames: m_linearLimit (float)
            // NetworkVarNames: m_linearDamping (float)
            // NetworkVarNames: m_angularLimit (float)
            // NetworkVarNames: m_angularDamping (float)
            // NetworkVarNames: m_linearForce (float)
            // NetworkVarNames: m_flFrequency (float)
            // NetworkVarNames: m_flDampingRatio (float)
            // NetworkVarNames: m_vecLinearForcePointAt (Vector)
            // NetworkVarNames: m_bCollapseToForcePoint (bool)
            // NetworkVarNames: m_vecLinearForcePointAtWorld (Vector)
            // NetworkVarNames: m_vecLinearForceDirection (Vector)
            // NetworkVarNames: m_bConvertToDebrisWhenPossible (bool)
            pub mod C_TriggerPhysics {
                pub const m_gravityScale: usize = 0xD30; // 
                pub const m_linearLimit: usize = 0xD34; // 
                pub const m_linearDamping: usize = 0xD38; // 
                pub const m_angularLimit: usize = 0xD3C; // 
                pub const m_angularDamping: usize = 0xD40; // 
                pub const m_linearForce: usize = 0xD44; // 
                pub const m_flFrequency: usize = 0xD48; // 
                pub const m_flDampingRatio: usize = 0xD4C; // 
                pub const m_vecLinearForcePointAt: usize = 0xD50; // 
                pub const m_bCollapseToForcePoint: usize = 0xD5C; // 
                pub const m_vecLinearForcePointAtWorld: usize = 0xD60; // 
                pub const m_vecLinearForceDirection: usize = 0xD6C; // 
                pub const m_bConvertToDebrisWhenPossible: usize = 0xD78; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_hCarriedHostage (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hCarriedHostageProp (CHandle<CBaseEntity>)
            pub mod CCSPlayer_HostageServices {
                pub const m_hCarriedHostage: usize = 0x40; // 
                pub const m_hCarriedHostageProp: usize = 0x44; // 
            }
            // Parent: C_CSWeaponBase
            // Field count: 0
            pub mod CBreachCharge {
            }
            // Parent: C_LightEntity
            // Field count: 0
            pub mod C_LightDirectionalEntity {
            }
            // Parent: C_BaseEntity
            // Field count: 18
            //
            // Metadata:
            // NetworkVarNames: m_Entity_hCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_Entity_bCustomCubemapTexture (bool)
            // NetworkVarNames: m_Entity_flInfluenceRadius (float)
            // NetworkVarNames: m_Entity_vBoxProjectMins (Vector)
            // NetworkVarNames: m_Entity_vBoxProjectMaxs (Vector)
            // NetworkVarNames: m_Entity_bMoveable (bool)
            // NetworkVarNames: m_Entity_nHandshake (int)
            // NetworkVarNames: m_Entity_nEnvCubeMapArrayIndex (int)
            // NetworkVarNames: m_Entity_nPriority (int)
            // NetworkVarNames: m_Entity_flEdgeFadeDist (float)
            // NetworkVarNames: m_Entity_vEdgeFadeDists (Vector)
            // NetworkVarNames: m_Entity_flDiffuseScale (float)
            // NetworkVarNames: m_Entity_bStartDisabled (bool)
            // NetworkVarNames: m_Entity_bDefaultEnvMap (bool)
            // NetworkVarNames: m_Entity_bDefaultSpecEnvMap (bool)
            // NetworkVarNames: m_Entity_bIndoorCubeMap (bool)
            // NetworkVarNames: m_Entity_bCopyDiffuseFromDefaultCubemap (bool)
            // NetworkVarNames: m_Entity_bEnabled (bool)
            pub mod C_EnvCubemap {
                pub const m_Entity_hCubemapTexture: usize = 0x5E8; // 
                pub const m_Entity_bCustomCubemapTexture: usize = 0x5F0; // 
                pub const m_Entity_flInfluenceRadius: usize = 0x5F4; // 
                pub const m_Entity_vBoxProjectMins: usize = 0x5F8; // 
                pub const m_Entity_vBoxProjectMaxs: usize = 0x604; // 
                pub const m_Entity_bMoveable: usize = 0x610; // 
                pub const m_Entity_nHandshake: usize = 0x614; // 
                pub const m_Entity_nEnvCubeMapArrayIndex: usize = 0x618; // 
                pub const m_Entity_nPriority: usize = 0x61C; // 
                pub const m_Entity_flEdgeFadeDist: usize = 0x620; // 
                pub const m_Entity_vEdgeFadeDists: usize = 0x624; // 
                pub const m_Entity_flDiffuseScale: usize = 0x630; // 
                pub const m_Entity_bStartDisabled: usize = 0x634; // 
                pub const m_Entity_bDefaultEnvMap: usize = 0x635; // 
                pub const m_Entity_bDefaultSpecEnvMap: usize = 0x636; // 
                pub const m_Entity_bIndoorCubeMap: usize = 0x637; // 
                pub const m_Entity_bCopyDiffuseFromDefaultCubemap: usize = 0x638; // 
                pub const m_Entity_bEnabled: usize = 0x648; // 
            }
            // Parent: C_BaseViewModel
            // Field count: 3
            pub mod C_PredictedViewModel {
                pub const m_vPredictedLagOffset: usize = 0xFF0; // 
                pub const m_targetSpeed: usize = 0xFFC; // 
                pub const m_currentSpeed: usize = 0x1008; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponBizon {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_AK47 {
            }
            // Parent: C_BaseEntity
            // Field count: 25
            //
            // Metadata:
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_bUpdateOnClient (bool)
            // NetworkVarNames: m_nInputType (ValueRemapperInputType_t)
            // NetworkVarNames: m_hRemapLineStart (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hRemapLineEnd (CHandle<CBaseEntity>)
            // NetworkVarNames: m_flMaximumChangePerSecond (float)
            // NetworkVarNames: m_flDisengageDistance (float)
            // NetworkVarNames: m_flEngageDistance (float)
            // NetworkVarNames: m_bRequiresUseKey (bool)
            // NetworkVarNames: m_nOutputType (ValueRemapperOutputType_t)
            // NetworkVarNames: m_hOutputEntities (CHandle<C_BaseEntity>)
            // NetworkVarNames: m_nHapticsType (ValueRemapperHapticsType_t)
            // NetworkVarNames: m_nMomentumType (ValueRemapperMomentumType_t)
            // NetworkVarNames: m_flMomentumModifier (float)
            // NetworkVarNames: m_flSnapValue (float)
            // NetworkVarNames: m_nRatchetType (ValueRemapperRatchetType_t)
            // NetworkVarNames: m_flInputOffset (float)
            pub mod C_PointValueRemapper {
                pub const m_bDisabled: usize = 0x568; // 
                pub const m_bDisabledOld: usize = 0x569; // 
                pub const m_bUpdateOnClient: usize = 0x56A; // 
                pub const m_nInputType: usize = 0x56C; // 
                pub const m_hRemapLineStart: usize = 0x570; // 
                pub const m_hRemapLineEnd: usize = 0x574; // 
                pub const m_flMaximumChangePerSecond: usize = 0x578; // 
                pub const m_flDisengageDistance: usize = 0x57C; // 
                pub const m_flEngageDistance: usize = 0x580; // 
                pub const m_bRequiresUseKey: usize = 0x584; // 
                pub const m_nOutputType: usize = 0x588; // 
                pub const m_hOutputEntities: usize = 0x590; // 
                pub const m_nHapticsType: usize = 0x5A8; // 
                pub const m_nMomentumType: usize = 0x5AC; // 
                pub const m_flMomentumModifier: usize = 0x5B0; // 
                pub const m_flSnapValue: usize = 0x5B4; // 
                pub const m_flCurrentMomentum: usize = 0x5B8; // 
                pub const m_nRatchetType: usize = 0x5BC; // 
                pub const m_flRatchetOffset: usize = 0x5C0; // 
                pub const m_flInputOffset: usize = 0x5C4; // 
                pub const m_bEngaged: usize = 0x5C8; // 
                pub const m_bFirstUpdate: usize = 0x5C9; // 
                pub const m_flPreviousValue: usize = 0x5CC; // 
                pub const m_flPreviousUpdateTickTime: usize = 0x5D0; // 
                pub const m_vecPreviousTestPoint: usize = 0x5D4; // 
            }
            // Parent: C_CSWeaponBase
            // Field count: 0
            pub mod C_Knife {
            }
            // Parent: C_EnvCubemap
            // Field count: 0
            pub mod C_EnvCubemapBox {
            }
            // Parent: C_Item
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_OwningPlayer (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_KillingPlayer (CHandle<CCSPlayerPawn>)
            pub mod C_ItemDogtags {
                pub const m_OwningPlayer: usize = 0x1768; // 
                pub const m_KillingPlayer: usize = 0x176C; // 
            }
            // Parent: C_CSGO_TeamPreviewCamera
            // Field count: 0
            pub mod C_CSGO_EndOfMatchCamera {
            }
            // Parent: C_BaseEntity
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_iszStackName (string_t)
            // NetworkVarNames: m_iszOperatorName (string_t)
            // NetworkVarNames: m_iszOpvarName (string_t)
            // NetworkVarNames: m_iOpvarIndex (int)
            // NetworkVarNames: m_bUseAutoCompare (bool)
            pub mod C_SoundOpvarSetPointBase {
                pub const m_iszStackName: usize = 0x568; // 
                pub const m_iszOperatorName: usize = 0x570; // 
                pub const m_iszOpvarName: usize = 0x578; // 
                pub const m_iOpvarIndex: usize = 0x580; // 
                pub const m_bUseAutoCompare: usize = 0x584; // 
            }
            // Parent: C_Breakable
            // Field count: 0
            pub mod C_PhysBox {
            }
            // Parent: CPlayerPawnComponent
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_vecSellbackPurchaseEntries (SellbackPurchaseEntry_t)
            pub mod CCSPlayer_BuyServices {
                pub const m_vecSellbackPurchaseEntries: usize = 0x40; // 
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CPathSimpleAPI {
            }
            // Parent: C_BaseCSGrenade
            // Field count: 0
            pub mod C_SensorGrenade {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_fFireTime (GameTime_t)
            pub mod C_WeaponTaser {
                pub const m_fFireTime: usize = 0x1B30; // 
                pub const m_nLastAttackTick: usize = 0x1B34; // 
            }
            // Parent: C_CSWeaponBase
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_bPlayingUninterruptableAct (bool)
            // NetworkVarNames: m_nUninterruptableActivity (PlayerAnimEvent_t)
            pub mod C_Fists {
                pub const m_bPlayingUninterruptableAct: usize = 0x1B10; // 
                pub const m_nUninterruptableActivity: usize = 0x1B14; // 
            }
            // Parent: C_PhysicsProp
            // Field count: 0
            pub mod C_PhysicsPropMultiplayer {
            }
            // Parent: CInfoDynamicShadowHint
            // Field count: 2
            pub mod CInfoDynamicShadowHintBox {
                pub const m_vBoxMins: usize = 0x580; // 
                pub const m_vBoxMaxs: usize = 0x58C; // 
            }
            // Parent: C_SoundOpvarSetPointEntity
            // Field count: 0
            pub mod C_SoundOpvarSetPathCornerEntity {
            }
            // Parent: C_BaseEntity
            // Field count: 18
            //
            // Metadata:
            // NetworkVarNames: m_flEndDistance (float)
            // NetworkVarNames: m_flStartDistance (float)
            // NetworkVarNames: m_flFogFalloffExponent (float)
            // NetworkVarNames: m_bHeightFogEnabled (bool)
            // NetworkVarNames: m_flFogHeightWidth (float)
            // NetworkVarNames: m_flFogHeightEnd (float)
            // NetworkVarNames: m_flFogHeightStart (float)
            // NetworkVarNames: m_flFogHeightExponent (float)
            // NetworkVarNames: m_flLODBias (float)
            // NetworkVarNames: m_bActive (bool)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_flFogMaxOpacity (float)
            // NetworkVarNames: m_nCubemapSourceType (int)
            // NetworkVarNames: m_hSkyMaterial (HMaterialStrong)
            // NetworkVarNames: m_iszSkyEntity (string_t)
            // NetworkVarNames: m_hFogCubemapTexture (HRenderTextureStrong)
            // NetworkVarNames: m_bHasHeightFogEnd (bool)
            pub mod C_EnvCubemapFog {
                pub const m_flEndDistance: usize = 0x568; // 
                pub const m_flStartDistance: usize = 0x56C; // 
                pub const m_flFogFalloffExponent: usize = 0x570; // 
                pub const m_bHeightFogEnabled: usize = 0x574; // 
                pub const m_flFogHeightWidth: usize = 0x578; // 
                pub const m_flFogHeightEnd: usize = 0x57C; // 
                pub const m_flFogHeightStart: usize = 0x580; // 
                pub const m_flFogHeightExponent: usize = 0x584; // 
                pub const m_flLODBias: usize = 0x588; // 
                pub const m_bActive: usize = 0x58C; // 
                pub const m_bStartDisabled: usize = 0x58D; // 
                pub const m_flFogMaxOpacity: usize = 0x590; // 
                pub const m_nCubemapSourceType: usize = 0x594; // 
                pub const m_hSkyMaterial: usize = 0x598; // 
                pub const m_iszSkyEntity: usize = 0x5A0; // 
                pub const m_hFogCubemapTexture: usize = 0x5A8; // 
                pub const m_bHasHeightFogEnd: usize = 0x5B0; // 
                pub const m_bFirstTime: usize = 0x5B1; // 
            }
            // Parent: C_CSWeaponBase
            // Field count: 0
            pub mod C_WeaponSawedoff {
            }
            // Parent: None
            // Field count: 17
            //
            // Metadata:
            // NetworkVarNames: m_collisionAttribute (VPhysicsCollisionAttribute_t)
            // NetworkVarNames: m_vecMins (Vector)
            // NetworkVarNames: m_vecMaxs (Vector)
            // NetworkVarNames: m_usSolidFlags (uint8)
            // NetworkVarNames: m_nSolidType (SolidType_t)
            // NetworkVarNames: m_triggerBloat (uint8)
            // NetworkVarNames: m_nSurroundType (SurroundingBoundsType_t)
            // NetworkVarNames: m_CollisionGroup (uint8)
            // NetworkVarNames: m_nEnablePhysics (uint8)
            // NetworkVarNames: m_vecSpecifiedSurroundingMins (Vector)
            // NetworkVarNames: m_vecSpecifiedSurroundingMaxs (Vector)
            // NetworkVarNames: m_vCapsuleCenter1 (Vector)
            // NetworkVarNames: m_vCapsuleCenter2 (Vector)
            // NetworkVarNames: m_flCapsuleRadius (float)
            pub mod CCollisionProperty {
                pub const m_collisionAttribute: usize = 0x10; // 
                pub const m_vecMins: usize = 0x40; // 
                pub const m_vecMaxs: usize = 0x4C; // 
                pub const m_usSolidFlags: usize = 0x5A; // 
                pub const m_nSolidType: usize = 0x5B; // 
                pub const m_triggerBloat: usize = 0x5C; // 
                pub const m_nSurroundType: usize = 0x5D; // 
                pub const m_CollisionGroup: usize = 0x5E; // 
                pub const m_nEnablePhysics: usize = 0x5F; // 
                pub const m_flBoundingRadius: usize = 0x60; // 
                pub const m_vecSpecifiedSurroundingMins: usize = 0x64; // 
                pub const m_vecSpecifiedSurroundingMaxs: usize = 0x70; // 
                pub const m_vecSurroundingMaxs: usize = 0x7C; // 
                pub const m_vecSurroundingMins: usize = 0x88; // 
                pub const m_vCapsuleCenter1: usize = 0x94; // 
                pub const m_vCapsuleCenter2: usize = 0xA0; // 
                pub const m_flCapsuleRadius: usize = 0xAC; // 
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_hSequence (HSequence)
            // NetworkVarNames: m_flPrevCycle (float32)
            // NetworkVarNames: m_flCycle (float32)
            pub mod CNetworkedSequenceOperation {
                pub const m_hSequence: usize = 0x8; // 
                pub const m_flPrevCycle: usize = 0xC; // 
                pub const m_flCycle: usize = 0x10; // 
                pub const m_flWeight: usize = 0x14; // 
                pub const m_bSequenceChangeNetworked: usize = 0x1C; // 
                pub const m_bDiscontinuity: usize = 0x1D; // 
                pub const m_flPrevCycleFromDiscontinuity: usize = 0x20; // 
                pub const m_flPrevCycleForAnimEventDetection: usize = 0x24; // 
            }
            // Parent: C_ModelPointEntity
            // Field count: 16
            //
            // Metadata:
            // NetworkVarNames: m_nUniqueID (int)
            // NetworkVarNames: m_unAccountID (uint32)
            // NetworkVarNames: m_unTraceID (uint32)
            // NetworkVarNames: m_rtGcTime (uint32)
            // NetworkVarNames: m_vecEndPos (Vector)
            // NetworkVarNames: m_vecStart (Vector)
            // NetworkVarNames: m_vecLeft (Vector)
            // NetworkVarNames: m_vecNormal (Vector)
            // NetworkVarNames: m_nPlayer (int)
            // NetworkVarNames: m_nEntity (int)
            // NetworkVarNames: m_nHitbox (int)
            // NetworkVarNames: m_flCreationTime (float)
            // NetworkVarNames: m_nTintID (int)
            // NetworkVarNames: m_nVersion (uint8)
            // NetworkVarNames: m_ubSignature (uint8)
            pub mod C_PlayerSprayDecal {
                pub const m_nUniqueID: usize = 0xD28; // 
                pub const m_unAccountID: usize = 0xD2C; // 
                pub const m_unTraceID: usize = 0xD30; // 
                pub const m_rtGcTime: usize = 0xD34; // 
                pub const m_vecEndPos: usize = 0xD38; // 
                pub const m_vecStart: usize = 0xD44; // 
                pub const m_vecLeft: usize = 0xD50; // 
                pub const m_vecNormal: usize = 0xD5C; // 
                pub const m_nPlayer: usize = 0xD68; // 
                pub const m_nEntity: usize = 0xD6C; // 
                pub const m_nHitbox: usize = 0xD70; // 
                pub const m_flCreationTime: usize = 0xD74; // 
                pub const m_nTintID: usize = 0xD78; // 
                pub const m_nVersion: usize = 0xD7C; // 
                pub const m_ubSignature: usize = 0xD7D; // 
                pub const m_SprayRenderHelper: usize = 0xE08; // 
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_ID (CUtlStringToken)
            // NetworkVarNames: m_Values (Vector4D)
            pub mod EntityRenderAttribute_t {
                pub const m_ID: usize = 0x30; // 
                pub const m_Values: usize = 0x34; // 
            }
            // Parent: C_PhysicsProp
            // Field count: 1
            //
            // Metadata:
            // MNetworkExcludeByName
            // NetworkVarNames: m_ShardDesc (shard_model_desc_t)
            pub mod C_ShatterGlassShardPhysics {
                pub const m_ShardDesc: usize = 0x1120; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponElite {
            }
            // Parent: C_BaseModelEntity
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_CLightComponent (CLightComponent::Storage_t)
            pub mod C_LightEntity {
                pub const m_CLightComponent: usize = 0xD28; // 
            }
            // Parent: C_BaseEntity
            // Field count: 6
            //
            // Metadata:
            // NetworkVarNames: m_flVisibilityStrength (float)
            // NetworkVarNames: m_flFogDistanceMultiplier (float)
            // NetworkVarNames: m_flFogMaxDensityMultiplier (float)
            // NetworkVarNames: m_flFadeTime (float)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_bIsEnabled (bool)
            pub mod C_PlayerVisibility {
                pub const m_flVisibilityStrength: usize = 0x568; // 
                pub const m_flFogDistanceMultiplier: usize = 0x56C; // 
                pub const m_flFogMaxDensityMultiplier: usize = 0x570; // 
                pub const m_flFadeTime: usize = 0x574; // 
                pub const m_bStartDisabled: usize = 0x578; // 
                pub const m_bIsEnabled: usize = 0x579; // 
            }
            // Parent: None
            // Field count: 25
            //
            // Metadata:
            // NetworkVarNames: dirPrimary (Vector)
            // NetworkVarNames: colorPrimary (Color)
            // NetworkVarNames: colorSecondary (Color)
            // NetworkVarNames: colorPrimaryLerpTo (Color)
            // NetworkVarNames: colorSecondaryLerpTo (Color)
            // NetworkVarNames: start (float32)
            // NetworkVarNames: end (float32)
            // NetworkVarNames: farz (float32)
            // NetworkVarNames: maxdensity (float32)
            // NetworkVarNames: exponent (float32)
            // NetworkVarNames: HDRColorScale (float32)
            // NetworkVarNames: skyboxFogFactor (float32)
            // NetworkVarNames: skyboxFogFactorLerpTo (float32)
            // NetworkVarNames: startLerpTo (float32)
            // NetworkVarNames: endLerpTo (float32)
            // NetworkVarNames: maxdensityLerpTo (float32)
            // NetworkVarNames: lerptime (GameTime_t)
            // NetworkVarNames: duration (float32)
            // NetworkVarNames: blendtobackground (float32)
            // NetworkVarNames: scattering (float32)
            // NetworkVarNames: locallightscale (float32)
            // NetworkVarNames: enable (bool)
            // NetworkVarNames: blend (bool)
            // NetworkVarNames: m_bNoReflectionFog (bool)
            pub mod fogparams_t {
                pub const dirPrimary: usize = 0x8; // 
                pub const colorPrimary: usize = 0x14; // 
                pub const colorSecondary: usize = 0x18; // 
                pub const colorPrimaryLerpTo: usize = 0x1C; // 
                pub const colorSecondaryLerpTo: usize = 0x20; // 
                pub const start: usize = 0x24; // 
                pub const end: usize = 0x28; // 
                pub const farz: usize = 0x2C; // 
                pub const maxdensity: usize = 0x30; // 
                pub const exponent: usize = 0x34; // 
                pub const HDRColorScale: usize = 0x38; // 
                pub const skyboxFogFactor: usize = 0x3C; // 
                pub const skyboxFogFactorLerpTo: usize = 0x40; // 
                pub const startLerpTo: usize = 0x44; // 
                pub const endLerpTo: usize = 0x48; // 
                pub const maxdensityLerpTo: usize = 0x4C; // 
                pub const lerptime: usize = 0x50; // 
                pub const duration: usize = 0x54; // 
                pub const blendtobackground: usize = 0x58; // 
                pub const scattering: usize = 0x5C; // 
                pub const locallightscale: usize = 0x60; // 
                pub const enable: usize = 0x64; // 
                pub const blend: usize = 0x65; // 
                pub const m_bNoReflectionFog: usize = 0x66; // 
                pub const m_bPadding: usize = 0x67; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 19
            //
            // Metadata:
            // NetworkVarNames: m_flexWeight (float32)
            // NetworkVarNames: m_blinktoggle (bool)
            pub mod C_BaseFlex {
                pub const m_flexWeight: usize = 0xF98; // C_NetworkUtlVectorBase<float32>
                pub const m_vLookTargetPosition: usize = 0xFB0; // 
                pub const m_blinktoggle: usize = 0xFC8; // 
                pub const m_nLastFlexUpdateFrameCount: usize = 0x1028; // 
                pub const m_CachedViewTarget: usize = 0x102C; // 
                pub const m_nNextSceneEventId: usize = 0x1038; // 
                pub const m_iBlink: usize = 0x103C; // 
                pub const m_blinktime: usize = 0x1040; // 
                pub const m_prevblinktoggle: usize = 0x1044; // 
                pub const m_iJawOpen: usize = 0x1048; // 
                pub const m_flJawOpenAmount: usize = 0x104C; // 
                pub const m_flBlinkAmount: usize = 0x1050; // 
                pub const m_iMouthAttachment: usize = 0x1054; // 
                pub const m_iEyeAttachment: usize = 0x1055; // 
                pub const m_bResetFlexWeightsOnModelChange: usize = 0x1056; // 
                pub const m_nEyeOcclusionRendererBone: usize = 0x1070; // 
                pub const m_mEyeOcclusionRendererCameraToBoneTransform: usize = 0x1074; // 
                pub const m_vEyeOcclusionRendererHalfExtent: usize = 0x10A4; // 
                pub const m_PhonemeClasses: usize = 0x10C0; // C_BaseFlex::Emphasized_Phoneme[3]
            }
            // Parent: C_BaseModelEntity
            // Field count: 12
            //
            // Metadata:
            // NetworkVarNames: m_hSkyMaterial (HMaterialStrong)
            // NetworkVarNames: m_hSkyMaterialLightingOnly (HMaterialStrong)
            // NetworkVarNames: m_bStartDisabled (bool)
            // NetworkVarNames: m_vTintColor (Color)
            // NetworkVarNames: m_vTintColorLightingOnly (Color)
            // NetworkVarNames: m_flBrightnessScale (float)
            // NetworkVarNames: m_nFogType (int)
            // NetworkVarNames: m_flFogMinStart (float)
            // NetworkVarNames: m_flFogMinEnd (float)
            // NetworkVarNames: m_flFogMaxStart (float)
            // NetworkVarNames: m_flFogMaxEnd (float)
            // NetworkVarNames: m_bEnabled (bool)
            pub mod C_EnvSky {
                pub const m_hSkyMaterial: usize = 0xD28; // 
                pub const m_hSkyMaterialLightingOnly: usize = 0xD30; // 
                pub const m_bStartDisabled: usize = 0xD38; // 
                pub const m_vTintColor: usize = 0xD39; // 
                pub const m_vTintColorLightingOnly: usize = 0xD3D; // 
                pub const m_flBrightnessScale: usize = 0xD44; // 
                pub const m_nFogType: usize = 0xD48; // 
                pub const m_flFogMinStart: usize = 0xD4C; // 
                pub const m_flFogMinEnd: usize = 0xD50; // 
                pub const m_flFogMaxStart: usize = 0xD54; // 
                pub const m_flFogMaxEnd: usize = 0xD58; // 
                pub const m_bEnabled: usize = 0xD5C; // 
            }
            // Parent: C_PointEntity
            // Field count: 0
            pub mod CInfoTarget {
            }
            // Parent: C_CSGO_TeamIntroCharacterPosition
            // Field count: 0
            pub mod C_CSGO_TeamIntroCounterTerroristPosition {
            }
            // Parent: C_BreakableProp
            // Field count: 23
            //
            // Metadata:
            // NetworkVarNames: m_bUseHitboxesForRenderBox (bool)
            // NetworkVarNames: m_bUseAnimGraph (bool)
            pub mod C_DynamicProp {
                pub const m_bUseHitboxesForRenderBox: usize = 0x1110; // 
                pub const m_bUseAnimGraph: usize = 0x1111; // 
                pub const m_pOutputAnimBegun: usize = 0x1118; // 
                pub const m_pOutputAnimOver: usize = 0x1140; // 
                pub const m_pOutputAnimLoopCycleOver: usize = 0x1168; // 
                pub const m_OnAnimReachedStart: usize = 0x1190; // 
                pub const m_OnAnimReachedEnd: usize = 0x11B8; // 
                pub const m_iszIdleAnim: usize = 0x11E0; // 
                pub const m_nIdleAnimLoopMode: usize = 0x11E8; // 
                pub const m_bRandomizeCycle: usize = 0x11EC; // 
                pub const m_bStartDisabled: usize = 0x11ED; // 
                pub const m_bFiredStartEndOutput: usize = 0x11EE; // 
                pub const m_bForceNpcExclude: usize = 0x11EF; // 
                pub const m_bCreateNonSolid: usize = 0x11F0; // 
                pub const m_bIsOverrideProp: usize = 0x11F1; // 
                pub const m_iInitialGlowState: usize = 0x11F4; // 
                pub const m_nGlowRange: usize = 0x11F8; // 
                pub const m_nGlowRangeMin: usize = 0x11FC; // 
                pub const m_glowColor: usize = 0x1200; // 
                pub const m_nGlowTeam: usize = 0x1204; // 
                pub const m_iCachedFrameCount: usize = 0x1208; // 
                pub const m_vecCachedRenderMins: usize = 0x120C; // 
                pub const m_vecCachedRenderMaxs: usize = 0x1218; // 
            }
            // Parent: CEntityComponent
            // Field count: 10
            pub mod CPropDataComponent {
                pub const m_flDmgModBullet: usize = 0x10; // 
                pub const m_flDmgModClub: usize = 0x14; // 
                pub const m_flDmgModExplosive: usize = 0x18; // 
                pub const m_flDmgModFire: usize = 0x1C; // 
                pub const m_iszPhysicsDamageTableName: usize = 0x20; // 
                pub const m_iszBasePropData: usize = 0x28; // 
                pub const m_nInteractions: usize = 0x30; // 
                pub const m_bSpawnMotionDisabled: usize = 0x34; // 
                pub const m_nDisableTakePhysicsDamageSpawnFlag: usize = 0x38; // 
                pub const m_nMotionDisabledSpawnFlag: usize = 0x3C; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponSCAR20 {
            }
            // Parent: C_BaseCSGrenade
            // Field count: 0
            pub mod C_DecoyGrenade {
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPulseProvideFeatureTag
            // MPulseLibraryBindings
            pub mod CCSPointScriptExtensions_player {
            }
            // Parent: C_CS2WeaponModuleBase
            // Field count: 2
            pub mod C_KeychainModule {
                pub const m_nKeychainDefID: usize = 0xF90; // 
                pub const m_nKeychainSeed: usize = 0xF94; // 
            }
            // Parent: C_BaseEntity
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_iszStackName (string_t)
            // NetworkVarNames: m_iszOperatorName (string_t)
            // NetworkVarNames: m_iszOpvarName (string_t)
            // NetworkVarNames: m_vDistanceInnerMins (Vector)
            // NetworkVarNames: m_vDistanceInnerMaxs (Vector)
            // NetworkVarNames: m_vDistanceOuterMins (Vector)
            // NetworkVarNames: m_vDistanceOuterMaxs (Vector)
            // NetworkVarNames: m_nAABBDirection (int)
            pub mod CCitadelSoundOpvarSetOBB {
                pub const m_iszStackName: usize = 0x580; // 
                pub const m_iszOperatorName: usize = 0x588; // 
                pub const m_iszOpvarName: usize = 0x590; // 
                pub const m_vDistanceInnerMins: usize = 0x598; // 
                pub const m_vDistanceInnerMaxs: usize = 0x5A4; // 
                pub const m_vDistanceOuterMins: usize = 0x5B0; // 
                pub const m_vDistanceOuterMaxs: usize = 0x5BC; // 
                pub const m_nAABBDirection: usize = 0x5C8; // 
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_unDefIdx (item_definition_index_t)
            // NetworkVarNames: m_nCost (int)
            // NetworkVarNames: m_nPrevArmor (int)
            // NetworkVarNames: m_bPrevHelmet (bool)
            // NetworkVarNames: m_hItem (CEntityHandle)
            pub mod SellbackPurchaseEntry_t {
                pub const m_unDefIdx: usize = 0x30; // 
                pub const m_nCost: usize = 0x34; // 
                pub const m_nPrevArmor: usize = 0x38; // 
                pub const m_bPrevHelmet: usize = 0x3C; // 
                pub const m_hItem: usize = 0x40; // 
            }
            // Parent: C_GameRules
            // Field count: 0
            pub mod C_SingleplayRules {
            }
            // Parent: CBaseAnimGraph
            // Field count: 25
            pub mod C_LocalTempEntity {
                pub const flags: usize = 0xF88; // 
                pub const die: usize = 0xF8C; // 
                pub const m_flFrameMax: usize = 0xF90; // 
                pub const x: usize = 0xF94; // 
                pub const y: usize = 0xF98; // 
                pub const fadeSpeed: usize = 0xF9C; // 
                pub const bounceFactor: usize = 0xFA0; // 
                pub const hitSound: usize = 0xFA4; // 
                pub const priority: usize = 0xFA8; // 
                pub const tentOffset: usize = 0xFAC; // 
                pub const m_vecTempEntAngVelocity: usize = 0xFB8; // 
                pub const tempent_renderamt: usize = 0xFC4; // 
                pub const m_vecNormal: usize = 0xFC8; // 
                pub const m_flSpriteScale: usize = 0xFD4; // 
                pub const m_nFlickerFrame: usize = 0xFD8; // 
                pub const m_flFrameRate: usize = 0xFDC; // 
                pub const m_flFrame: usize = 0xFE0; // 
                pub const m_pszImpactEffect: usize = 0xFE8; // 
                pub const m_pszParticleEffect: usize = 0xFF0; // 
                pub const m_bParticleCollision: usize = 0xFF8; // 
                pub const m_iLastCollisionFrame: usize = 0xFFC; // 
                pub const m_vLastCollisionOrigin: usize = 0x1000; // 
                pub const m_vecTempEntVelocity: usize = 0x100C; // 
                pub const m_vecPrevAbsOrigin: usize = 0x1018; // 
                pub const m_vecTempEntAcceleration: usize = 0x1024; // 
            }
            // Parent: None
            // Field count: 2
            pub mod C_EnvWindShared__WindAveEvent_t {
                pub const m_flStartWindSpeed: usize = 0x0; // 
                pub const m_flAveWindSpeed: usize = 0x4; // 
            }
            // Parent: C_BaseCSGrenadeProjectile
            // Field count: 1
            //
            // Metadata:
            // NetworkVarNames: m_bIsIncGrenade (bool)
            pub mod C_MolotovProjectile {
                pub const m_bIsIncGrenade: usize = 0x1208; // 
            }
            // Parent: C_LightDirectionalEntity
            // Field count: 0
            pub mod C_LightEnvironmentEntity {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponUMP45 {
            }
            // Parent: C_PointEntity
            // Field count: 9
            //
            // Metadata:
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_nResolutionX (int)
            // NetworkVarNames: m_nResolutionY (int)
            // NetworkVarNames: m_szLayoutFileName (string_t)
            // NetworkVarNames: m_RenderAttrName (string_t)
            // NetworkVarNames: m_TargetEntities (CHandle<C_BaseModelEntity>)
            // NetworkVarNames: m_nTargetChangeCount (int)
            // NetworkVarNames: m_vecCSSClasses (string_t)
            pub mod CInfoOffscreenPanoramaTexture {
                pub const m_bDisabled: usize = 0x568; // 
                pub const m_nResolutionX: usize = 0x56C; // 
                pub const m_nResolutionY: usize = 0x570; // 
                pub const m_szLayoutFileName: usize = 0x578; // 
                pub const m_RenderAttrName: usize = 0x580; // 
                pub const m_TargetEntities: usize = 0x588; // 
                pub const m_nTargetChangeCount: usize = 0x5A0; // 
                pub const m_vecCSSClasses: usize = 0x5A8; // 
                pub const m_bCheckCSSClasses: usize = 0x720; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 7
            //
            // Metadata:
            // NetworkVarNames: m_Flags (uint8)
            // NetworkVarNames: m_LightStyle (uint8)
            // NetworkVarNames: m_Radius (float32)
            // NetworkVarNames: m_Exponent (int32)
            // NetworkVarNames: m_InnerAngle (float32)
            // NetworkVarNames: m_OuterAngle (float32)
            // NetworkVarNames: m_SpotRadius (float32)
            pub mod C_DynamicLight {
                pub const m_Flags: usize = 0xD28; // 
                pub const m_LightStyle: usize = 0xD29; // 
                pub const m_Radius: usize = 0xD2C; // 
                pub const m_Exponent: usize = 0xD30; // 
                pub const m_InnerAngle: usize = 0xD34; // 
                pub const m_OuterAngle: usize = 0xD38; // 
                pub const m_SpotRadius: usize = 0xD3C; // 
            }
            // Parent: C_BasePlayerWeapon
            // Field count: 72
            //
            // Metadata:
            // MNetworkExcludeByName
            // NetworkVarNames: m_flFireSequenceStartTime (float)
            // NetworkVarNames: m_nFireSequenceStartTimeChange (int)
            // NetworkVarNames: m_ePlayerFireEvent (PlayerAnimEvent_t)
            // NetworkVarNames: m_ePlayerFireEventAttackType (WeaponAttackType_t)
            // NetworkVarNames: m_iState (CSWeaponState_t)
            // NetworkVarNames: m_nViewModelIndex (uint32)
            // NetworkVarNames: m_flTimeWeaponIdle (GameTime_t)
            // NetworkVarNames: m_weaponMode (CSWeaponMode)
            // NetworkVarNames: m_fAccuracyPenalty (float)
            // NetworkVarNames: m_iRecoilIndex (int)
            // NetworkVarNames: m_flRecoilIndex (float)
            // NetworkVarNames: m_bBurstMode (bool)
            // NetworkVarNames: m_nPostponeFireReadyTicks (GameTick_t)
            // NetworkVarNames: m_flPostponeFireReadyFrac (float)
            // NetworkVarNames: m_bInReload (bool)
            // NetworkVarNames: m_bReloadVisuallyComplete (bool)
            // NetworkVarNames: m_flDroppedAtTime (GameTime_t)
            // NetworkVarNames: m_bIsHauledBack (bool)
            // NetworkVarNames: m_bSilencerOn (bool)
            // NetworkVarNames: m_flTimeSilencerSwitchComplete (GameTime_t)
            // NetworkVarNames: m_iOriginalTeamNumber (int)
            // NetworkVarNames: m_iMostRecentTeamNumber (int)
            // NetworkVarNames: m_bDroppedNearBuyZone (bool)
            // NetworkVarNames: m_hPrevOwner (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_nDropTick (GameTick_t)
            // NetworkVarNames: m_fLastShotTime (GameTime_t)
            // NetworkVarNames: m_iIronSightMode (int)
            // NetworkVarNames: m_iNumEmptyAttacks (int)
            pub mod C_CSWeaponBase {
                pub const m_flFireSequenceStartTime: usize = 0x16DC; // 
                pub const m_nFireSequenceStartTimeChange: usize = 0x16E0; // 
                pub const m_nFireSequenceStartTimeAck: usize = 0x16E4; // 
                pub const m_ePlayerFireEvent: usize = 0x16E8; // 
                pub const m_ePlayerFireEventAttackType: usize = 0x16EC; // 
                pub const m_seqIdle: usize = 0x16F0; // 
                pub const m_seqFirePrimary: usize = 0x16F4; // 
                pub const m_seqFireSecondary: usize = 0x16F8; // 
                pub const m_thirdPersonFireSequences: usize = 0x1700; // 
                pub const m_hCurrentThirdPersonSequence: usize = 0x1718; // 
                pub const m_nSilencerBoneIndex: usize = 0x171C; // 
                pub const m_thirdPersonSequences: usize = 0x1720; // 
                pub const m_ClientPreviousWeaponState: usize = 0x1758; // 
                pub const m_iState: usize = 0x175C; // 
                pub const m_flCrosshairDistance: usize = 0x1760; // 
                pub const m_iAmmoLastCheck: usize = 0x1764; // 
                pub const m_iAlpha: usize = 0x1768; // 
                pub const m_iScopeTextureID: usize = 0x176C; // 
                pub const m_iCrosshairTextureID: usize = 0x1770; // 
                pub const m_flGunAccuracyPositionDeprecated: usize = 0x1774; // 
                pub const m_nLastEmptySoundCmdNum: usize = 0x1778; // 
                pub const m_nViewModelIndex: usize = 0x177C; // 
                pub const m_bReloadsWithClips: usize = 0x1780; // 
                pub const m_flTimeWeaponIdle: usize = 0x1784; // 
                pub const m_bFireOnEmpty: usize = 0x1788; // 
                pub const m_OnPlayerPickup: usize = 0x1790; // 
                pub const m_weaponMode: usize = 0x17B8; // 
                pub const m_flTurningInaccuracyDelta: usize = 0x17BC; // 
                pub const m_vecTurningInaccuracyEyeDirLast: usize = 0x17C0; // 
                pub const m_flTurningInaccuracy: usize = 0x17CC; // 
                pub const m_fAccuracyPenalty: usize = 0x17D0; // 
                pub const m_flLastAccuracyUpdateTime: usize = 0x17D4; // 
                pub const m_fAccuracySmoothedForZoom: usize = 0x17D8; // 
                pub const m_fScopeZoomEndTime: usize = 0x17DC; // 
                pub const m_iRecoilIndex: usize = 0x17E0; // 
                pub const m_flRecoilIndex: usize = 0x17E4; // 
                pub const m_bBurstMode: usize = 0x17E8; // 
                pub const m_flLastBurstModeChangeTime: usize = 0x17EC; // 
                pub const m_nPostponeFireReadyTicks: usize = 0x17F0; // 
                pub const m_flPostponeFireReadyFrac: usize = 0x17F4; // 
                pub const m_bInReload: usize = 0x17F8; // 
                pub const m_bReloadVisuallyComplete: usize = 0x17F9; // 
                pub const m_flDroppedAtTime: usize = 0x17FC; // 
                pub const m_bIsHauledBack: usize = 0x1800; // 
                pub const m_bSilencerOn: usize = 0x1801; // 
                pub const m_flTimeSilencerSwitchComplete: usize = 0x1804; // 
                pub const m_iOriginalTeamNumber: usize = 0x1808; // 
                pub const m_iMostRecentTeamNumber: usize = 0x180C; // 
                pub const m_bDroppedNearBuyZone: usize = 0x1810; // 
                pub const m_flNextAttackRenderTimeOffset: usize = 0x1814; // 
                pub const m_bClearWeaponIdentifyingUGC: usize = 0x18B0; // 
                pub const m_bVisualsDataSet: usize = 0x18B1; // 
                pub const m_bOldFirstPersonSpectatedState: usize = 0x18B2; // 
                pub const m_bUIWeapon: usize = 0x18B3; // 
                pub const m_nCustomEconReloadEventId: usize = 0x18B4; // 
                pub const m_hPrevOwner: usize = 0x18C0; // 
                pub const m_nDropTick: usize = 0x18C4; // 
                pub const m_donated: usize = 0x18E4; // 
                pub const m_fLastShotTime: usize = 0x18E8; // 
                pub const m_bWasOwnedByCT: usize = 0x18EC; // 
                pub const m_bWasOwnedByTerrorist: usize = 0x18ED; // 
                pub const m_gunHeat: usize = 0x18F0; // 
                pub const m_smokeAttachments: usize = 0x18F4; // 
                pub const m_lastSmokeTime: usize = 0x18F8; // 
                pub const m_flNextClientFireBulletTime: usize = 0x18FC; // 
                pub const m_flNextClientFireBulletTime_Repredict: usize = 0x1900; // 
                pub const m_IronSightController: usize = 0x19E0; // 
                pub const m_iIronSightMode: usize = 0x1A90; // 
                pub const m_flLastLOSTraceFailureTime: usize = 0x1AA0; // 
                pub const m_iNumEmptyAttacks: usize = 0x1AA4; // 
                pub const m_flLastMagDropRequestTime: usize = 0x1B00; // 
                pub const m_flWatTickOffset: usize = 0x1B04; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod C_TriggerVolume {
            }
            // Parent: C_FuncBrush
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_EffectName (string_t)
            // NetworkVarNames: m_bState (bool)
            pub mod C_FuncElectrifiedVolume {
                pub const m_nAmbientEffect: usize = 0xD28; // 
                pub const m_EffectName: usize = 0xD30; // 
                pub const m_bState: usize = 0xD38; // 
            }
            // Parent: CPlayer_WeaponServices
            // Field count: 5
            //
            // Metadata:
            // NetworkVarNames: m_flNextAttack (GameTime_t)
            // NetworkVarNames: m_bIsLookingAtWeapon (bool)
            // NetworkVarNames: m_bIsHoldingLookAtWeapon (bool)
            pub mod CCSPlayer_WeaponServices {
                pub const m_flNextAttack: usize = 0xB8; // 
                pub const m_bIsLookingAtWeapon: usize = 0xBC; // 
                pub const m_bIsHoldingLookAtWeapon: usize = 0xBD; // 
                pub const m_nOldShootPositionHistoryCount: usize = 0xC0; // 
                pub const m_nOldInputHistoryCount: usize = 0x458; // 
            }
            // Parent: C_BaseEntity
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_flFadeStartDist (float32)
            // NetworkVarNames: m_flFadeEndDist (float32)
            pub mod C_EnvDetailController {
                pub const m_flFadeStartDist: usize = 0x568; // 
                pub const m_flFadeEndDist: usize = 0x56C; // 
            }
            // Parent: CEntityInstance
            // Field count: 82
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // NetworkVarNames: m_CBodyComponent (CBodyComponent::Storage_t)
            // NetworkVarNames: m_iMaxHealth (int32)
            // NetworkVarNames: m_iHealth (int32)
            // NetworkVarNames: m_lifeState (uint8)
            // NetworkVarNames: m_bTakesDamage (bool)
            // NetworkVarNames: m_nTakeDamageFlags (TakeDamageFlags_t)
            // NetworkVarNames: m_nPlatformType (EntityPlatformTypes_t)
            // NetworkVarNames: m_ubInterpolationFrame (uint8)
            // NetworkVarNames: m_nSubclassID (EntitySubclassID_t)
            // NetworkVarNames: m_flAnimTime (float32)
            // NetworkVarNames: m_flSimulationTime (float32)
            // NetworkVarNames: m_flCreateTime (GameTime_t)
            // NetworkVarNames: m_flSpeed (float)
            // NetworkVarNames: m_bClientSideRagdoll (bool)
            // NetworkVarNames: m_iTeamNum (uint8)
            // NetworkVarNames: m_spawnflags (uint32)
            // NetworkVarNames: m_nNextThinkTick (GameTick_t)
            // NetworkVarNames: m_fFlags (uint32)
            // NetworkVarNames: m_vecBaseVelocity (Vector)
            // NetworkVarNames: m_hEffectEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_hOwnerEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_MoveCollide (MoveCollide_t)
            // NetworkVarNames: m_MoveType (MoveType_t)
            // NetworkVarNames: m_flWaterLevel (float32)
            // NetworkVarNames: m_fEffects (uint32)
            // NetworkVarNames: m_hGroundEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_nGroundBodyIndex (int)
            // NetworkVarNames: m_flFriction (float32)
            // NetworkVarNames: m_flElasticity (float32)
            // NetworkVarNames: m_flGravityScale (float32)
            // NetworkVarNames: m_flTimeScale (float32)
            // NetworkVarNames: m_bAnimatedEveryTick (bool)
            // NetworkVarNames: m_flNavIgnoreUntilTime (GameTime_t)
            // NetworkVarNames: m_nBloodType (BloodType)
            pub mod C_BaseEntity {
                pub const m_CBodyComponent: usize = 0x38; // 
                pub const m_NetworkTransmitComponent: usize = 0x40; // 
                pub const m_nLastThinkTick: usize = 0x320; // 
                pub const m_pGameSceneNode: usize = 0x328; // 
                pub const m_pRenderComponent: usize = 0x330; // 
                pub const m_pCollision: usize = 0x338; // 
                pub const m_iMaxHealth: usize = 0x340; // 
                pub const m_iHealth: usize = 0x344; // 
                pub const m_lifeState: usize = 0x348; // 
                pub const m_bTakesDamage: usize = 0x349; // 
                pub const m_nTakeDamageFlags: usize = 0x350; // 
                pub const m_nPlatformType: usize = 0x358; // 
                pub const m_ubInterpolationFrame: usize = 0x359; // 
                pub const m_hSceneObjectController: usize = 0x35C; // 
                pub const m_nNoInterpolationTick: usize = 0x360; // 
                pub const m_nVisibilityNoInterpolationTick: usize = 0x364; // 
                pub const m_flProxyRandomValue: usize = 0x368; // 
                pub const m_iEFlags: usize = 0x36C; // 
                pub const m_nWaterType: usize = 0x370; // 
                pub const m_bInterpolateEvenWithNoModel: usize = 0x371; // 
                pub const m_bPredictionEligible: usize = 0x372; // 
                pub const m_bApplyLayerMatchIDToModel: usize = 0x373; // 
                pub const m_tokLayerMatchID: usize = 0x374; // 
                pub const m_nSubclassID: usize = 0x378; // 
                pub const m_nSimulationTick: usize = 0x388; // 
                pub const m_iCurrentThinkContext: usize = 0x38C; // 
                pub const m_aThinkFunctions: usize = 0x390; // 
                pub const m_bDisabledContextThinks: usize = 0x3A8; // 
                pub const m_flAnimTime: usize = 0x3AC; // 
                pub const m_flSimulationTime: usize = 0x3B0; // 
                pub const m_nSceneObjectOverrideFlags: usize = 0x3B4; // 
                pub const m_bHasSuccessfullyInterpolated: usize = 0x3B5; // 
                pub const m_bHasAddedVarsToInterpolation: usize = 0x3B6; // 
                pub const m_bRenderEvenWhenNotSuccessfullyInterpolated: usize = 0x3B7; // 
                pub const m_nInterpolationLatchDirtyFlags: usize = 0x3B8; // 
                pub const m_ListEntry: usize = 0x3C0; // 
                pub const m_flCreateTime: usize = 0x3D8; // 
                pub const m_flSpeed: usize = 0x3DC; // 
                pub const m_EntClientFlags: usize = 0x3E0; // 
                pub const m_bClientSideRagdoll: usize = 0x3E2; // 
                pub const m_iTeamNum: usize = 0x3E3; // 
                pub const m_spawnflags: usize = 0x3E4; // 
                pub const m_nNextThinkTick: usize = 0x3E8; // 
                pub const m_fFlags: usize = 0x3EC; // 
                pub const m_vecAbsVelocity: usize = 0x3F0; // 
                pub const m_vecVelocity: usize = 0x400; // 
                pub const m_vecBaseVelocity: usize = 0x430; // 
                pub const m_hEffectEntity: usize = 0x43C; // 
                pub const m_hOwnerEntity: usize = 0x440; // 
                pub const m_MoveCollide: usize = 0x444; // 
                pub const m_MoveType: usize = 0x445; // 
                pub const m_nActualMoveType: usize = 0x446; // 
                pub const m_flWaterLevel: usize = 0x448; // 
                pub const m_fEffects: usize = 0x44C; // 
                pub const m_hGroundEntity: usize = 0x450; // 
                pub const m_nGroundBodyIndex: usize = 0x454; // 
                pub const m_flFriction: usize = 0x458; // 
                pub const m_flElasticity: usize = 0x45C; // 
                pub const m_flGravityScale: usize = 0x460; // 
                pub const m_flTimeScale: usize = 0x464; // 
                pub const m_bAnimatedEveryTick: usize = 0x468; // 
                pub const m_flNavIgnoreUntilTime: usize = 0x46C; // 
                pub const m_hThink: usize = 0x470; // 
                pub const m_fBBoxVisFlags: usize = 0x480; // 
                pub const m_bPredictable: usize = 0x481; // 
                pub const m_bRenderWithViewModels: usize = 0x482; // 
                pub const m_nSplitUserPlayerPredictionSlot: usize = 0x484; // 
                pub const m_nFirstPredictableCommand: usize = 0x488; // 
                pub const m_nLastPredictableCommand: usize = 0x48C; // 
                pub const m_hOldMoveParent: usize = 0x490; // 
                pub const m_Particles: usize = 0x498; // 
                pub const m_vecPredictedScriptFloats: usize = 0x4C0; // 
                pub const m_vecPredictedScriptFloatIDs: usize = 0x4D8; // 
                pub const m_nNextScriptVarRecordID: usize = 0x508; // 
                pub const m_vecAngVelocity: usize = 0x518; // 
                pub const m_DataChangeEventRef: usize = 0x524; // 
                pub const m_dependencies: usize = 0x528; // 
                pub const m_nCreationTick: usize = 0x540; // 
                pub const m_bAnimTimeChanged: usize = 0x54D; // 
                pub const m_bSimulationTimeChanged: usize = 0x54E; // 
                pub const m_sUniqueHammerID: usize = 0x558; // 
                pub const m_nBloodType: usize = 0x560; // 
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // NetworkVarNames: m_nameStringableIndex (int32)
            pub mod CEntityIdentity {
                pub const m_nameStringableIndex: usize = 0x14; // 
                pub const m_name: usize = 0x18; // 
                pub const m_designerName: usize = 0x20; // 
                pub const m_flags: usize = 0x30; // 
                pub const m_worldGroupId: usize = 0x38; // 
                pub const m_fDataObjectTypes: usize = 0x3C; // 
                pub const m_PathIndex: usize = 0x40; // 
                pub const m_pPrev: usize = 0x58; // 
                pub const m_pNext: usize = 0x60; // 
                pub const m_pPrevByClass: usize = 0x68; // 
                pub const m_pNextByClass: usize = 0x70; // 
            }
            // Parent: C_CS2WeaponModuleBase
            // Field count: 1
            pub mod C_StattrakModule {
                pub const m_bKnife: usize = 0xF90; // 
            }
            // Parent: C_BreakableProp
            // Field count: 1
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkIncludeByName
            // NetworkVarNames: m_bAwake (bool)
            pub mod C_PhysicsProp {
                pub const m_bAwake: usize = 0x1110; // 
            }
            // Parent: CBaseProp
            // Field count: 29
            //
            // Metadata:
            // NetworkVarNames: m_CPropDataComponent (CPropDataComponent::Storage_t)
            // NetworkVarNames: m_noGhostCollision (bool)
            pub mod C_BreakableProp {
                pub const m_CPropDataComponent: usize = 0xFC8; // 
                pub const m_OnBreak: usize = 0x1008; // 
                pub const m_OnHealthChanged: usize = 0x1030; // CEntityOutputTemplate<float32>
                pub const m_OnTakeDamage: usize = 0x1058; // 
                pub const m_impactEnergyScale: usize = 0x1080; // 
                pub const m_iMinHealthDmg: usize = 0x1084; // 
                pub const m_flPressureDelay: usize = 0x1088; // 
                pub const m_flDefBurstScale: usize = 0x108C; // 
                pub const m_vDefBurstOffset: usize = 0x1090; // 
                pub const m_hBreaker: usize = 0x109C; // 
                pub const m_PerformanceMode: usize = 0x10A0; // 
                pub const m_flPreventDamageBeforeTime: usize = 0x10A4; // 
                pub const m_BreakableContentsType: usize = 0x10A8; // 
                pub const m_strBreakableContentsPropGroupOverride: usize = 0x10B0; // 
                pub const m_strBreakableContentsParticleOverride: usize = 0x10B8; // 
                pub const m_bHasBreakPiecesOrCommands: usize = 0x10C0; // 
                pub const m_explodeDamage: usize = 0x10C4; // 
                pub const m_explodeRadius: usize = 0x10C8; // 
                pub const m_explosionDelay: usize = 0x10D0; // 
                pub const m_explosionBuildupSound: usize = 0x10D8; // 
                pub const m_explosionCustomEffect: usize = 0x10E0; // 
                pub const m_explosionCustomSound: usize = 0x10E8; // 
                pub const m_explosionModifier: usize = 0x10F0; // 
                pub const m_hPhysicsAttacker: usize = 0x10F8; // 
                pub const m_flLastPhysicsInfluenceTime: usize = 0x10FC; // 
                pub const m_flDefaultFadeScale: usize = 0x1100; // 
                pub const m_hLastAttacker: usize = 0x1104; // 
                pub const m_hFlareEnt: usize = 0x1108; // 
                pub const m_noGhostCollision: usize = 0x110C; // 
            }
            // Parent: C_BaseToggle
            // Field count: 0
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkOverride
            pub mod C_FuncMoveLinear {
            }
            // Parent: C_BaseModelEntity
            // Field count: 24
            //
            // Metadata:
            // MNetworkIncludeByUserGroup
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkOverride
            // NetworkVarNames: m_flFrameRate (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            // NetworkVarNames: m_nNumBeamEnts (uint8)
            // NetworkVarNames: m_hBaseMaterial (HMaterialStrong)
            // NetworkVarNames: m_nHaloIndex (HMaterialStrong)
            // NetworkVarNames: m_nBeamType (BeamType_t)
            // NetworkVarNames: m_nBeamFlags (uint32)
            // NetworkVarNames: m_hAttachEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_nAttachIndex (AttachmentHandle_t)
            // NetworkVarNames: m_fWidth (float32)
            // NetworkVarNames: m_fEndWidth (float32)
            // NetworkVarNames: m_fFadeLength (float32)
            // NetworkVarNames: m_fHaloScale (float32)
            // NetworkVarNames: m_fAmplitude (float32)
            // NetworkVarNames: m_fStartFrame (float32)
            // NetworkVarNames: m_fSpeed (float32)
            // NetworkVarNames: m_flFrame (float32)
            // NetworkVarNames: m_nClipStyle (BeamClipStyle_t)
            // NetworkVarNames: m_bTurnedOff (bool)
            // NetworkVarNames: m_vecEndPos (Vector)
            pub mod C_Beam {
                pub const m_flFrameRate: usize = 0xD28; // 
                pub const m_flHDRColorScale: usize = 0xD2C; // 
                pub const m_flFireTime: usize = 0xD30; // 
                pub const m_flDamage: usize = 0xD34; // 
                pub const m_nNumBeamEnts: usize = 0xD38; // 
                pub const m_queryHandleHalo: usize = 0xD3C; // 
                pub const m_hBaseMaterial: usize = 0xD60; // 
                pub const m_nHaloIndex: usize = 0xD68; // 
                pub const m_nBeamType: usize = 0xD70; // 
                pub const m_nBeamFlags: usize = 0xD74; // 
                pub const m_hAttachEntity: usize = 0xD78; // 
                pub const m_nAttachIndex: usize = 0xDA0; // 
                pub const m_fWidth: usize = 0xDAC; // 
                pub const m_fEndWidth: usize = 0xDB0; // 
                pub const m_fFadeLength: usize = 0xDB4; // 
                pub const m_fHaloScale: usize = 0xDB8; // 
                pub const m_fAmplitude: usize = 0xDBC; // 
                pub const m_fStartFrame: usize = 0xDC0; // 
                pub const m_fSpeed: usize = 0xDC4; // 
                pub const m_flFrame: usize = 0xDC8; // 
                pub const m_nClipStyle: usize = 0xDCC; // 
                pub const m_bTurnedOff: usize = 0xDD0; // 
                pub const m_vecEndPos: usize = 0xDD4; // 
                pub const m_hEndEntity: usize = 0xDE0; // 
            }
            // Parent: C_BaseEntity
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_nVariant (int)
            // NetworkVarNames: m_nRandom (int)
            // NetworkVarNames: m_nOrdinal (int)
            // NetworkVarNames: m_sWeaponName (CUtlString)
            // NetworkVarNames: m_xuid (XUID)
            // NetworkVarNames: m_agentItem (CEconItemView)
            // NetworkVarNames: m_glovesItem (CEconItemView)
            // NetworkVarNames: m_weaponItem (CEconItemView)
            pub mod C_CSGO_TeamPreviewCharacterPosition {
                pub const m_nVariant: usize = 0x568; // 
                pub const m_nRandom: usize = 0x56C; // 
                pub const m_nOrdinal: usize = 0x570; // 
                pub const m_sWeaponName: usize = 0x578; // 
                pub const m_xuid: usize = 0x580; // 
                pub const m_agentItem: usize = 0x588; // 
                pub const m_glovesItem: usize = 0x9D0; // 
                pub const m_weaponItem: usize = 0xE18; // 
            }
            // Parent: C_DynamicProp
            // Field count: 0
            pub mod C_DynamicPropAlias_prop_dynamic_override {
            }
            // Parent: C_PointEntity
            // Field count: 9
            //
            // Metadata:
            // NetworkVarNames: m_iszOverlayNames (string_t)
            // NetworkVarNames: m_flOverlayTimes (float32)
            // NetworkVarNames: m_flStartTime (GameTime_t)
            // NetworkVarNames: m_iDesiredOverlay (int32)
            // NetworkVarNames: m_bIsActive (bool)
            pub mod C_EnvScreenOverlay {
                pub const m_iszOverlayNames: usize = 0x568; // 
                pub const m_flOverlayTimes: usize = 0x5B8; // 
                pub const m_flStartTime: usize = 0x5E0; // 
                pub const m_iDesiredOverlay: usize = 0x5E4; // 
                pub const m_bIsActive: usize = 0x5E8; // 
                pub const m_bWasActive: usize = 0x5E9; // 
                pub const m_iCachedDesiredOverlay: usize = 0x5EC; // 
                pub const m_iCurrentOverlay: usize = 0x5F0; // 
                pub const m_flCurrentOverlayTime: usize = 0x5F4; // 
            }
            // Parent: CBasePlayerWeaponVData
            // Field count: 92
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertySuppressBaseClassField
            // MPropertySuppressBaseClassField
            pub mod CCSWeaponBaseVData {
                pub const m_WeaponType: usize = 0x348; // 
                pub const m_WeaponCategory: usize = 0x34C; // 
                pub const m_szViewModel: usize = 0x350; // 
                pub const m_szPlayerModel: usize = 0x430; // 
                pub const m_szWorldDroppedModel: usize = 0x510; // 
                pub const m_szAimsightLensMaskModel: usize = 0x5F0; // 
                pub const m_szMagazineModel: usize = 0x6D0; // 
                pub const m_szHeatEffect: usize = 0x7B0; // 
                pub const m_szEjectBrassEffect: usize = 0x890; // 
                pub const m_szMuzzleFlashParticleAlt: usize = 0x970; // 
                pub const m_szMuzzleFlashThirdPersonParticle: usize = 0xA50; // 
                pub const m_szMuzzleFlashThirdPersonParticleAlt: usize = 0xB30; // 
                pub const m_szTracerParticle: usize = 0xC10; // 
                pub const m_GearSlot: usize = 0xCF0; // 
                pub const m_GearSlotPosition: usize = 0xCF4; // 
                pub const m_DefaultLoadoutSlot: usize = 0xCF8; // 
                pub const m_sWrongTeamMsg: usize = 0xD00; // 
                pub const m_nPrice: usize = 0xD08; // 
                pub const m_nKillAward: usize = 0xD0C; // 
                pub const m_nPrimaryReserveAmmoMax: usize = 0xD10; // 
                pub const m_nSecondaryReserveAmmoMax: usize = 0xD14; // 
                pub const m_bMeleeWeapon: usize = 0xD18; // 
                pub const m_bHasBurstMode: usize = 0xD19; // 
                pub const m_bIsRevolver: usize = 0xD1A; // 
                pub const m_bCannotShootUnderwater: usize = 0xD1B; // 
                pub const m_szName: usize = 0xD20; // 
                pub const m_szAnimExtension: usize = 0xD28; // 
                pub const m_eSilencerType: usize = 0xD30; // 
                pub const m_nCrosshairMinDistance: usize = 0xD34; // 
                pub const m_nCrosshairDeltaDistance: usize = 0xD38; // 
                pub const m_bIsFullAuto: usize = 0xD3C; // 
                pub const m_nNumBullets: usize = 0xD40; // 
                pub const m_flCycleTime: usize = 0xD44; // 
                pub const m_flMaxSpeed: usize = 0xD4C; // 
                pub const m_flSpread: usize = 0xD54; // 
                pub const m_flInaccuracyCrouch: usize = 0xD5C; // 
                pub const m_flInaccuracyStand: usize = 0xD64; // 
                pub const m_flInaccuracyJump: usize = 0xD6C; // 
                pub const m_flInaccuracyLand: usize = 0xD74; // 
                pub const m_flInaccuracyLadder: usize = 0xD7C; // 
                pub const m_flInaccuracyFire: usize = 0xD84; // 
                pub const m_flInaccuracyMove: usize = 0xD8C; // 
                pub const m_flRecoilAngle: usize = 0xD94; // 
                pub const m_flRecoilAngleVariance: usize = 0xD9C; // 
                pub const m_flRecoilMagnitude: usize = 0xDA4; // 
                pub const m_flRecoilMagnitudeVariance: usize = 0xDAC; // 
                pub const m_nTracerFrequency: usize = 0xDB4; // 
                pub const m_flInaccuracyJumpInitial: usize = 0xDBC; // 
                pub const m_flInaccuracyJumpApex: usize = 0xDC0; // 
                pub const m_flInaccuracyReload: usize = 0xDC4; // 
                pub const m_nRecoilSeed: usize = 0xDC8; // 
                pub const m_nSpreadSeed: usize = 0xDCC; // 
                pub const m_flTimeToIdleAfterFire: usize = 0xDD0; // 
                pub const m_flIdleInterval: usize = 0xDD4; // 
                pub const m_flAttackMovespeedFactor: usize = 0xDD8; // 
                pub const m_flHeatPerShot: usize = 0xDDC; // 
                pub const m_flInaccuracyPitchShift: usize = 0xDE0; // 
                pub const m_flInaccuracyAltSoundThreshold: usize = 0xDE4; // 
                pub const m_flBotAudibleRange: usize = 0xDE8; // 
                pub const m_szUseRadioSubtitle: usize = 0xDF0; // 
                pub const m_bUnzoomsAfterShot: usize = 0xDF8; // 
                pub const m_bHideViewModelWhenZoomed: usize = 0xDF9; // 
                pub const m_nZoomLevels: usize = 0xDFC; // 
                pub const m_nZoomFOV1: usize = 0xE00; // 
                pub const m_nZoomFOV2: usize = 0xE04; // 
                pub const m_flZoomTime0: usize = 0xE08; // 
                pub const m_flZoomTime1: usize = 0xE0C; // 
                pub const m_flZoomTime2: usize = 0xE10; // 
                pub const m_flIronSightPullUpSpeed: usize = 0xE14; // 
                pub const m_flIronSightPutDownSpeed: usize = 0xE18; // 
                pub const m_flIronSightFOV: usize = 0xE1C; // 
                pub const m_flIronSightPivotForward: usize = 0xE20; // 
                pub const m_flIronSightLooseness: usize = 0xE24; // 
                pub const m_angPivotAngle: usize = 0xE28; // 
                pub const m_vecIronSightEyePos: usize = 0xE34; // 
                pub const m_nDamage: usize = 0xE40; // 
                pub const m_flHeadshotMultiplier: usize = 0xE44; // 
                pub const m_flArmorRatio: usize = 0xE48; // 
                pub const m_flPenetration: usize = 0xE4C; // 
                pub const m_flRange: usize = 0xE50; // 
                pub const m_flRangeModifier: usize = 0xE54; // 
                pub const m_flFlinchVelocityModifierLarge: usize = 0xE58; // 
                pub const m_flFlinchVelocityModifierSmall: usize = 0xE5C; // 
                pub const m_flRecoveryTimeCrouch: usize = 0xE60; // 
                pub const m_flRecoveryTimeStand: usize = 0xE64; // 
                pub const m_flRecoveryTimeCrouchFinal: usize = 0xE68; // 
                pub const m_flRecoveryTimeStandFinal: usize = 0xE6C; // 
                pub const m_nRecoveryTransitionStartBullet: usize = 0xE70; // 
                pub const m_nRecoveryTransitionEndBullet: usize = 0xE74; // 
                pub const m_flThrowVelocity: usize = 0xE78; // 
                pub const m_vSmokeColor: usize = 0xE7C; // 
                pub const m_szAnimClass: usize = 0xE88; // 
            }
            // Parent: C_ModelPointEntity
            // Field count: 12
            //
            // Metadata:
            // NetworkVarNames: m_messageText (char)
            // NetworkVarNames: m_FontName (char)
            // NetworkVarNames: m_bEnabled (bool)
            // NetworkVarNames: m_bFullbright (bool)
            // NetworkVarNames: m_flWorldUnitsPerPx (float)
            // NetworkVarNames: m_flFontSize (float)
            // NetworkVarNames: m_flDepthOffset (float)
            // NetworkVarNames: m_Color (Color)
            // NetworkVarNames: m_nJustifyHorizontal (PointWorldTextJustifyHorizontal_t)
            // NetworkVarNames: m_nJustifyVertical (PointWorldTextJustifyVertical_t)
            // NetworkVarNames: m_nReorientMode (PointWorldTextReorientMode_t)
            pub mod C_PointWorldText {
                pub const m_bForceRecreateNextUpdate: usize = 0xD30; // 
                pub const m_messageText: usize = 0xD40; // 
                pub const m_FontName: usize = 0xF40; // 
                pub const m_bEnabled: usize = 0xF80; // 
                pub const m_bFullbright: usize = 0xF81; // 
                pub const m_flWorldUnitsPerPx: usize = 0xF84; // 
                pub const m_flFontSize: usize = 0xF88; // 
                pub const m_flDepthOffset: usize = 0xF8C; // 
                pub const m_Color: usize = 0xF90; // 
                pub const m_nJustifyHorizontal: usize = 0xF94; // 
                pub const m_nJustifyVertical: usize = 0xF98; // 
                pub const m_nReorientMode: usize = 0xF9C; // 
            }
            // Parent: C_BaseGrenade
            // Field count: 0
            pub mod CBumpMineProjectile {
            }
            // Parent: CBasePlayerController
            // Field count: 62
            //
            // Metadata:
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // MNetworkUserGroupProxy
            // NetworkVarNames: m_pInGameMoneyServices (CCSPlayerController_InGameMoneyServices*)
            // NetworkVarNames: m_pInventoryServices (CCSPlayerController_InventoryServices*)
            // NetworkVarNames: m_pActionTrackingServices (CCSPlayerController_ActionTrackingServices*)
            // NetworkVarNames: m_pDamageServices (CCSPlayerController_DamageServices*)
            // NetworkVarNames: m_iPing (uint32)
            // NetworkVarNames: m_bHasCommunicationAbuseMute (bool)
            // NetworkVarNames: m_szCrosshairCodes (string_t)
            // NetworkVarNames: m_iPendingTeamNum (uint8)
            // NetworkVarNames: m_flForceTeamTime (GameTime_t)
            // NetworkVarNames: m_iCompTeammateColor (int)
            // NetworkVarNames: m_bEverPlayedOnTeam (bool)
            // NetworkVarNames: m_szClan (string_t)
            // NetworkVarNames: m_iCoachingTeam (int)
            // NetworkVarNames: m_nPlayerDominated (uint64)
            // NetworkVarNames: m_nPlayerDominatingMe (uint64)
            // NetworkVarNames: m_iCompetitiveRanking (int)
            // NetworkVarNames: m_iCompetitiveWins (int)
            // NetworkVarNames: m_iCompetitiveRankType (int8)
            // NetworkVarNames: m_iCompetitiveRankingPredicted_Win (int)
            // NetworkVarNames: m_iCompetitiveRankingPredicted_Loss (int)
            // NetworkVarNames: m_iCompetitiveRankingPredicted_Tie (int)
            // NetworkVarNames: m_nEndMatchNextMapVote (int)
            // NetworkVarNames: m_unActiveQuestId (uint16)
            // NetworkVarNames: m_nQuestProgressReason (QuestProgress::Reason)
            // NetworkVarNames: m_unPlayerTvControlFlags (uint32)
            // NetworkVarNames: m_nDisconnectionTick (int)
            // NetworkVarNames: m_bControllingBot (bool)
            // NetworkVarNames: m_bHasControlledBotThisRound (bool)
            // NetworkVarNames: m_bCanControlObservedBot (bool)
            // NetworkVarNames: m_hPlayerPawn (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_hObserverPawn (CHandle<CCSObserverPawn>)
            // NetworkVarNames: m_bPawnIsAlive (bool)
            // NetworkVarNames: m_iPawnHealth (uint32)
            // NetworkVarNames: m_iPawnArmor (int)
            // NetworkVarNames: m_bPawnHasDefuser (bool)
            // NetworkVarNames: m_bPawnHasHelmet (bool)
            // NetworkVarNames: m_nPawnCharacterDefIndex (item_definition_index_t)
            // NetworkVarNames: m_iPawnLifetimeStart (int)
            // NetworkVarNames: m_iPawnLifetimeEnd (int)
            // NetworkVarNames: m_iPawnBotDifficulty (int)
            // NetworkVarNames: m_hOriginalControllerOfCurrentPawn (CHandle<CCSPlayerController>)
            // NetworkVarNames: m_iScore (int32)
            // NetworkVarNames: m_vecKills (EKillTypes_t)
            // NetworkVarNames: m_bMvpNoMusic (bool)
            // NetworkVarNames: m_eMvpReason (int)
            // NetworkVarNames: m_iMusicKitID (int)
            // NetworkVarNames: m_iMusicKitMVPs (int)
            // NetworkVarNames: m_iMVPs (int)
            pub mod CCSPlayerController {
                pub const m_pInGameMoneyServices: usize = 0x720; // 
                pub const m_pInventoryServices: usize = 0x728; // 
                pub const m_pActionTrackingServices: usize = 0x730; // 
                pub const m_pDamageServices: usize = 0x738; // 
                pub const m_iPing: usize = 0x740; // 
                pub const m_bHasCommunicationAbuseMute: usize = 0x744; // 
                pub const m_szCrosshairCodes: usize = 0x748; // 
                pub const m_iPendingTeamNum: usize = 0x750; // 
                pub const m_flForceTeamTime: usize = 0x754; // 
                pub const m_iCompTeammateColor: usize = 0x758; // 
                pub const m_bEverPlayedOnTeam: usize = 0x75C; // 
                pub const m_flPreviousForceJoinTeamTime: usize = 0x760; // 
                pub const m_szClan: usize = 0x768; // 
                pub const m_sSanitizedPlayerName: usize = 0x770; // 
                pub const m_iCoachingTeam: usize = 0x778; // 
                pub const m_nPlayerDominated: usize = 0x780; // 
                pub const m_nPlayerDominatingMe: usize = 0x788; // 
                pub const m_iCompetitiveRanking: usize = 0x790; // 
                pub const m_iCompetitiveWins: usize = 0x794; // 
                pub const m_iCompetitiveRankType: usize = 0x798; // 
                pub const m_iCompetitiveRankingPredicted_Win: usize = 0x79C; // 
                pub const m_iCompetitiveRankingPredicted_Loss: usize = 0x7A0; // 
                pub const m_iCompetitiveRankingPredicted_Tie: usize = 0x7A4; // 
                pub const m_nEndMatchNextMapVote: usize = 0x7A8; // 
                pub const m_unActiveQuestId: usize = 0x7AC; // 
                pub const m_nQuestProgressReason: usize = 0x7B0; // 
                pub const m_unPlayerTvControlFlags: usize = 0x7B4; // 
                pub const m_iDraftIndex: usize = 0x7E0; // 
                pub const m_msQueuedModeDisconnectionTimestamp: usize = 0x7E4; // 
                pub const m_uiAbandonRecordedReason: usize = 0x7E8; // 
                pub const m_bCannotBeKicked: usize = 0x7EC; // 
                pub const m_bEverFullyConnected: usize = 0x7ED; // 
                pub const m_bAbandonAllowsSurrender: usize = 0x7EE; // 
                pub const m_bAbandonOffersInstantSurrender: usize = 0x7EF; // 
                pub const m_bDisconnection1MinWarningPrinted: usize = 0x7F0; // 
                pub const m_bScoreReported: usize = 0x7F1; // 
                pub const m_nDisconnectionTick: usize = 0x7F4; // 
                pub const m_bControllingBot: usize = 0x800; // 
                pub const m_bHasControlledBotThisRound: usize = 0x801; // 
                pub const m_bHasBeenControlledByPlayerThisRound: usize = 0x802; // 
                pub const m_nBotsControlledThisRound: usize = 0x804; // 
                pub const m_bCanControlObservedBot: usize = 0x808; // 
                pub const m_hPlayerPawn: usize = 0x80C; // 
                pub const m_hObserverPawn: usize = 0x810; // 
                pub const m_bPawnIsAlive: usize = 0x814; // 
                pub const m_iPawnHealth: usize = 0x818; // 
                pub const m_iPawnArmor: usize = 0x81C; // 
                pub const m_bPawnHasDefuser: usize = 0x820; // 
                pub const m_bPawnHasHelmet: usize = 0x821; // 
                pub const m_nPawnCharacterDefIndex: usize = 0x822; // 
                pub const m_iPawnLifetimeStart: usize = 0x824; // 
                pub const m_iPawnLifetimeEnd: usize = 0x828; // 
                pub const m_iPawnBotDifficulty: usize = 0x82C; // 
                pub const m_hOriginalControllerOfCurrentPawn: usize = 0x830; // 
                pub const m_iScore: usize = 0x834; // 
                pub const m_vecKills: usize = 0x838; // 
                pub const m_bMvpNoMusic: usize = 0x850; // 
                pub const m_eMvpReason: usize = 0x854; // 
                pub const m_iMusicKitID: usize = 0x858; // 
                pub const m_iMusicKitMVPs: usize = 0x85C; // 
                pub const m_iMVPs: usize = 0x860; // 
                pub const m_bIsPlayerNameDirty: usize = 0x864; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponFiveSeven {
            }
            // Parent: C_DynamicProp
            // Field count: 7
            //
            // Metadata:
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkExcludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkIncludeByName
            // NetworkVarNames: m_eDoorState (DoorState_t)
            // NetworkVarNames: m_bLocked (bool)
            // NetworkVarNames: m_closedPosition (Vector)
            // NetworkVarNames: m_closedAngles (QAngle)
            // NetworkVarNames: m_hMaster (CHandle<C_BasePropDoor>)
            pub mod C_BasePropDoor {
                pub const m_eDoorState: usize = 0x1238; // 
                pub const m_modelChanged: usize = 0x123C; // 
                pub const m_bLocked: usize = 0x123D; // 
                pub const m_closedPosition: usize = 0x1240; // 
                pub const m_closedAngles: usize = 0x124C; // 
                pub const m_hMaster: usize = 0x1258; // 
                pub const m_vWhereToSetLightingOrigin: usize = 0x125C; // 
            }
            // Parent: C_CSGO_TeamSelectCharacterPosition
            // Field count: 0
            pub mod C_CSGO_TeamSelectTerroristPosition {
            }
            // Parent: C_CS2WeaponModuleBase
            // Field count: 1
            pub mod C_NametagModule {
                pub const m_strNametagString: usize = 0xF90; // 
            }
            // Parent: C_BreakableProp
            // Field count: 6
            pub mod C_PhysPropClientside {
                pub const m_flTouchDelta: usize = 0x1110; // 
                pub const m_fDeathTime: usize = 0x1114; // 
                pub const m_inertiaScale: usize = 0x1118; // 
                pub const m_vecDamagePosition: usize = 0x111C; // 
                pub const m_vecDamageDirection: usize = 0x1128; // 
                pub const m_nDamageType: usize = 0x1134; // 
            }
            // Parent: C_CSGO_TeamPreviewCamera
            // Field count: 0
            pub mod C_CSGO_TeamSelectCamera {
            }
            // Parent: C_BaseEntity
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_bDisabled (bool)
            // NetworkVarNames: m_iszSoundAreaType (string_t)
            // NetworkVarNames: m_vPos (Vector)
            pub mod C_SoundAreaEntityBase {
                pub const m_bDisabled: usize = 0x568; // 
                pub const m_bWasEnabled: usize = 0x570; // 
                pub const m_iszSoundAreaType: usize = 0x578; // 
                pub const m_vPos: usize = 0x580; // 
            }
            // Parent: C_BaseEntity
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_fog (fogparams_t)
            pub mod C_FogController {
                pub const m_fog: usize = 0x568; // 
                pub const m_bUseAngles: usize = 0x5D0; // 
                pub const m_iChangedVariables: usize = 0x5D4; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 0
            pub mod CPlayer_ItemServices {
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod C_FuncBrush {
            }
            // Parent: C_BaseEntity
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_worldName (string_t)
            // NetworkVarNames: m_layerName (string_t)
            // NetworkVarNames: m_bWorldLayerVisible (bool)
            // NetworkVarNames: m_bEntitiesSpawned (bool)
            pub mod CInfoWorldLayer {
                pub const m_pOutputOnEntitiesSpawned: usize = 0x568; // 
                pub const m_worldName: usize = 0x590; // 
                pub const m_layerName: usize = 0x598; // 
                pub const m_bWorldLayerVisible: usize = 0x5A0; // 
                pub const m_bEntitiesSpawned: usize = 0x5A1; // 
                pub const m_bCreateAsChildSpawnGroup: usize = 0x5A2; // 
                pub const m_hLayerSpawnGroup: usize = 0x5A4; // 
                pub const m_bWorldLayerActuallyVisible: usize = 0x5A8; // 
            }
            // Parent: C_BaseCombatCharacter
            // Field count: 0
            pub mod C_NetTestBaseCombatCharacter {
            }
            // Parent: CBaseAnimGraph
            // Field count: 2
            pub mod C_PhysMagnet {
                pub const m_aAttachedObjectsFromServer: usize = 0xF88; // 
                pub const m_aAttachedObjects: usize = 0xFA0; // CUtlVector<CHandle<C_BaseEntity>>
            }
            // Parent: C_BaseModelEntity
            // Field count: 8
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkOverride
            // MNetworkOverride
            // MNetworkOverride
            // MNetworkOverride
            // MNetworkOverride
            // NetworkVarNames: m_vecMoveDirEntitySpace (Vector)
            // NetworkVarNames: m_flTargetSpeed (float32)
            // NetworkVarNames: m_nTransitionStartTick (GameTick_t)
            // NetworkVarNames: m_nTransitionDurationTicks (int)
            // NetworkVarNames: m_flTransitionStartSpeed (float32)
            // NetworkVarNames: m_hConveyorModels (EHANDLE)
            pub mod C_FuncConveyor {
                pub const m_vecMoveDirEntitySpace: usize = 0xD30; // 
                pub const m_flTargetSpeed: usize = 0xD3C; // 
                pub const m_nTransitionStartTick: usize = 0xD40; // 
                pub const m_nTransitionDurationTicks: usize = 0xD44; // 
                pub const m_flTransitionStartSpeed: usize = 0xD48; // 
                pub const m_hConveyorModels: usize = 0xD50; // 
                pub const m_flCurrentConveyorOffset: usize = 0xD68; // 
                pub const m_flCurrentConveyorSpeed: usize = 0xD6C; // 
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // NetworkVarNames: m_pEntity (CEntityIdentity*)
            // NetworkVarNames: m_CScriptComponent (CScriptComponent::Storage_t)
            pub mod CEntityInstance {
                pub const m_iszPrivateVScripts: usize = 0x8; // 
                pub const m_pEntity: usize = 0x10; // 
                pub const m_CScriptComponent: usize = 0x28; // 
                pub const m_bVisibleinPVS: usize = 0x30; // 
            }
            // Parent: None
            // Field count: 3
            pub mod ServerAuthoritativeWeaponSlot_t {
                pub const unClass: usize = 0x28; // 
                pub const unSlot: usize = 0x2A; // 
                pub const unItemDefIdx: usize = 0x2C; // 
            }
            // Parent: CPlayerPawnComponent
            // Field count: 0
            pub mod CCSPlayer_DamageReactServices {
            }
            // Parent: CBaseAnimGraph
            // Field count: 16
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkExcludeByUserGroup
            // MNetworkOverride
            // MNetworkIncludeByName
            // NetworkVarNames: m_nViewModelIndex (uint32)
            // NetworkVarNames: m_nAnimationParity (uint32)
            // NetworkVarNames: m_flAnimationStartTime (float32)
            // NetworkVarNames: m_hWeapon (CHandle<CBasePlayerWeapon>)
            // NetworkVarNames: m_hControlPanel (CHandle<CBaseEntity>)
            pub mod C_BaseViewModel {
                pub const m_vecLastFacing: usize = 0xF90; // 
                pub const m_nViewModelIndex: usize = 0xF9C; // 
                pub const m_nAnimationParity: usize = 0xFA0; // 
                pub const m_flAnimationStartTime: usize = 0xFA4; // 
                pub const m_hWeapon: usize = 0xFA8; // 
                pub const m_sVMName: usize = 0xFB0; // 
                pub const m_sAnimationPrefix: usize = 0xFB8; // 
                pub const m_iCameraAttachment: usize = 0xFC0; // 
                pub const m_vecLastCameraAngles: usize = 0xFC4; // 
                pub const m_previousElapsedDuration: usize = 0xFD0; // 
                pub const m_previousCycle: usize = 0xFD4; // 
                pub const m_nOldAnimationParity: usize = 0xFD8; // 
                pub const m_hOldLayerSequence: usize = 0xFDC; // 
                pub const m_oldLayer: usize = 0xFE0; // 
                pub const m_oldLayerStartTime: usize = 0xFE4; // 
                pub const m_hControlPanel: usize = 0xFE8; // 
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod C_PortraitWorldCallbackHandler {
            }
            // Parent: CHostageRescueZoneShim
            // Field count: 0
            pub mod CHostageRescueZone {
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponNegev {
            }
            // Parent: None
            // Field count: 33
            //
            // Metadata:
            // NetworkVarNames: m_hParent (CGameSceneNodeHandle)
            // NetworkVarNames: m_vecOrigin (CNetworkOriginCellCoordQuantizedVector)
            // NetworkVarNames: m_angRotation (QAngle)
            // NetworkVarNames: m_flScale (float)
            // NetworkVarNames: m_name (CUtlStringToken)
            // NetworkVarNames: m_hierarchyAttachName (CUtlStringToken)
            pub mod CGameSceneNode {
                pub const m_nodeToWorld: usize = 0x10; // 
                pub const m_pOwner: usize = 0x30; // 
                pub const m_pParent: usize = 0x38; // 
                pub const m_pChild: usize = 0x40; // 
                pub const m_pNextSibling: usize = 0x48; // 
                pub const m_hParent: usize = 0x78; // 
                pub const m_vecOrigin: usize = 0x88; // CNetworkOriginCellCoordQuantizedVector
                pub const m_angRotation: usize = 0xC0; // 
                pub const m_flScale: usize = 0xCC; // 
                pub const m_vecAbsOrigin: usize = 0xD0; // 
                pub const m_angAbsRotation: usize = 0xDC; // 
                pub const m_flAbsScale: usize = 0xE8; // 
                pub const m_nParentAttachmentOrBone: usize = 0xEC; // 
                pub const m_bDebugAbsOriginChanges: usize = 0xEE; // 
                pub const m_bDormant: usize = 0xEF; // 
                pub const m_bForceParentToBeNetworked: usize = 0xF0; // 
                pub const m_bDirtyHierarchy: usize = 0x0; // 
                pub const m_bDirtyBoneMergeInfo: usize = 0x0; // 
                pub const m_bNetworkedPositionChanged: usize = 0x0; // 
                pub const m_bNetworkedAnglesChanged: usize = 0x0; // 
                pub const m_bNetworkedScaleChanged: usize = 0x0; // 
                pub const m_bWillBeCallingPostDataUpdate: usize = 0x0; // 
                pub const m_bBoneMergeFlex: usize = 0x0; // 
                pub const m_nLatchAbsOrigin: usize = 0x0; // 
                pub const m_bDirtyBoneMergeBoneToRoot: usize = 0x0; // 
                pub const m_nHierarchicalDepth: usize = 0xF3; // 
                pub const m_nHierarchyType: usize = 0xF4; // 
                pub const m_nDoNotSetAnimTimeInInvalidatePhysicsCount: usize = 0xF5; // 
                pub const m_name: usize = 0xF8; // 
                pub const m_hierarchyAttachName: usize = 0x138; // 
                pub const m_flZOffset: usize = 0x13C; // 
                pub const m_flClientLocalScale: usize = 0x140; // 
                pub const m_vRenderOrigin: usize = 0x144; // 
            }
            // Parent: C_MolotovGrenade
            // Field count: 0
            pub mod C_IncendiaryGrenade {
            }
            // Parent: C_CSWeaponBase
            // Field count: 0
            pub mod CTablet {
            }
            // Parent: CBasePulseGraphInstance
            // Field count: 1
            //
            // Metadata:
            // MPulseInstanceDomainInfo
            // MPulseDomainHookInfo
            // MPulseLibraryBindings
            // MPulseDomainOptInFeatureTag
            pub mod CCSPointScript {
                pub const m_pParent: usize = 0xF8; // 
            }
            // Parent: C_CSPlayerPawnBase
            // Field count: 1
            //
            // Metadata:
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            // MNetworkVarTypeOverride
            // MNetworkIncludeByName
            pub mod C_CSObserverPawn {
                pub const m_hDetectParentChange: usize = 0x1510; // 
            }
            // Parent: C_BaseEntity
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: m_hEntAttached (CHandle<C_BaseEntity>)
            // NetworkVarNames: m_bCheapEffect (bool)
            pub mod C_EntityFlame {
                pub const m_hEntAttached: usize = 0x568; // 
                pub const m_hOldAttached: usize = 0x590; // 
                pub const m_bCheapEffect: usize = 0x594; // 
            }
            // Parent: C_BaseGrenade
            // Field count: 0
            pub mod CTripWireFireProjectile {
            }
            // Parent: C_BaseModelEntity
            // Field count: 0
            pub mod C_Breakable {
            }
            // Parent: CPlayer_ObserverServices
            // Field count: 8
            pub mod CCSObserver_ObserverServices {
                pub const m_hLastObserverTarget: usize = 0x58; // 
                pub const m_vecObserverInterpolateOffset: usize = 0x5C; // 
                pub const m_vecObserverInterpStartPos: usize = 0x68; // 
                pub const m_flObsInterp_PathLength: usize = 0x74; // 
                pub const m_qObsInterp_OrientationStart: usize = 0x80; // 
                pub const m_qObsInterp_OrientationTravelDir: usize = 0x90; // 
                pub const m_obsInterpState: usize = 0xA0; // 
                pub const m_bObserverInterpolationNeedsDeferredSetup: usize = 0xA4; // 
            }
            // Parent: C_BaseEntity
            // Field count: 0
            pub mod C_TintController {
            }
            // Parent: CPlayerPawnComponent
            // Field count: 0
            pub mod CPlayer_AutoaimServices {
            }
            // Parent: C_BaseCombatCharacter
            // Field count: 23
            //
            // Metadata:
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkIncludeByName
            // MNetworkOverride
            // NetworkVarNames: m_entitySpottedState (EntitySpottedState_t)
            // NetworkVarNames: m_leader (CHandle<CBaseEntity>)
            // NetworkVarNames: m_reuseTimer (CountdownTimer)
            // NetworkVarNames: m_vel (Vector)
            // NetworkVarNames: m_isRescued (bool)
            // NetworkVarNames: m_jumpedThisFrame (bool)
            // NetworkVarNames: m_nHostageState (int)
            // NetworkVarNames: m_bHandsHaveBeenCut (bool)
            // NetworkVarNames: m_hHostageGrabber (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_flRescueStartTime (GameTime_t)
            // NetworkVarNames: m_flGrabSuccessTime (GameTime_t)
            // NetworkVarNames: m_flDropStartTime (GameTime_t)
            pub mod C_Hostage {
                pub const m_entitySpottedState: usize = 0x11A8; // 
                pub const m_leader: usize = 0x11C0; // 
                pub const m_reuseTimer: usize = 0x11C8; // 
                pub const m_vel: usize = 0x11E0; // 
                pub const m_isRescued: usize = 0x11EC; // 
                pub const m_jumpedThisFrame: usize = 0x11ED; // 
                pub const m_nHostageState: usize = 0x11F0; // 
                pub const m_bHandsHaveBeenCut: usize = 0x11F4; // 
                pub const m_hHostageGrabber: usize = 0x11F8; // 
                pub const m_fLastGrabTime: usize = 0x11FC; // 
                pub const m_vecGrabbedPos: usize = 0x1200; // 
                pub const m_flRescueStartTime: usize = 0x120C; // 
                pub const m_flGrabSuccessTime: usize = 0x1210; // 
                pub const m_flDropStartTime: usize = 0x1214; // 
                pub const m_flDeadOrRescuedTime: usize = 0x1218; // 
                pub const m_blinkTimer: usize = 0x1220; // 
                pub const m_lookAt: usize = 0x1238; // 
                pub const m_lookAroundTimer: usize = 0x1248; // 
                pub const m_isInit: usize = 0x1260; // 
                pub const m_eyeAttachment: usize = 0x1261; // 
                pub const m_chestAttachment: usize = 0x1262; // 
                pub const m_pPredictionOwner: usize = 0x1268; // 
                pub const m_fNewestAlphaThinkTime: usize = 0x1270; // 
            }
            // Parent: C_BaseModelEntity
            // Field count: 24
            //
            // Metadata:
            // NetworkVarNames: m_hSpriteMaterial (HMaterialStrong)
            // NetworkVarNames: m_hAttachedToEntity (CHandle<CBaseEntity>)
            // NetworkVarNames: m_nAttachment (AttachmentHandle_t)
            // NetworkVarNames: m_flSpriteFramerate (float32)
            // NetworkVarNames: m_flFrame (float32)
            // NetworkVarNames: m_nBrightness (uint32)
            // NetworkVarNames: m_flBrightnessDuration (float32)
            // NetworkVarNames: m_flSpriteScale (float32)
            // NetworkVarNames: m_flScaleDuration (float32)
            // NetworkVarNames: m_bWorldSpaceScale (bool)
            // NetworkVarNames: m_flGlowProxySize (float32)
            // NetworkVarNames: m_flHDRColorScale (float32)
            pub mod C_Sprite {
                pub const m_hSpriteMaterial: usize = 0xD28; // 
                pub const m_hAttachedToEntity: usize = 0xD30; // 
                pub const m_nAttachment: usize = 0xD34; // 
                pub const m_flSpriteFramerate: usize = 0xD38; // 
                pub const m_flFrame: usize = 0xD3C; // 
                pub const m_flDieTime: usize = 0xD40; // 
                pub const m_nBrightness: usize = 0xD50; // 
                pub const m_flBrightnessDuration: usize = 0xD54; // 
                pub const m_flSpriteScale: usize = 0xD58; // 
                pub const m_flScaleDuration: usize = 0xD5C; // 
                pub const m_bWorldSpaceScale: usize = 0xD60; // 
                pub const m_flGlowProxySize: usize = 0xD64; // 
                pub const m_flHDRColorScale: usize = 0xD68; // 
                pub const m_flLastTime: usize = 0xD6C; // 
                pub const m_flMaxFrame: usize = 0xD70; // 
                pub const m_flStartScale: usize = 0xD74; // 
                pub const m_flDestScale: usize = 0xD78; // 
                pub const m_flScaleTimeStart: usize = 0xD7C; // 
                pub const m_nStartBrightness: usize = 0xD80; // 
                pub const m_nDestBrightness: usize = 0xD84; // 
                pub const m_flBrightnessTimeStart: usize = 0xD88; // 
                pub const m_hOldSpriteMaterial: usize = 0xD90; // CWeakHandle<InfoForResourceTypeIMaterial2>
                pub const m_nSpriteWidth: usize = 0xE30; // 
                pub const m_nSpriteHeight: usize = 0xE34; // 
            }
            // Parent: C_CSWeaponBaseGun
            // Field count: 0
            pub mod C_WeaponM4A1Silencer {
            }
            // Parent: CBaseAnimGraph
            // Field count: 30
            //
            // Metadata:
            // NetworkVarNames: m_bBombTicking (bool)
            // NetworkVarNames: m_nBombSite (int)
            // NetworkVarNames: m_nSourceSoundscapeHash (int)
            // NetworkVarNames: m_entitySpottedState (EntitySpottedState_t)
            // NetworkVarNames: m_flC4Blow (GameTime_t)
            // NetworkVarNames: m_bCannotBeDefused (bool)
            // NetworkVarNames: m_bHasExploded (bool)
            // NetworkVarNames: m_flTimerLength (float)
            // NetworkVarNames: m_bBeingDefused (bool)
            // NetworkVarNames: m_flDefuseLength (float)
            // NetworkVarNames: m_flDefuseCountDown (GameTime_t)
            // NetworkVarNames: m_bBombDefused (bool)
            // NetworkVarNames: m_hBombDefuser (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_hControlPanel (CHandle<CBaseEntity>)
            // NetworkVarNames: m_AttributeManager (CAttributeContainer)
            pub mod C_PlantedC4 {
                pub const m_bBombTicking: usize = 0xF90; // 
                pub const m_nBombSite: usize = 0xF94; // 
                pub const m_nSourceSoundscapeHash: usize = 0xF98; // 
                pub const m_entitySpottedState: usize = 0xFA0; // 
                pub const m_flNextGlow: usize = 0xFB8; // 
                pub const m_flNextBeep: usize = 0xFBC; // 
                pub const m_flC4Blow: usize = 0xFC0; // 
                pub const m_bCannotBeDefused: usize = 0xFC4; // 
                pub const m_bHasExploded: usize = 0xFC5; // 
                pub const m_flTimerLength: usize = 0xFC8; // 
                pub const m_bBeingDefused: usize = 0xFCC; // 
                pub const m_bTriggerWarning: usize = 0xFD0; // 
                pub const m_bExplodeWarning: usize = 0xFD4; // 
                pub const m_bC4Activated: usize = 0xFD8; // 
                pub const m_bTenSecWarning: usize = 0xFD9; // 
                pub const m_flDefuseLength: usize = 0xFDC; // 
                pub const m_flDefuseCountDown: usize = 0xFE0; // 
                pub const m_bBombDefused: usize = 0xFE4; // 
                pub const m_hBombDefuser: usize = 0xFE8; // 
                pub const m_hControlPanel: usize = 0xFEC; // 
                pub const m_AttributeManager: usize = 0xFF0; // 
                pub const m_hDefuserMultimeter: usize = 0x1498; // 
                pub const m_flNextRadarFlashTime: usize = 0x149C; // 
                pub const m_bRadarFlash: usize = 0x14A0; // 
                pub const m_pBombDefuser: usize = 0x14A4; // 
                pub const m_fLastDefuseTime: usize = 0x14A8; // 
                pub const m_pPredictionOwner: usize = 0x14B0; // 
                pub const m_vecC4ExplodeSpectatePos: usize = 0x14B8; // 
                pub const m_vecC4ExplodeSpectateAng: usize = 0x14C4; // 
                pub const m_flC4ExplodeSpectateDuration: usize = 0x14D0; // 
            }
            // Parent: CBaseAnimGraph
            // Field count: 8
            //
            // Metadata:
            // NetworkVarNames: m_ragPos (Vector)
            // NetworkVarNames: m_ragAngles (QAngle)
            // NetworkVarNames: m_flBlendWeight (float32)
            // NetworkVarNames: m_hRagdollSource (EHANDLE)
            pub mod C_RagdollProp {
                pub const m_ragPos: usize = 0xF90; // C_NetworkUtlVectorBase<Vector>
                pub const m_ragAngles: usize = 0xFA8; // C_NetworkUtlVectorBase<QAngle>
                pub const m_flBlendWeight: usize = 0xFC0; // 
                pub const m_hRagdollSource: usize = 0xFC4; // 
                pub const m_iEyeAttachment: usize = 0xFC8; // 
                pub const m_flBlendWeightCurrent: usize = 0xFCC; // 
                pub const m_parentPhysicsBoneIndices: usize = 0xFD0; // 
                pub const m_worldSpaceBoneComputationOrder: usize = 0xFE8; // 
            }
            // Parent: C_CSGO_TeamPreviewCamera
            // Field count: 0
            pub mod C_CSGO_TerroristTeamIntroCamera {
            }
            // Parent: C_BaseEntity
            // Field count: 7
            //
            // Metadata:
            // NetworkVarNames: m_iActiveIssueIndex (int)
            // NetworkVarNames: m_iOnlyTeamToVote (int)
            // NetworkVarNames: m_nVoteOptionCount (int)
            // NetworkVarNames: m_nPotentialVotes (int)
            // NetworkVarNames: m_bIsYesNoVote (bool)
            pub mod C_VoteController {
                pub const m_iActiveIssueIndex: usize = 0x578; // 
                pub const m_iOnlyTeamToVote: usize = 0x57C; // 
                pub const m_nVoteOptionCount: usize = 0x580; // 
                pub const m_nPotentialVotes: usize = 0x594; // 
                pub const m_bVotesDirty: usize = 0x598; // 
                pub const m_bTypeDirty: usize = 0x599; // 
                pub const m_bIsYesNoVote: usize = 0x59A; // 
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // NetworkVarNames: nType (FixAngleSet_t)
            // NetworkVarNames: qAngle (QAngle)
            // NetworkVarNames: nIndex (uint32)
            pub mod ViewAngleServerChange_t {
                pub const nType: usize = 0x30; // 
                pub const qAngle: usize = 0x34; // 
                pub const nIndex: usize = 0x40; // 
            }
            // Parent: C_DynamicProp
            // Field count: 7
            //
            // Metadata:
            // NetworkVarNames: m_jumpedThisFrame (bool)
            // NetworkVarNames: m_leader (CHandle<CCSPlayerPawn>)
            // NetworkVarNames: m_AttributeManager (CAttributeContainer)
            pub mod C_Chicken {
                pub const m_hHolidayHatAddon: usize = 0x1230; // 
                pub const m_jumpedThisFrame: usize = 0x1234; // 
                pub const m_leader: usize = 0x1238; // 
                pub const m_AttributeManager: usize = 0x1240; // 
                pub const m_bAttributesInitialized: usize = 0x16E8; // 
                pub const m_hWaterWakeParticles: usize = 0x16EC; // 
                pub const m_bIsPreviewModel: usize = 0x16F0; // 
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // NetworkVarNames: m_Transforms (CTransform)
            // NetworkVarNames: m_hOwner (EHANDLE)
            pub mod PhysicsRagdollPose_t {
                pub const m_Transforms: usize = 0x8; // C_NetworkUtlVectorBase<CTransform>
                pub const m_hOwner: usize = 0x20; // 
            }
        }
    }
}
