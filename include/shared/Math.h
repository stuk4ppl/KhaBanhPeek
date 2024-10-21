#pragma once

#define BitMaskForListEntry 0x7FFF
#define BitMaskForCurrentPawn 0x1FF
#define IN_JUMP (1 << 16)
#define MinEntIndex 0
#define MaxEntIndex 35557
#define ForceAction 65537
#define UnForceAction 256

struct Vec3 {
	float x, y, z;

	Vec3() {
		x = y = z = 0;
	}

	Vec3(float _x, float _y, float _z) : x(_x), y(_y), z(_z) {}

	Vec3 operator+(Vec3 other) {
		return { this->x + other.x, this->y + other.y, this->z + other.z };
	}
};

struct Vec2 {
	float x, y;

	Vec2() {
		x = y = 0;
	}

	Vec2(float _x, float _y) : x(_x), y(_y) {}

	Vec2 operator+(Vec2 other) {
		return { this->x + other.x, this->y + other.y};
	}
	bool operator==(int number) {
		return ((x == 0) && (y == 0));
	}
};
