#include <Windows.h>
#include <vector>

#include "../../include/shared/UseOffset.h"
#include "../../sdk/memory/memory.h"
#include "../../include/schemas/Player.h"

#include <iostream> // For debug

ColorBYTE* CGlowProperty::GetColor() {
    return (ColorBYTE*)((uintptr_t)this + Offsets::m_Glow + Offsets::m_ColorOveride);
}

uintptr_t* CGlowProperty::GetIsGlowing() {
    return (uintptr_t*)((uintptr_t)this + Offsets::m_Glow + Offsets::bGlowing);
}

uintptr_t* EntitySpottedState::GetIsSpotted() {
    uintptr_t Padding = 0x1;
    return (uintptr_t*)(((uintptr_t)this - Padding) + Offsets::EntitySpottedState + Offsets::isSpotted);
}

uintptr_t* EntitySpottedState::GetIsSpottedByMask() {
    uintptr_t Padding = 0x1;
    return (uintptr_t*)(((uintptr_t)this - Padding) + Offsets::EntitySpottedState + Offsets::isSpottedByMask);
}

ColorBYTE* BaseModelEntity::GetClrRender() {
    return (ColorBYTE*)((uintptr_t)this + Offsets::ClrRender);
}

int Player::GetMaxPlayer() {
    static uintptr_t EngineModule = (uintptr_t)GetModuleHandle(L"engine2.dll");
    uintptr_t NetworkGameClient = mem::ReadMem<uintptr_t>(EngineModule + Offsets::NetworkGameClient);

    uintptr_t MaxClient = mem::ReadMem<uintptr_t>(NetworkGameClient + Offsets::MaxClient);

    return MaxClient;
    
}

Player* Player::GetPlayer(int index) {
	static uintptr_t ClientModule = (uintptr_t)(GetModuleHandle(L"client.dll"));
	static uintptr_t EntityList = mem::ReadMem<uintptr_t>(ClientModule + Offsets::EntityList);
    uintptr_t ListEntry = mem::ReadMem<uintptr_t>(EntityList + 0x10);


    for (short i = 0; i < GetMaxPlayer(); i++) {
        if (ListEntry == 0) {
            break;
        }

        uintptr_t* Flag = (uintptr_t*)(ListEntry + (2 * 0x78)); //Flag o day de check xem Controller thu 2 co NULL hay ko, vi khi vao tran Controller nay moi duoc khoi tao

        if (*Flag == NULL) {
            break;
        }

        uintptr_t* CurrentControllerAddress = (uintptr_t*)(ListEntry + (i + 1) * 0x78);

        if (*CurrentControllerAddress == NULL) {
            break;
        }

        uintptr_t CurrentController = mem::ReadMem<uintptr_t>((uintptr_t)CurrentControllerAddress); //Controller duoc coi la nguoi dang dieu khien nhan vat trong game, moi lien quan nhu sau: Controller -> &CSPlayerPawn, PlayerPawn duoc coi
                                                                                                    //la nhan vat trong game ( Luu y : Controller cung duoc coi la 1 thuc the )

        uintptr_t PawnHandle = mem::ReadMem<uintptr_t>(CurrentController + Offsets::hPlayerPawn); //PawnHandle la 2 BYTE nhung phai lay ca 8 BYTE de AND voi Bitmask, 1 BYTE cao dung de lay ListEntry, 1 BYTE thap dung de lam ID Entity trong
                                                                                                  //Entitylist

        if (PawnHandle == 0xFFFFFFFFFFFFFFFF) { //0xFFFFFF la mac dinh cua PawnHandle khi giu ID cua Pawn nao, li do la vi khi out tran, Pawn se bi delete truoc sau do moi den Controller nen no se set thanh gia tri mac dinh
            continue;
        }
        
        uintptr_t ListEntryMain = mem::ReadMem<uintptr_t>(EntityList + (0x8 * ((PawnHandle & BitMaskForListEntry) >> 9) + 0x10)); //Bitmask 7FFF dung de loc ra 2 byte trong chuoi 8 byte, sau do duoc dich chuyen 9 bit, 
                                                                                                                                  //muc dich la lay index cho ListEntry, vi co nhieu ListEntry, moi List dai dien cho 1 mang
                                                                                                                                  //cac Entity khac nhau

        uintptr_t CurrentPawn = mem::ReadMem<uintptr_t>(ListEntryMain + 0x78 * (PawnHandle & BitMaskForCurrentPawn)); //1FF dung de loc ra 1 byte (8 bit) cuoi cung trong PawnHandle, ket qua se duoc cho la ID cua Entity trong EntityList
                                                                                                                      //moi phan tu trong EntityList deu la struct co kich thuoc 0x78 chua cac thuoc tinh cua Entity, de tim duoc Entity mong muon
                                                                                                                      //ta can ID nay`, va ID * 0x78 se ra duoc vi tri cua Entity trong EntityList

        if ((index == i) && (CurrentPawn)) {
            return (Player*)CurrentPawn;
        }
    }

    return nullptr;
}

int* Player::GetHealth() {
    return (int*)((uintptr_t)this + Offsets::Health);
}

int* Player::GetTeam() {
    return (int*)((uintptr_t)this + Offsets::TeamNum);
}

Vec3* Player::GetVecOrigin() {
    return (Vec3*)(mem::ReadMem<uintptr_t>((uintptr_t)this + Offsets::pGameSceneNode) + Offsets::VecOrigin);
}

Vec3* Player::GetViewOffset() {
    return (Vec3*)((uintptr_t)this + Offsets::ViewOffset); 
}

Vec3 Player::GetBonePos(int BoneID) {
    uintptr_t BoneMatrix = mem::ReadMem<uintptr_t>(mem::ReadMem<uintptr_t>((uintptr_t)this + Offsets::pGameSceneNode) + Offsets::BoneMatrix);
    Vec3* CopyBone = (Vec3*)(BoneMatrix + (BoneID * 32));
    Vec3 MainBone;
    MainBone.x = CopyBone->x;
    MainBone.y = CopyBone->y;
    MainBone.z = CopyBone->z;
    
    return MainBone;
}

Player* Player::GetPlayerByCrossHairID(uintptr_t ID) {
    static uintptr_t ClientModule = (uintptr_t)(GetModuleHandle(L"client.dll"));
    static uintptr_t EntityList = mem::ReadMem<uintptr_t>(ClientModule + Offsets::EntityList);

    if (EntityList && (ID > MinEntIndex && ID < MaxEntIndex)) {
        uintptr_t ListEntry = mem::ReadMem<uintptr_t>(EntityList + (0x8 * ((ID & BitMaskForListEntry) >> 9) + 0x10));

        if (ListEntry) {
            Player* EntityPawn = (Player*)mem::ReadMem<uintptr_t>(ListEntry + 0x78 * (ID & BitMaskForCurrentPawn));
            return EntityPawn;
        }
        else return 0;
    }
    else return 0;
}

void Player::GlowPlayer() {
    Glow::Render();
}