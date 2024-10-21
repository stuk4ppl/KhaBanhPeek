#include <Windows.h>
#include <cmath>

#include "../../include/schemas/LocalPlayer.h"
#include "../../include/shared/UseOffset.h"
#include "../../sdk/memory/memory.h"

LocalPlayer* LocalPlayer::GetLocalPlayer() {
	static uintptr_t ClientModule = (uintptr_t)(GetModuleHandle(L"client.dll"));
	LocalPlayer* Localplayer = (LocalPlayer*)mem::ReadMem<uintptr_t>(ClientModule + Offsets::LocalPlayer);

	return Localplayer;
}

Vec3* LocalPlayer::GetVecOrigin() {
	return (Vec3*)(mem::ReadMem<uintptr_t>((uintptr_t)this + Offsets::pGameSceneNode) + Offsets::VecOrigin);
}

Vec3* LocalPlayer::GetViewOffsets() {
	return (Vec3*)((uintptr_t)this + Offsets::ViewOffset);
}

int* LocalPlayer::GetHealth() {
	return (int*)((uintptr_t)this + Offsets::Health);
}

int* LocalPlayer::GetTeam() {
	return (int*)((uintptr_t)this + Offsets::TeamNum);
}

Vec2 LocalPlayer::CalcAngle(Vec3* EnemyBone) { // Enemy o day co the la head hoac chest, tuy vao Vec3 toa do cua bone truyen vao, ban chat cua ham la 3D to 2D Screen
	double PI = 3.14159265358; // Khai bao PI
	static uintptr_t ClientModule = (uintptr_t)(GetModuleHandle(L"client.dll"));
	static Vec3* ViewAngles = (Vec3*)(ClientModule + Offsets::ViewAngle); // Lay goc nhin hien tai cua nguoi choi

	Vec3 Origin = *GetVecOrigin(); // Lay toa do goc cua nguoi choi
	Vec3 ViewOffset = *GetViewOffsets(); // Viewoffset = tuc camera pos - origin pos ( vi camera pos nam tren head nen luc nao cung se lon hon origin pos, the nen tru luc nao cung se duoc so duong )
	Vec3 CameraPos = Origin + ViewOffset; //Camera Pos, origin ( toa do o chan ), viewofset ( toa do cua camera - origin ) ( x = 0, y = 0, z = 63, 63 o day la truc z )
	Vec3* CameraPosition = &CameraPos;

	Vec3 Delta = { EnemyBone->x - CameraPosition->x, EnemyBone->y - CameraPosition->y, EnemyBone->z - CameraPosition->z }; // Delta la do. bu` cua 2 Vec3 trong toa do 3D
	float EuclidDistance = sqrt(pow(Delta.x, 2) + pow(Delta.y, 2) + pow(Delta.z, 2)); //Cong thuc Euclid, tinh khoang cach 2 toa do trong khong gian 3 chieu

	float pitch = -asin(Delta.z / EuclidDistance) * (180 / PI); // Chieu doc camera
	float yaw = atan2(Delta.y, Delta.x) * (180 / PI); //Chieu ngang cua camera

	if ((pitch > -89 || pitch < 89) && (yaw > -180 || yaw < 180)) {
		Vec2 PositionOnScreen;
		PositionOnScreen.x = pitch;
		PositionOnScreen.y = yaw;
		return PositionOnScreen;
	}
}

Vec2* LocalPlayer::GetViewAngles() {
	static uintptr_t ClientModule = (uintptr_t)(GetModuleHandle(L"client.dll"));
	Vec2* ViewAngles = (Vec2*)(ClientModule + Offsets::ViewAngle); // Lay goc nhin hien tai cua nguoi choi

	return ViewAngles;
}

float LocalPlayer::GetDistance(Vec3* Enemy) {
	Vec3* LocalPosition = GetVecOrigin();
	Vec3 Delta = Vec3(Enemy->x - LocalPosition->x, Enemy->y - LocalPosition->y, Enemy->z - LocalPosition->z);

	return sqrt(pow(Delta.x, 2) + pow(Delta.y, 2) + pow(Delta.z, 2));
}

uintptr_t* LocalPlayer::GetCrossHairID() {
	return (uintptr_t*)((uintptr_t)this + Offsets::IDEntIndex);
}

uintptr_t* LocalPlayer::GetHandleToCurrentWeapon() {
	return (uintptr_t*)((uintptr_t)this + Offsets::ClippingWeapon);
}

const char* LocalPlayer::GetWeaponNameOnHand() { // SUA ham nay sao cho thanh phuong thuc thanh vien cua LocalPlayer
	uintptr_t* HandleWeapon = GetHandleToCurrentWeapon();
	if (HandleWeapon) {
		uintptr_t WeaponData = mem::ReadMem<uintptr_t>(*HandleWeapon + 0x10);

		if (WeaponData + 0x20) {
			uintptr_t WeaponName = mem::ReadMem<uintptr_t>(WeaponData + 0x20);
			const char* Name = (char*)WeaponName;
			return Name;
		}
	}
	return "Unknow";
}

bool LocalPlayer::isScoped() {
	return mem::ReadMem<bool>((uintptr_t)this + Offsets::isScoped);
}

bool LocalPlayer::isPlayerConnected() {
	static uintptr_t EngineModule = (uintptr_t)GetModuleHandle(L"engine2.dll");
	uintptr_t NetworkGameClient = mem::ReadMem<uintptr_t>(EngineModule + Offsets::NetworkGameClient);

	uintptr_t ClientState = mem::ReadMem<uintptr_t>(NetworkGameClient + Offsets::SignOnState);
	if (ClientState == 6) {
		return true;
	}
	else {
		return false;
	}
}