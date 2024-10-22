#include "../../include/bot/TriggerBot.h"

static uintptr_t ClientModule = (uintptr_t)GetModuleHandle(L"client.dll");

Player* TriggerBot::GetEntityPawnTargetByCrossHair() {
    uintptr_t CrossHairID = 0;

    LocalPlayer* Localplayer = LocalPlayer::GetLocalPlayer();
    if (Localplayer) {
        CrossHairID = *Localplayer->GetCrossHairID();
    }
    return Player::GetPlayerByCrossHairID(CrossHairID);
}

Player* TriggerBot::CheckEgilibity() {
    Player* PlayerPawn = GetEntityPawnTargetByCrossHair();
    LocalPlayer* Localplayer = LocalPlayer::GetLocalPlayer();

    if (!Localplayer) {
        return nullptr;
    }
    if (PlayerPawn) {
        if ((strcmp(Localplayer->GetWeaponNameOnHand(), "weapon_awp") == 0) ||
            (strcmp(Localplayer->GetWeaponNameOnHand(), "weapon_ssg08") == 0) ||
            (strcmp(Localplayer->GetWeaponNameOnHand(), "weapon_scar20") == 0) ||
            (strcmp(Localplayer->GetWeaponNameOnHand(), "weapon_g3sg1") == 0)) {
            if (!Localplayer->isScoped()) {
                return 0;
            }
        }

        if (*PlayerPawn->GetHealth() <= 0 || (*PlayerPawn->GetTeam() == *Localplayer->GetTeam())) {
            return 0;
        }
        else {
            return PlayerPawn;
        }
    }
    return 0;
}

void TriggerBot::Shoot(float DelayTime) {
    PlayerAction* AttackInstance = PlayerAction::Attack();
    *(uintptr_t*)AttackInstance = ForceAction;
    Sleep(1);
    *(uintptr_t*)AttackInstance = UnForceAction;
    Sleep(DelayTime);
}

int TriggerBot::GetDelayTime() {
    const char* WeaponName = LocalPlayer::GetLocalPlayer()->GetWeaponNameOnHand();
    if (strcmp(WeaponName, "weapon_ak47") == 0) {
        DelayTime = 6;
    }
    else if (strcmp(WeaponName, "weapon_deagle") == 0) {
        DelayTime = 1;
    }
    else if (strcmp(WeaponName, "weapon_m4a1") == 0) {
        DelayTime = 1;
    }
    else if (strcmp(WeaponName, "weapon_awp") == 0) {
        DelayTime = 3;
    }
    else if (strcmp(WeaponName, "weapon_ssg08") == 0) {
        DelayTime = 3;
    }
    return DelayTime;
}

float TriggerBot::GetDistance(Player* player) {
    LocalPlayer* Localplayer = LocalPlayer::GetLocalPlayer();

    if (!Localplayer) {
        return 0.0;
    }

    if (!player) {
        return 0.0;
    }

    Vec3* VecOrigin = Localplayer->GetVecOrigin();
    Vec3* PVecOrigin = player->GetVecOrigin();

    return Localplayer->GetDistance(PVecOrigin) / ( 100 / GetDelayTime());
}

void TriggerBot::Trigger() {
    if (Player* player = CheckEgilibity()) {
        float DelayTime = GetDistance(player);
        Shoot(DelayTime);
    }
}
