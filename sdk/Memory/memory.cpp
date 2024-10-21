#include "memory.h"

void mem::PatchEx(BYTE* dst, BYTE* src, unsigned int size, HANDLE ProcessHandle) {
	DWORD OldProtect;
	VirtualProtectEx(ProcessHandle, dst, size, PAGE_EXECUTE_READWRITE, &OldProtect);
	WriteProcessMemory(ProcessHandle, dst, src, size, nullptr);
	VirtualProtectEx(ProcessHandle, dst, size, OldProtect, &OldProtect);
}

void mem::NopEx(BYTE* dst, unsigned int size, HANDLE ProcessHandle) {
	BYTE* NopArray = new BYTE [size];
	memset(NopArray, 0x90, size);

	PatchEx(dst, NopArray, size, ProcessHandle);
	delete[] NopArray;
}

void mem::PatchIn(BYTE* dst, BYTE* src, unsigned int size) {
	DWORD OldProtect;
	VirtualProtect(dst, size, PAGE_EXECUTE_READWRITE, &OldProtect);
	memcpy(dst, src, size);
	VirtualProtect(dst, size, OldProtect, &OldProtect);
}

void mem::NopIn(BYTE* dst, unsigned int size) {
	DWORD OldProtect;
	VirtualProtect(dst, size, PAGE_EXECUTE_READWRITE, &OldProtect);
	memset(dst, 0x90, size);
	VirtualProtect(dst, size, OldProtect, &OldProtect);
}

bool mem::Hook(void* src, void* dst, int length) {
	if (length < 5) {
		return false;
	}

	DWORD CurrentProtection;
	VirtualProtect(src, length, PAGE_EXECUTE_READWRITE, &CurrentProtection);

	memset(src, 0x90, length);
	
	DWORD RelativeAddress = ((DWORD)dst - (DWORD)src) - 5; //Thu ko tru 5 xem say ra gi
	
	*(BYTE*)src = 0xE9;
	*(DWORD*)((DWORD)src + 1) = RelativeAddress;

	DWORD temp;
	VirtualProtect(src, length, CurrentProtection, &temp);

	return true;
}

