#pragma once
#include "../shared/Math.h"
class LocalPlayer
{
private:
	LocalPlayer();
public:
	static LocalPlayer* GetLocalPlayer();

	Vec3* GetVecOrigin();

	Vec3* GetViewOffsets();

	int* GetHealth();

	int* GetTeam();

	uintptr_t* GetCrossHairID();

	Vec2* GetViewAngles();

	float GetDistance(Vec3* Enemy);

	const char* GetWeaponNameOnHand();

	uintptr_t* GetHandleToCurrentWeapon();

	Vec2 CalcAngle(Vec3* Enemy);

	bool isScoped();

	bool isPlayerConnected();
};

