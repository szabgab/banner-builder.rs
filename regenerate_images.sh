# regenerate all the images in the site/examples/ folder

set -e
for yaml in site/examples/*.yaml;
do
    # echo $yaml
    image=$(echo $yaml | sed 's/\.yaml/\.png/')
    # echo $image
    # echo "---"
    cargo run $yaml $image
done