# Part Two

Part 2:
It turns out that most of the message is unreadable!!!!! You start to think that this might be a dead end with
corrupted information and sit down to look though the data you have parsed. Interestingly your command unit
starts to pull out patterns and you think that instead of the data being corrupted it might just be encrypted.
Your command unit's quantum prossessor starts to hum and your arm feels the warmth radiating off of it.

Hours later and after the command unit reach uncomfortable temperatures it informs you that the cryptography
is not based off any modern quantum or even pre-quantum technology and instead seems to be relying on a
polyalphabetic cipher dating to the pre-silicon era. Sadly while easy for humans, polyalphabetic ciphers
are difficult for Quantum AI's to decode based on the simplistic nature of polyalphabetic ciphers and
Quantum AI's outrageous disdain for simple.

It is up to you to write a function to decode the polyalphabetic cipher!

NOTE: The polyalphabetic cipher is a simple substitution cipher that uses a keyword to generate a key
in order to encode the message the key is repeated until the message is fully encoded. The key is also
 not use white space or punctuation and should be removed from the key before decoding the message.

example:
Key: "The Black Cat Rides Again"
Value: "When I was a young child, I never thought the world would end by our own hands."
Encoded: "Poio T wcc c yhlvj gzird, Q axcis ehqeihm kph agrrd ebnsh fyd di quk feq lsnjs."
