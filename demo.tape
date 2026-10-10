# VHS tape: scripted gameplay demo for the README.
# Regenerate with: vhs demo.tape  (needs `brew install vhs ffmpeg`)
Output assets/demo.gif

Set Shell zsh
Set FontSize 14
Set Width 1000
Set Height 600
Set Framerate 30
Set Padding 10

Hide
Type "./target/debug/space-invaders"
Enter
Show
Sleep 1s

Space
Sleep 700ms
Left 6
Space
Sleep 700ms
Right 10
Space
Sleep 700ms
Left 4
Space
Sleep 3s

Type "q"
Sleep 500ms
