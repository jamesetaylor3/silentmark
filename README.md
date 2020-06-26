# silentmark

silentmark is a command line tool that hides information in plain sight

currently the only (and hardly) supported feature is hiding text in an image

### Build

You will have to install cargo to build this. If you do this, just navigate to project and build this command. Can conveniently find the compiled target in the target directory

`cargo build --release`

### Usage

To add a silentmartk to an image, you can use the -w tag. This will repetitively hide the contents in smark.txt in the image in.jpeg. This output will be silentmarked-out.png. Use the --image (-i) and --message (-m) arguments to indicate the path to the image to be silentmark and the path to the text file with the message.

`silentmark write --image in.jpeg --message smark.txt`

To retrieve the hidden silentmark, you can use the read subcommand. It will essentially to the reverse of the last command and take the silentmark in the image of the first argument and paste it in out.txt. Use the image argument (-i) and message argument (-m) to indicate the input image path and output text path.

`silentmark read --image silentmarked-out.png --message out.txt`

Run `silenmark help` to learn more!

### Couple things to note

* While you can import and image type you want, the silentmarked export will be in png. other types are not accepted right now

### Up next
* Other modes of hiding (e.g. text or image in .mp3)