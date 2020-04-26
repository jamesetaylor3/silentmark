# silentmark

silentmark is a command line tool that hides information in plain sight

currently the only (and hardly) supported feature is hiding text in an image

### Build

You will have to install cargo to build this. If you do this, just navigate to project and build this command. Can conveniently find the compiled target in the target directory

`cargo build --release`

### Usage

To add a silentmartk to an image, you can use the -w tag. This will repetitively hide the contents in smark.txt in the image in.jpeg. This output will be silentmarked-out.png.

`silentmark in.jpeg -w smark.txt`

To retrieve the hidden silentmark, you can use the -r tag. It will essentially to the reverse of the last command and take the silentmark in the image of the first argument and paste it in out.txt

`silentmark silentmarked-out.png -r out.txt`

### Couple things to note

* While you can import and image type you want, the silentmarked export will be in png. other types are not accepted right now

### Up next
* Customizing output file names
* Customizing output file types
* Other modes of hiding (e.g. text or image in .mp3)