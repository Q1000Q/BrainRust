s"ab"   // "ab" string
<2      // Moves 2 backward (to start of the string)
.>.     // Prints "ab"
<       // Moves to start
;       // Swaps 'a' with 'b'
.>.     // Prints the same cells as above, but now they are "ba"

\.      // Sets current cell to LFeed and prints
A       // Prints current address

\.      // Sets current cell to LFeed and prints

b'a'    // Sets current cell to ASCII value of 'a'
p       // Prints current value as decimal number (should be ASCII value of 'a' - 97)
\.      // Sets current cell to LFeed and prints
^       // Zeroes the cell
p       // Prints current value as decimal number (should be 0)