#pragma once
#include <Windows.h>
#include "../shared/UseOffset.h"

class PlayerAction {
private:
	PlayerAction();
public:
	static PlayerAction* Jump();

	static PlayerAction* Attack();
};