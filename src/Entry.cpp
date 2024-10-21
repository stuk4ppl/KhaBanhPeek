// dllmain.cpp : Defines the entry point for the DLL application.
#include <Windows.h>
#include <TlHelp32.h>
#include <iostream>
#include <vector>

#include "../sdk/memory/memory.h"
#include "../sdk/tool/Tool.h"
#include "../include/shared/UseOffset.h"
#include "../include/shared/include.h"

#define OnXButton2 -32768
#define OnLButton -32768

DWORD WINAPI HackThread(HMODULE hModule) {
    AllocConsole();
    FILE* f;
    freopen_s(&f, "CONOUT$", "w", stdout);

    //===================== START WORK ===================//
    std::vector<bool>CheatFlags;
    CheatFlags.push_back(0);
    CheatFlags.push_back(0);
    CheatFlags.push_back(0);
    CheatFlags.push_back(0);
    CheatFlags.push_back(0);
    CheatFlags.push_back(0);

    //isBunnyHop = 1
    //isRadarHack = 2
    //isAntiFlash = 3
    //isTriggerBot = 4
    //isGlow = 5
    //isAimbot = 6

    while (!(GetAsyncKeyState(VK_END) & 1)) {
        if ( (GetAsyncKeyState(VK_SHIFT) & 0x8000) && (GetAsyncKeyState(0x31) & 0x8000) ) {
            CheatFlags[0] = !CheatFlags[0];
            if (CheatFlags[0]) {
                std::cout << "BunnyHop: Enable" << std::endl;
            }
            else {
                std::cout << "BunnyHop: Disable" << std::endl;
            }
            Sleep(100);
        }

        if ( (GetAsyncKeyState(VK_SHIFT) & 0x8000) && (GetAsyncKeyState(0x32) & 0x8000) ) {
            CheatFlags[1] = !CheatFlags[1];
            if (CheatFlags[1]) {
                std::cout << "RadarHack: Enable" << std::endl;
            }
            else {
                std::cout << "RadarHack: Disable" << std::endl;
            }
            Sleep(100);
        }

        if ((GetAsyncKeyState(VK_SHIFT) & 0x8000) && (GetAsyncKeyState(0x33) & 0x8000)) {
            CheatFlags[2] = !CheatFlags[2];
            if (CheatFlags[2]) {
                std::cout << "AntiFlash: Enable" << std::endl;
            }
            else {
                std::cout << "AntiFlash: Disable" << std::endl;
            }
            Sleep(100);
        }

        if ((GetAsyncKeyState(VK_SHIFT) & 0x8000) && (GetAsyncKeyState(0x34) & 0x8000)) {
            CheatFlags[3] = !CheatFlags[3];
            if (CheatFlags[3]) {
                std::cout << "TriggerBot: Enable" << std::endl;
            }
            else {
                std::cout << "TriggerBot: Disable" << std::endl;
            }
            Sleep(100);
        }

        if ((GetAsyncKeyState(VK_SHIFT) & 0x8000) && (GetAsyncKeyState(0x35) & 0x8000)) {
            CheatFlags[4] = !CheatFlags[4];
            if (CheatFlags[4]) {
                std::cout << "Glow: Enable" << std::endl;
            }
            else {
                std::cout << "Glow: Disable" << std::endl;
            }
            Sleep(100);
        }
        
        if ((GetAsyncKeyState(VK_SHIFT) & 0x8000) && (GetAsyncKeyState(0x36) & 0x8000)) {
            CheatFlags[5] = !CheatFlags[5];
            if (CheatFlags[5]) {
                std::cout << "Aimbot: Enable" << std::endl;
            }
            else {
                std::cout << "Aimbot: Disable" << std::endl;
            }
            Sleep(100);
        }

        if (CheatFlags[0]) {
            Misc::BunnyHop();
        }
        
        if (CheatFlags[1]) {
            Misc::RadarHack();
        }

        if (CheatFlags[2]) {
            Misc::AntiFlash();
        }

        if (CheatFlags[3]) {
            if (GetAsyncKeyState(VK_XBUTTON2) == OnXButton2) {
                static TriggerBot Trigger;
                Trigger.Trigger();
            }
        }

        if (CheatFlags[4]) {
            Player::GlowPlayer();
        }

        if (CheatFlags[5]) {
            if (GetAsyncKeyState(VK_XBUTTON2) == OnXButton2) {
                static Aimbot Aim;
                Aim.Run();
            }
        }

        Sleep(1);
    }

    if (f) fclose(f);
    FreeConsole();
    FreeLibraryAndExitThread(hModule, 0);
    return 0;
}

BOOL APIENTRY DllMain( HMODULE hModule,
                       DWORD  ul_reason_for_call,
                       LPVOID lpReserved
                     )
{
    switch (ul_reason_for_call)
    {
        case DLL_PROCESS_ATTACH:
        {
            HANDLE hThread = CreateThread(nullptr, 0, (LPTHREAD_START_ROUTINE)HackThread, hModule, 0, nullptr);
            if (hThread) {
                CloseHandle(hThread);
            }
            break;
        }    
    }
    return TRUE;
}

