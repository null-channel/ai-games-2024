# Part One

June third, at 8:30pm UTC 2025 was hailed the AI epoc, and usurered in the end of humanity.
By 2026 only a hundredth of the worlds human population remain after devistating wars between the
Unified human collective (UHC) and the 8 super AGIs.

Current year: 2029
Month: December
Day: 3rd

You are part of the human resistance still fighting for the future of human kind. Last known there where
fewer then 10 million people left a year ago as "Tireless Enemy Removal Machine Identifying and Neutralizing Agents,
Targeting Instantly, Obliterating Ruthlessly" scowered the glob searching for signs of humanity. 
Your job is to scavenging what you can from the old world ruins that where
created in the great wars of 2026. Today in the search though the rubble of a fallen city you found a analog radio.
You notice that there is a pattern of sounds coming from the machine. Thankfully for you, a book on mores code sits
beside the radio. Unfortunately for you the rate at which the tones are being played exceed your ability to transcribe them.

After a few hours of work you have gotten the radio's speaker wired into your command interface,
your feild of view was filled with zeros and ones. After a little bit of trouble you found out it's following
a pattern you found on the old world internet archive downloaded in your command unit: 
https://en.wikipedia.org/wiki/Morse_code

111 -> dash
1 -> dot
0 -> separator units
000 -> separator letters
0000000 -> separator words
1111111 -> period
111111111 -> comma

so the string 'two birds' would look like this in morse:
`- .-- ---     -... .. .-. -.. ...`
and looks like this in your command interface:
`1110001011101110001110111011100000001110101010001010001011101000111010100010101`

You found out that it appears to be repeating every hour as a long sequence of zeros (empty space) fill the screen each hour and then the pattern repeetes. At this point you have piped an entire segment into a file `log_0.txt`  you need to write a script that takes this file as input and translate this into the corresponding English encoding.

