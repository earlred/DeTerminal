[Setup]
AppName=DeTerminal
AppVersion=0.1.1
DefaultDirName={pf}\DeTerminal
DefaultGroupName=DeTerminal
OutputDir=installer/windows/Output
OutputBaseFilename=DeTerminal-Setup
Compression=lzma
SolidCompression=yes

[Files]
Source: "target\\release\\determinal.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\\DeTerminal"; Filename: "{app}\\determinal.exe"
