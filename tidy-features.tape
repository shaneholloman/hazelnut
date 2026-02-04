# Tidy TUI Feature Demo
# Run: vhs tidy-features.tape

Output demo-features.gif

Set FontSize 14
Set Width 1200
Set Height 700
Set Padding 20

Type "./target/release/tidy"
Enter
Sleep 2s

# Dashboard view - show stats
Sleep 1.5s

# Navigate to Rules view
Type "2"
Sleep 1s

# Navigate through rules
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms

# Toggle a rule off
Enter
Sleep 800ms

# Toggle it back on
Enter
Sleep 800ms

# Go to first rule
Type "g"
Sleep 500ms

# Go to last rule
Type "G"
Sleep 500ms

# Navigate to Watches view
Type "3"
Sleep 1s

# Navigate watches
Type "j"
Sleep 400ms
Type "k"
Sleep 400ms

# Navigate to Log view
Type "4"
Sleep 1s

# Show help popup
Type "?"
Sleep 2s

# Close help
Type "?"
Sleep 500ms

# Back to Dashboard
Type "1"
Sleep 1s

# Change theme for fun
Ctrl+t
Sleep 600ms
Ctrl+t
Sleep 600ms
Ctrl+t
Sleep 600ms

# Quick view switching with Tab
Tab
Sleep 500ms
Tab
Sleep 500ms
Tab
Sleep 500ms
Tab
Sleep 500ms

# Quit
Type "q"
Sleep 300ms
