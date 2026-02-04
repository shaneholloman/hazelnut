# Tidy TUI High-Res Screenshots for README
# Run: vhs tidy-readme.tape

Output demo.gif

Set FontSize 16
Set Width 1400
Set Height 900
Set Padding 30
Set TypingSpeed 50ms

Type "./target/release/tidy"
Enter
Sleep 2s

# Dashboard
Screenshot screenshots/01-dashboard.png
Sleep 1s

# Open theme picker
Type "t"
Sleep 1s
Screenshot screenshots/02-theme-picker.png

# Navigate themes
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Screenshot screenshots/03-theme-picker-selection.png

# Select Nord theme
Enter
Sleep 1s
Screenshot screenshots/04-dashboard-nord.png

# Go to Rules
Type "2"
Sleep 1s
Screenshot screenshots/05-rules.png

# Navigate rules
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Screenshot screenshots/06-rules-selected.png

# Toggle rule
Enter
Sleep 500ms
Screenshot screenshots/07-rule-toggled.png

# Go to Watches
Type "3"
Sleep 1s
Screenshot screenshots/08-watches.png

# Go to Log
Type "4"
Sleep 1s
Screenshot screenshots/09-log.png

# Show help
Type "?"
Sleep 1s
Screenshot screenshots/10-help.png
Type "?"
Sleep 500ms

# Open theme picker and switch to Cyberpunk for dramatic effect
Type "t"
Sleep 500ms
Type "G"
Sleep 500ms
Enter
Sleep 1s
Screenshot screenshots/11-cyberpunk.png

# Back to default theme (Dracula)
Type "t"
Sleep 300ms
Type "g"
Sleep 300ms
Enter
Sleep 500ms

# Back to dashboard
Type "1"
Sleep 1s

# Quit
Type "q"
Sleep 300ms
