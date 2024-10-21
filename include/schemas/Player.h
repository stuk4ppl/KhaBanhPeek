#pragma once
#include "../shared/Math.h"
#include "../visual/Glow.h"
#include "../shared/Color.h"

class Player;

struct CGlowProperty {
public:
	ColorBYTE* GetColor();
	uintptr_t* GetIsGlowing();
};

struct EntitySpottedState {
public:
	uintptr_t* GetIsSpotted();
	uintptr_t* GetIsSpottedByMask();
};

struct BaseModelEntity {
public:
	ColorBYTE* GetClrRender();
};

class Player
{
private:
	Player();

public:
	static int GetMaxPlayer();

	static Player* GetPlayer(int index);

	static Player* GetPlayerByCrossHairID(uintptr_t ID);

	static void GlowPlayer();

	CGlowProperty Glow;
	
	EntitySpottedState EntitySpotted;
	
	BaseModelEntity Model;

	int* GetHealth();

	int* GetTeam();

	Vec3* GetVecOrigin();

	Vec3* GetViewOffset();

	Vec3 GetBonePos(int BoneID);

}; 


