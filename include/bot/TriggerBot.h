#pragma once
#include <Windows.h>
#include <cmath>

#include "../shared/Math.h"
#include "../shared/UseOffset.h"
#include "../../sdk/memory/memory.h"
#include "../schemas/Player.h"
#include "../schemas/LocalPlayer.h"
#include "../schemas/PlayerAction.h"

#define DefaultShootSpeed 10

class TriggerBot {
private:
	Player* GetEntityPawnTargetByCrossHair();

	Player* CheckEgilibity();

	int GetDelayTime();

	float GetDistance(Player* player);

	void Shoot(float SpeedBaseOfDistance);

	float DelayTime = DefaultShootSpeed;
public:
	void Trigger();
};

