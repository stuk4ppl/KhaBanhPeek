#pragma once
#include <Windows.h>

struct InfoOfOverwriteAddress {
public:
	InfoOfOverwriteAddress(DWORD BytesCount, uintptr_t RelativeHookAddress) : BytesOverwrite(BytesCount), RelativeAddress(RelativeHookAddress){
		ModuleBase = (uintptr_t)GetModuleHandle(0);
		AddressForHook = ModuleBase + RelativeAddress;
	}
	uintptr_t RelativeAddress;
	DWORD AddressForHook;
	DWORD BytesOverwrite;
	uintptr_t ModuleBase;

};

namespace mem {
	void PatchEx(BYTE* dst, BYTE* src, unsigned int size, HANDLE ProcessHandle); //Patch external

	void NopEx(BYTE* dst, unsigned int size, HANDLE ProcessHandle); //Nop external

	void PatchIn(BYTE* dst, BYTE* src, unsigned int size); //Patch internal

	void NopIn(BYTE* dst, unsigned int size); //Patch internal

	bool Hook(void* src, void* dst, int length);

	template<typename Param>
	Param ReadMem(uintptr_t Address) {
		return *(uintptr_t*)Address;
	}

	template<typename Param>
	void WriteMem(uintptr_t Address, Param value) {
		*(uintptr_t*)Address = value;
	}
}
