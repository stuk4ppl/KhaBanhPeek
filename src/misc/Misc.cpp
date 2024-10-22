#include "../../include/misc/Misc.h"

static uintptr_t ClientModule = (uintptr_t)GetModuleHandle(L"client.dll");

Misc::Misc() {}

Misc* Misc::GetMiscHandle() {
    static Misc misc;
    return &misc;
}

void Misc::RadarHack() {
    LocalPlayer* Localplayer = LocalPlayer::GetLocalPlayer();

    if (!Localplayer) {
        return;
    }

    for (short i = 0; i < Player::GetMaxPlayer(); i++) {
        Player* player = Player::GetPlayer(i);

        if (!player) {
            continue;
        }

        if ((uintptr_t)player == (uintptr_t)Localplayer) {
            continue;
        }

        if (*player->GetHealth() <= 0 || (*player->GetTeam() == *Localplayer->GetTeam())) {
            continue;
        }

        //Set radar
        *player->EntitySpotted.GetIsSpotted() = 1;
        *player->EntitySpotted.GetIsSpottedByMask() = 1;
    }
}

void Misc::AntiFlash() {
    Misc* misc = Misc::GetMiscHandle();
    uintptr_t* FlashDuration = misc->GetFlashDuration();

    if (!FlashDuration) {
        return;
    }
    *FlashDuration = 0;
}

uintptr_t* Misc::GetFlashDuration() {
    LocalPlayer* Localplayer = LocalPlayer::GetLocalPlayer();

    if (!Localplayer) {
        return nullptr;
    }
    return (uintptr_t*)((uintptr_t)Localplayer + Offsets::FlashDuration);
}

void Misc::BunnyHop() {
    PlayerAction* JumpInstance = PlayerAction::Jump();

    if (!JumpInstance) {
        return;
    }

    static bool jump = false;
    if (GetAsyncKeyState(VK_SPACE)) {
        if (!jump) {
            *(uintptr_t*)JumpInstance |= IN_JUMP; // Dat bit thanh 1
            jump = true;
        }
        else {
            *(uintptr_t*)JumpInstance &= ~IN_JUMP; // Dat bit thanh 0
            jump = false;;
        }
    }
}


