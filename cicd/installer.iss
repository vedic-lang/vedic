[Registry]
; Add the installation folder to the PATH environment variable
Root: HKA; Subkey: "{code:GetEnvironmentKey}"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"
; File ext setting
Root: HKA; Subkey: "Software\Classes\ved\OpenWithProgids"; ValueType: string; ValueName: "vedic"; ValueData: ""; Flags: uninsdeletevalue
;others
Root: HKA; Subkey: "Software\Classes\vedic"; ValueType: string; ValueName: ""; ValueData: "vedic"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\vedic\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\vedic.exe,0"
Root: HKA; Subkey: "Software\Classes\vedic\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\vedic.exe"" ""%1"""
Root: HKA; Subkey: "Software\Classes\Applications\vedic.exe\SupportedTypes"; ValueType: string; ValueName: ".myp"; ValueData: ""

