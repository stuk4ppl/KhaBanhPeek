#pragma once
#include <Windows.h>

#include "../shared/UseOffset.h"
#include "../../sdk/memory/memory.h"
#include "../shared/Math.h"
#include "../schemas/LocalPlayer.h"
#include "../schemas/PlayerAction.h"
#include "../schemas/Player.h"

class Misc {
private:
	Misc();

	uintptr_t* GetFlashDuration();
public:
	static Misc* GetMiscHandle();

	static void RadarHack();
	static void AntiFlash();
	static void BunnyHop();
};
