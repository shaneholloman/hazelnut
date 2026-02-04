# Tidy TUI Demo - All 15 Themes
# Run: vhs tidy-themes.tape

Output demo-themes.gif

Set FontSize 14
Set Width 1200
Set Height 700
Set Padding 20

Type "./target/release/tidy"
Enter
Sleep 2s

# Dashboard - Dracula (default)
Sleep 1s

# Cycle through all 15 themes
# 1. Dracula -> 2. One Dark Pro
Ctrl+t
Sleep 800ms

# 2. One Dark Pro -> 3. Nord
Ctrl+t
Sleep 800ms

# 3. Nord -> 4. Catppuccin Mocha
Ctrl+t
Sleep 800ms

# 4. Catppuccin Mocha -> 5. Catppuccin Latte
Ctrl+t
Sleep 800ms

# 5. Catppuccin Latte -> 6. Gruvbox Dark
Ctrl+t
Sleep 800ms

# 6. Gruvbox Dark -> 7. Gruvbox Light
Ctrl+t
Sleep 800ms

# 7. Gruvbox Light -> 8. Tokyo Night
Ctrl+t
Sleep 800ms

# 8. Tokyo Night -> 9. Solarized Dark
Ctrl+t
Sleep 800ms

# 9. Solarized Dark -> 10. Solarized Light
Ctrl+t
Sleep 800ms

# 10. Solarized Light -> 11. Monokai Pro
Ctrl+t
Sleep 800ms

# 11. Monokai Pro -> 12. Rosé Pine
Ctrl+t
Sleep 800ms

# 12. Rosé Pine -> 13. Kanagawa
Ctrl+t
Sleep 800ms

# 13. Kanagawa -> 14. Everforest
Ctrl+t
Sleep 800ms

# 14. Everforest -> 15. Cyberpunk
Ctrl+t
Sleep 1s

# Back to Dracula
Ctrl+t
Sleep 500ms

# Show some navigation
Type "2"
Sleep 1s
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "3"
Sleep 1s
Type "1"
Sleep 1s

# Quit
Type "q"
Sleep 300ms
