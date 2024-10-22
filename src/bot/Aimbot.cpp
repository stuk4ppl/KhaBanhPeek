#include "../../include/bot/Aimbot.h"

float Aimbot::GetDistanceBetween2DPoint(Vec2 p1, Vec2 p2) {
    Vec2 Delta = { p2.x - p1.x, p2.y - p1.y };
    return sqrt(pow(Delta.x, 2) + pow(Delta.y, 2));
}

Vec2 Aimbot::WorldToScreenEnemy(LocalPlayer* Local, Player* Target) {
    Vec3 Bone = Target->GetBonePos(6);
    Vec2 PositionOnScreen = LocalPlayer::GetLocalPlayer()->CalcAngle(&Bone); //Take yaw / pitch on screen of enemy head
    Vec2* LocalViewAngles = LocalPlayer::GetLocalPlayer()->GetViewAngles();

    float Distance = GetDistanceBetween2DPoint(*LocalViewAngles, PositionOnScreen);

    if (abs(Distance) < FOV) {
        return PositionOnScreen;
    }
    else {
        return { 0, 0 };
    }
    return { 0,0 };
}

void Aimbot::RunAimbot() {
    LocalPlayer* Localplayer = LocalPlayer::GetLocalPlayer();

    if (!Localplayer) {
        return;
    }

    for (int i = 0; i < Player::GetMaxPlayer(); i++) {
        Player* CurrentPlayer = Player::GetPlayer(i);

        if (!CurrentPlayer || ((uintptr_t)CurrentPlayer == *(uintptr_t*)Localplayer)) {
            continue;
        }

        if (*CurrentPlayer->GetTeam() == *Localplayer->GetTeam()) {
            continue;
        }

        if (*CurrentPlayer->GetHealth() < 1 || *Localplayer->GetHealth() < 1) {
            continue;
        }

        Vec2 PositionNeedAim = WorldToScreenEnemy(Localplayer, CurrentPlayer);
        SetNewAnglesForPlayer(PositionNeedAim);
    }
}

void Aimbot::SetNewAnglesForPlayer(Vec2 NewAngle) {
    if (!(NewAngle == 0)) {
        Vec2* LocalViewAngles = LocalPlayer::GetLocalPlayer()->GetViewAngles();
        LocalViewAngles->x = NewAngle.x;
        LocalViewAngles->y = NewAngle.y;
    }
}

void Aimbot::Run() {
    RunAimbot();
}