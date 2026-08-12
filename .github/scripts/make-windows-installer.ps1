# Packages a Windows build into a distributable installer .exe using
# Inno Setup, which ships preinstalled on GitHub's windows-latest runners.
param(
  [Parameter(Mandatory = $true)][string]$AppId,
  [Parameter(Mandatory = $true)][string]$SourceDir,
  [Parameter(Mandatory = $true)][string]$ExeName
)

$ErrorActionPreference = "Stop"

$iss = @"
[Setup]
AppName=$AppId
AppVersion=1.0
DefaultDirName={autopf}\$AppId
DefaultGroupName=$AppId
OutputDir=.
OutputBaseFilename=$AppId-setup
Compression=lzma
SolidCompression=yes
DisableProgramGroupPage=yes

[Files]
Source: "$SourceDir\*"; DestDir: "{app}"; Flags: recursesubdirs

[Icons]
Name: "{group}\$AppId"; Filename: "{app}\$ExeName"
Name: "{autodesktop}\$AppId"; Filename: "{app}\$ExeName"
"@

Set-Content -Path "$AppId.iss" -Value $iss -Encoding UTF8
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" "$AppId.iss"
