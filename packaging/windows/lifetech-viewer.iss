; LifeTech Viewer's Windows installer (Inno Setup 6, https://jrsoftware.org).
; Installs for the current user, without administrator rights, into
; %LOCALAPPDATA%\Programs\LifeTech Viewer: the folder stays writable, so the
; viewer can update itself there. Start menu entry, optional desktop
; shortcut, and an uninstaller in Settings › Apps. The viewer's settings,
; chats and cache are in the user's profile and stay after an uninstall.
;
; Built from the packaged folder (scripts/package.ps1), see docs/BUILDING.md:
;   ISCC.exe /DAppVersion=1.1.0 /DSourceDir=<packaged folder>
;            /DOutputName=LifeTech-Viewer-1.1.0-windows-x64-setup
;            /O<output folder> packaging\windows\lifetech-viewer.iss

#ifndef AppVersion
  #define AppVersion "0.0.0"
#endif
#ifndef SourceDir
  #error Pass /DSourceDir=<the packaged folder>
#endif
#ifndef OutputName
  #define OutputName "LifeTech-Viewer-" + AppVersion + "-windows-x64-setup"
#endif
; /DPersonal: the owner's personal build, installed apart from the public one
#ifdef Personal
  #define AppName "LifeTech Viewer Personal"
  #define AppGuid "{{FF6CD3EA-936A-48E4-B056-2A1055A7E959}"
#else
  #define AppName "LifeTech Viewer"
  #define AppGuid "{{3FF7871D-8EBA-4486-B1F4-3BEF964C32B2}"
#endif

[Setup]
AppId={#AppGuid}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=LifeTech Systems
AppPublisherURL=https://viewer.lifetech.systems
AppSupportURL=https://github.com/LethalCyborg/LifeTech-Viewer/issues
AppUpdatesURL=https://viewer.lifetech.systems/#download
DefaultDirName={localappdata}\Programs\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
OutputBaseFilename={#OutputName}
SetupIconFile=..\..\assets\branding\lifetech.ico
UninstallDisplayIcon={app}\lifetech-viewer.exe
UninstallDisplayName={#AppName}
LicenseFile=..\..\LICENSE
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#SourceDir}\*"; DestDir: "{app}"; Flags: recursesubdirs createallsubdirs ignoreversion

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\lifetech-viewer.exe"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\lifetech-viewer.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\lifetech-viewer.exe"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent
