#pragma once
#include <stdint.h>

#include "../../sdk/A2xdump/output/offsets.hpp"
#include "../../sdk/A2xdump/output/buttons.hpp"
#include "../../sdk/A2xdump/output/client_dll.hpp"

//================= A2XDUMPER OFFSET =================//
namespace Offsets {
	//Offsets::dll
	constexpr uintptr_t LocalPlayer = cs2_dumper::offsets::client_dll::dwLocalPlayerPawn;
	constexpr uintptr_t EntityList = cs2_dumper::offsets::client_dll::dwEntityList;
	constexpr uintptr_t ViewAngle = cs2_dumper::offsets::client_dll::dwViewAngles;

	//buttons.dll
	constexpr uintptr_t ForceJump = cs2_dumper::buttons::jump;
	constexpr uintptr_t ForceAttack = cs2_dumper::buttons::attack;

	//engine2.dll
	constexpr uintptr_t SignOnState = cs2_dumper::offsets::engine2_dll::dwNetworkGameClient_signOnState;
	constexpr uintptr_t NetworkGameClient = cs2_dumper::offsets::engine2_dll::dwNetworkGameClient;
	constexpr uintptr_t MaxClient = cs2_dumper::offsets::engine2_dll::dwNetworkGameClient_maxClients;

	//client.dll
	//C_CSPlayerPawn
	constexpr uintptr_t isScoped = cs2_dumper::schemas::client_dll::C_CSPlayerPawn::m_bIsScoped;
	constexpr uintptr_t EntitySpottedState = cs2_dumper::schemas::client_dll::C_CSPlayerPawn::m_entitySpottedState;

	//C_CSPlayerPawnBase
	constexpr uintptr_t FlashDuration = cs2_dumper::schemas::client_dll::C_CSPlayerPawnBase::m_flFlashDuration;
	constexpr uintptr_t IDEntIndex = cs2_dumper::schemas::client_dll::C_CSPlayerPawnBase::m_iIDEntIndex;
	constexpr uintptr_t ClippingWeapon = cs2_dumper::schemas::client_dll::C_CSPlayerPawnBase::m_pClippingWeapon;
	constexpr uintptr_t WeaponName = 0x20; // ClippingWeapon + 0x10 + 0x20 = WeaponName;

	//C_BaseEntity
	constexpr uintptr_t TeamNum = cs2_dumper::schemas::client_dll::C_BaseEntity::m_iTeamNum;
	constexpr uintptr_t Health = cs2_dumper::schemas::client_dll::C_BaseEntity::m_iHealth;
	constexpr uintptr_t pGameSceneNode = cs2_dumper::schemas::client_dll::C_BaseEntity::m_pGameSceneNode;

	//EntitySpottedState_t
	constexpr uintptr_t isSpotted = cs2_dumper::schemas::client_dll::EntitySpottedState_t::m_bSpotted;
	constexpr uintptr_t isSpottedByMask = cs2_dumper::schemas::client_dll::EntitySpottedState_t::m_bSpottedByMask;

	//CGameSceneNode
	constexpr uintptr_t VecOrigin = cs2_dumper::schemas::client_dll::CGameSceneNode::m_vecOrigin;

	//CCSPlayerController
	constexpr uintptr_t hPlayerPawn = cs2_dumper::schemas::client_dll::CCSPlayerController::m_hPlayerPawn;

	//C_BaseModelEntity
	constexpr uintptr_t m_Glow = cs2_dumper::schemas::client_dll::C_BaseModelEntity::m_Glow;
	constexpr uintptr_t ViewOffset = cs2_dumper::schemas::client_dll::C_BaseModelEntity::m_vecViewOffset;
	constexpr uintptr_t ClrRender = cs2_dumper::schemas::client_dll::C_BaseModelEntity::m_clrRender;

	//CGlowProperty
	constexpr uintptr_t m_ColorOveride = cs2_dumper::schemas::client_dll::CGlowProperty::m_glowColorOverride;
	constexpr uintptr_t bGlowing = cs2_dumper::schemas::client_dll::CGlowProperty::m_bGlowing;

	//CSkeletonInstance
	constexpr uintptr_t ModelState = cs2_dumper::schemas::client_dll::CSkeletonInstance::m_modelState;
	constexpr uintptr_t BoneMatrix = cs2_dumper::schemas::client_dll::CSkeletonInstance::m_modelState + 0x80;

};