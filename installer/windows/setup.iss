[Setup]
AppName=DeTerminal
AppVersion=0.1.1
DefaultDirName={pf}\DeTerminal
DefaultGroupName=DeTerminal
OutputDir=installer/windows/Output
OutputBaseFilename=DeTerminal-Setup
Compression=lzma
SolidCompression=yes

#define AppExe "..\\..\\target\\release\\determinal.exe"

[Files]
Source: "{#AppExe}"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\\DeTerminal"; Filename: "{app}\\determinal.exe"
