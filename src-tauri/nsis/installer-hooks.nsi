; NSIS Installer Hooks for OpenCap
; This adds a "OpenCap Settings" shortcut to the Start Menu

!macro NSIS_HOOK_POSTINSTALL
  ; Create Start Menu shortcut for Settings (same location as main shortcut)
  CreateShortcut "$SMPROGRAMS\${PRODUCTNAME} Settings.lnk" "$INSTDIR\${MAINBINARYNAME}.exe" "--settings" "$INSTDIR\${MAINBINARYNAME}.exe" 0
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  ; Remove the Settings shortcut
  Delete "$SMPROGRAMS\${PRODUCTNAME} Settings.lnk"
!macroend
