#pragma once
#include <Windows.h>

#include "../../sdk/memory/memory.h"

#include "../shared/UseOffset.h"
#include "../shared/Math.h"
#include "../shared/Color.h"

#include "../schemas/LocalPlayer.h"
#include "../schemas/Player.h"

class Player;

class Glow {
private:
	Glow();

	void GlowCustomize(Player* player);

	ColorBYTE EnemyColor = { 0xFF, 0x56, 0x00, 0xFF };
	                       //  R  -  G  -  B  -  A
	ColorBYTE AllyColor =  { 0x00, 0xFF, 0xFF, 0xFF };
public:
	static void Render();

	static Glow* GetGlowHandle();
};
