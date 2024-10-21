#include "../../include/schemas/PlayerAction.h"

PlayerAction* PlayerAction::Jump() {
	static uintptr_t ClientModule = (uintptr_t)GetModuleHandle(L"client.dll");
	static PlayerAction* Jump = (PlayerAction*)(ClientModule + Offsets::ForceJump);

	return Jump;
}

PlayerAction* PlayerAction::Attack() {
	static uintptr_t ClientModule = (uintptr_t)GetModuleHandle(L"client.dll");
	static PlayerAction* Attack = (PlayerAction*)(ClientModule + Offsets::ForceAttack);

	return Attack;
}