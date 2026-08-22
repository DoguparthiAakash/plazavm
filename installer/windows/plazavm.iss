[Setup]
AppName=PlazaVM
AppVersion={#AppVersion}
AppPublisher=PlazaVM Contributors
AppPublisherURL=https://github.com/plazavm/plazavm
DefaultDirName={autopf}\PlazaVM
PrivilegesRequired=lowest
OutputBaseFilename=PlazaVM_Setup
Compression=lzma
SolidCompression=yes
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
ChangesEnvironment=yes
DisableWelcomePage=no
DisableDirPage=no

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\..\build\installer\windows\staging\plaza.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\build\installer\windows\staging\plaza-desktop.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\PlazaVM\PlazaVM Desktop"; Filename: "{app}\plaza-desktop.exe"
Name: "{autodesktop}\PlazaVM"; Filename: "{app}\plaza-desktop.exe"; Tasks: desktopicon

[Code]
const
  EnvironmentKey = 'Environment';

function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', OrigPath) then
  begin
    Result := True;
    exit;
  end;
  { Use case-insensitive search for path }
  Result := Pos(';' + Lowercase(Param) + ';', ';' + Lowercase(OrigPath) + ';') = 0;
end;

procedure RemovePath(Path: string);
var
  Paths: string;
  P: Integer;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths) then
    exit;
  
  Paths := ';' + Paths + ';';
  P := Pos(';' + Lowercase(Path) + ';', Lowercase(Paths));
  if P = 0 then
    exit;
  
  Delete(Paths, P, Length(Path) + 1);
  if Length(Paths) > 0 then
  begin
    if Paths[1] = ';' then Delete(Paths, 1, 1);
    if (Length(Paths) > 0) and (Paths[Length(Paths)] = ';') then Delete(Paths, Length(Paths), 1);
  end;
  
  if Paths = '' then
    RegDeleteValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path')
  else
    RegWriteStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', Paths);
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  OrigPath: string;
  NewPath: string;
begin
  if CurStep = ssPostInstall then
  begin
    if NeedsAddPath(ExpandConstant('{app}')) then
    begin
      if not RegQueryStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', OrigPath) then
        OrigPath := '';
      
      NewPath := OrigPath;
      if (NewPath <> '') and (NewPath[Length(NewPath)] <> ';') then
        NewPath := NewPath + ';';
      NewPath := NewPath + ExpandConstant('{app}');
      
      RegWriteStringValue(HKEY_CURRENT_USER, EnvironmentKey, 'Path', NewPath);
    end;
  end;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
  begin
    RemovePath(ExpandConstant('{app}'));
  end;
end;
