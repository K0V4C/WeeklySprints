# Huffman and Differential Coding

Here is simple implementation of static Huffman algorithm combined with differential coding done in Rust.

To properly run the program it is needed to first contrusct probbability table with option 1.
After that construct huffman table with option 3, and can choose a picture u want to transfer with option 2.

Transfer is done between 2 threads using a channel to send data in beatween.

Every pixel is broken down into R, G, B components and then differentiated in next iterations. Each of those numbers
is funneled into Huffman algorithm and sent through the channel.

After doing steps 1 and 3 you can rerun simulation any number of times using option 4. When you are done you can exit with
option 5.

#### Credits

Images used in this are work from artits:

pulsar2105
Pinki
jrmnt
