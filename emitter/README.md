generation requires two passes. the first will download the [specification files](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-resource-specification-format.html) as well as downloads the public documentation. It saves and extracts necessary data into temp.json and then exits.

the second pass will read from temp.json and actually output the code. note we don't do anything special to resolve temp.json. it assumes its in the current working directory, so ensure you are in /emitter before running the command.

full generation instructions:

```sh
cd emitter
cargo build --release
rm -rf ../output
# this one is human-made, and we want it to exist:
git checkout ../output/cfn_resources
./../target/release/emitter
# the first pass creates temp.json
# then we simply run it again
./../target/release/emitter
```
