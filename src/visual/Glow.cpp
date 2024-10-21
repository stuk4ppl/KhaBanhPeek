#include "../../include/visual/Glow.h"

Glow::Glow() {}

void Glow::Render() {
    static uintptr_t ClientModule = (uintptr_t)GetModuleHandle(L"client.dll");
    
    LocalPlayer* Localplayer = LocalPlayer::GetLocalPlayer();

    if (Localplayer->isPlayerConnected()) {
        for (short i = 0; i < Player::GetMaxPlayer(); i++) {
            Player* player = Player::GetPlayer(i);

            if (!player) {
                continue;
            }

            if ((uintptr_t)player == (uintptr_t)Localplayer) {
                continue;
            }

            if (*player->GetHealth() <= 0) {
                continue;
            }

            Glow::GetGlowHandle()->GlowCustomize(player);
        }
    }
}

void Glow::GlowCustomize(Player* player) {
    LocalPlayer* Localplayer = LocalPlayer::GetLocalPlayer();

    if (*player->GetTeam() == *Localplayer->GetTeam()) {

        if ((*player->GetHealth() <= 100) && (*player->GetHealth() > 70)) {
            *player->Glow.GetColor() = AllyColor;
        }
        else if ( (*player->GetHealth() <= 70) && (*player->GetHealth() > 40)) {
            *player->Glow.GetColor() = { 0xEB, 0xD6, 0x1B, 0xFF };
        }
        else if ((*player->GetHealth() <= 40) && (*player->GetHealth() > 10)) {
            *player->Glow.GetColor() = { 0xE8, 0x83, 0x17, 0xFF };
        }
        else {
            *player->Glow.GetColor() = { 0xEB, 0x26, 0x26, 0xFF };
        }

        *player->Glow.GetIsGlowing() = 1;
    }
    else {
        if ((*player->GetHealth() <= 100) && (*player->GetHealth() > 70)) {
            *player->Glow.GetColor() = EnemyColor;
        }
        else if ((*player->GetHealth() <= 70) && (*player->GetHealth() > 40)) {
            *player->Glow.GetColor() = { 0xEB, 0xD6, 0x1B, 0xFF };
        }
        else if ((*player->GetHealth() <= 40) && (*player->GetHealth() > 10)) {
            *player->Glow.GetColor() = { 0xE8, 0x83, 0x17, 0xFF };
        }
        else {
            *player->Glow.GetColor() = { 0xEB, 0x26, 0x26, 0xFF };
        }

        *player->Glow.GetIsGlowing() = 1;
    }
}

Glow* Glow::GetGlowHandle() {
    static Glow glow;

    return &glow;
}