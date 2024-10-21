#pragma once
#include <Windows.h>
#include <cmath>

#include "../schemas/LocalPlayer.h"
#include "../schemas/Player.h"
#include "../shared/Math.h"

class Aimbot {
private:
	float GetDistanceBetween2DPoint(Vec2 p1, Vec2 p2);

	Vec2 WorldToScreenEnemy(LocalPlayer* Local, Player* Target);

	void SetNewAnglesForPlayer(Vec2 NewAngle);

	void RunAimbot();
public:
	void Run();

	int FOV = 5;

};