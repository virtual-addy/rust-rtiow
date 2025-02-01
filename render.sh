build_path="./target/debug"
# build current source code
cargo build
# Check if old render file exists
if [ -f $build_path/im.ppm ]; then
#  remove render file if found
  rm $build_path/im.ppm
  echo "Previous render removed"
else
  echo "No previous render found"
fi
# Render and output .ppm image file
$build_path/rtiow >> $build_path/im.ppm
# Show rendered image
open $build_path/im.ppm
