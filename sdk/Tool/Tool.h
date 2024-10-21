#pragma once
#include <Windows.h>
#include <TlHelp32.h>
#include <vector>

DWORD GetProcessID(const wchar_t* ProcessName);

uintptr_t GetModuleBaseAdress(DWORD ProcessID, const wchar_t* ModuleName);

uintptr_t FindDMAAddy(HANDLE handle, uintptr_t ptr, std::vector<unsigned int> offsets);

uintptr_t FindDMAAddyIn(uintptr_t ptr, std::vector<unsigned int> offsets);
