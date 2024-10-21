#include "tool.h"

DWORD GetProcessID(const wchar_t* ProcessName) {
	DWORD pID = 0;

	HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

	if (hSnapshot != INVALID_HANDLE_VALUE) {
		PROCESSENTRY32 ProcessEntry;

		ProcessEntry.dwSize = sizeof(ProcessEntry);
		if (Process32First(hSnapshot, &ProcessEntry)) {
			do {
				if (_wcsicmp(ProcessEntry.szExeFile, ProcessName) == 0) {
					pID = ProcessEntry.th32ProcessID;
					break;
				}
			} while (Process32Next(hSnapshot, &ProcessEntry));
		}
	}

	CloseHandle(hSnapshot);
	return pID;
}

uintptr_t GetModuleBaseAdress(DWORD ProcessID, const wchar_t* ModuleName) {
	uintptr_t ModuleAddress = 0;

	HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, ProcessID);

	if (hSnapshot != INVALID_HANDLE_VALUE) {
		MODULEENTRY32 ModuleEntry;
		ModuleEntry.dwSize = sizeof(ModuleEntry);

		if (Module32First(hSnapshot, &ModuleEntry)) {
			do {
				if (_wcsicmp(ModuleEntry.szModule, ModuleName) == 0) {
					ModuleAddress = (uintptr_t)ModuleEntry.modBaseAddr;
					break;
				}
			} while (Module32Next(hSnapshot, &ModuleEntry));
		}
	}

	CloseHandle(hSnapshot);
	return ModuleAddress;
}

uintptr_t FindDMAAddy(HANDLE handle, uintptr_t ptr, std::vector<unsigned int> offsets) {
	for (int i = 0; i < offsets.size(); i++) {
		ReadProcessMemory(handle, (VOID*)ptr, &ptr, sizeof(ptr), nullptr);
		ptr += offsets[i];
	}
	return ptr;
}

uintptr_t FindDMAAddyIn(uintptr_t ptr, std::vector<unsigned int> offsets) {
	uintptr_t addr = ptr;
	for (int i = 0; i < offsets.size(); i++) {
		addr = *(uintptr_t*)addr;
		addr += offsets[i];
	}
	return addr;
}